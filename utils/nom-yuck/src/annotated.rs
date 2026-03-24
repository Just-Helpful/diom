use diom_fmt::{Flush, Format, IndentWriter, Indented};
use nom::Offset;
use std::{fmt::Write, ops::Range};

#[derive(Clone)]
/// How to display annotations on the given input
pub struct Annotated<I> {
  /// The input to display annotations on
  pub input: I,
  /// The character used to represent the position of an annotation
  pub caret: char,
  /// The character used to represent spaces before `Self::caret`
  pub fill: char,
}

/// An alias constructor for displaying annotations on input
pub fn on<I>(input: I) -> Annotated<I> {
  input.into()
}
/// How to display annotations on a given input (alias of `Annotated`)
pub type On<I> = Annotated<I>;

impl<I> From<I> for Annotated<I> {
  fn from(value: I) -> Self {
    Self::basic(value)
  }
}
impl<I: Default> Default for Annotated<I> {
  fn default() -> Self {
    Self::from(I::default())
  }
}

impl<I> Annotated<I> {
  pub fn basic(input: I) -> Self {
    Self {
      input,
      caret: '^',
      fill: ' ',
    }
  }
}

impl<I: Clone> Format for Annotated<I> {
  type Writer<W: Write> = AnnotationWriter<I, W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    AnnotationWriter {
      config: self.clone(),
      write: Indented::from(self.fill).writer(w),
      span: None,
    }
  }
}

pub struct AnnotationWriter<I, W> {
  /// The input to write annotations on
  pub config: Annotated<I>,
  /// Where to write annotated output to
  pub write: IndentWriter<W>,
  /// The previous range that was displayed.\
  /// This lets us keep track of when to reprint the input\
  /// when displaying a new annotation / error
  span: Option<Range<usize>>,
}

pub const LINE_LENGTH: usize = 80;

/// Calculates the range within which the `index` fits in `input`
fn range_for(input: &str, index: usize) -> Range<usize> {
  let mut start = input[..index].rfind('\n').unwrap_or(0);
  let mut end = input[start..].find('\n').unwrap_or(input.len());

  if end - start <= LINE_LENGTH {
    return start..end;
  }
  start = start
    .min(index.saturating_sub(LINE_LENGTH / 2))
    .min(end.saturating_sub(LINE_LENGTH));
  end = end
    .max(index.saturating_add(LINE_LENGTH / 2))
    .max(start.saturating_add(LINE_LENGTH));
  start..end
}

/// Calculates the position of a cursor within the input `str`
fn cursor_position(input: &str, rest: &str) -> (Range<usize>, usize) {
  let index = input.offset(rest);
  let Range { start, end } = range_for(input, index);
  let offset = input[start..index].chars().count() + 2;
  (start..end, offset)
}

// @todo maybe we shouldn't be using `AsRef` here?\
// we ideally want a trait that represents how far into\
// the `Display`ed value of `self.config.input` `rest` is...
impl<I: AsRef<str>, W: Write> AnnotationWriter<I, W> {
  /// Writes a cursor corresponding to an update to the span of the input
  fn new_span_cursor(&mut self, span: Range<usize>, idx: usize) -> std::fmt::Result {
    let input = self.config.input.as_ref();
    writeln!(self.write.write, "`{}`", &input[span])?;
    self.span_cursor(idx)
  }

  /// Writes a cursor corresponding to no update to the span of the input
  fn span_cursor(&mut self, idx: usize) -> std::fmt::Result {
    let Annotated { fill, caret, .. } = self.config;
    let space = fill.to_string().repeat(idx);
    write!(self.write.write, "{space}{caret}{fill}")?;
    self.write.indent = idx + 2;
    Ok(())
  }

  /// Sets the location of the next annotation to write
  pub fn set_origin(&mut self, rest: impl AsRef<str>) -> std::fmt::Result {
    let i_str = self.config.input.as_ref();
    let (range, idx) = cursor_position(i_str, rest.as_ref());

    // first annotation made => new cursor
    let Some(span) = self.span.take() else {
      self.new_span_cursor(range.clone(), idx)?;
      self.span = Some(range);
      return Ok(());
    };
    self.write.write.write_char('\n')?;

    // span != range => new cursor
    if span != range {
      self.new_span_cursor(range.clone(), idx)?;
      self.span = Some(range);
      return Ok(());
    }

    // no change to span => re-use cursor
    self.span_cursor(idx)?;
    self.span = Some(span);
    Ok(())
  }
}

impl<I, W: Write> Write for AnnotationWriter<I, W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    self.write.write_str(s)
  }
}

impl<I, W> Flush for AnnotationWriter<I, W> {}
