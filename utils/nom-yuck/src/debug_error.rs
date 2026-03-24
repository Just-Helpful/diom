use crate::annotated::AnnotationWriter;
use crate::Annotated;
use diom_fmt::DisplayAs;
use nom::error::ContextError;
use nom::error::ErrorKind;
use nom::error::FromExternalError;
use nom::error::ParseError;
use nom::Err;
use nom::Needed;
use nom::Offset as _;
use std::fmt::from_fn;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::Range;

#[derive(Clone, Eq, PartialEq)]
/// Error context for `ParserError`
pub enum DebugErrorKind {
  /// Static string added by the `context` function
  Context(&'static str),
  /// Indicates which character was expected by the `char` function
  Char(char),
  /// Error kind given by various nom parsers
  Nom(ErrorKind),
  /// Any external errors that have been thrown
  External(ErrorKind, String),
}

impl Debug for DebugErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Context(c) => write!(f, "Context({c:?})"),
      Self::Char(c) => write!(f, "Char({c:?})"),
      Self::Nom(k) => write!(f, "Nom({k:?})"),
      Self::External(k, d) => write!(f, "External({k:?}, {d})"),
    }
  }
}

impl Display for DebugErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Context(c) => write!(f, "with context: {c}"),
      Self::Char(c) => write!(f, "got an unexpected character {c:?}"),
      Self::Nom(k) => write!(f, "got a internal nom error {k:?}"),
      Self::External(k, d) => write!(f, "got an external error from {k:?}:\n{d}"),
    }
  }
}

/// Displays the kind of error that was encountered at a given position in input `i`, where `rest = input[i..]`
fn display_kind<I: Display>(
  rest: &I,
  kind: &DebugErrorKind,
  f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
  write!(f, "{rest}\n{kind}")
}

/// Similar to nom's `VerboseError` in that it creates a stack of errors\
/// but it also allows for custom error kinds that implement `Debug`.\
/// With some post processing, it can be used to display user friendly error messages
#[derive(Debug)]
pub struct DebugError<I> {
  pub(crate) errors: Vec<(I, DebugErrorKind)>,
}

impl<I> ParseError<I> for DebugError<I> {
  fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
    other.errors.push((input, DebugErrorKind::Nom(kind)));
    other
  }

  fn from_char(input: I, c: char) -> Self {
    Self {
      errors: vec![(input, DebugErrorKind::Char(c))],
    }
  }

  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    Self {
      errors: vec![(input, DebugErrorKind::Nom(kind))],
    }
  }
}

impl<I> ContextError<I> for DebugError<I> {
  fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
    other.errors.push((input, DebugErrorKind::Context(ctx)));
    other
  }
}

impl<I, E: Debug> FromExternalError<I, E> for DebugError<I> {
  fn from_external_error(input: I, kind: ErrorKind, e: E) -> Self {
    Self {
      errors: vec![(input, DebugErrorKind::External(kind, format!("{e:?}")))],
    }
  }
}

impl<I> DebugError<I> {
  /// Alters the input type taken by the error
  pub fn map_input<I1>(self, mut f: impl FnMut(I) -> I1) -> DebugError<I1> {
    DebugError {
      errors: self
        .errors
        .into_iter()
        .map(|(input, kind)| (f(input), kind))
        .collect(),
    }
  }
}

pub(crate) const LINE_LENGTH: usize = 80;

/// Calculates the range within which the `index` fits in `input`
pub(crate) fn range_for(input: &str, index: usize) -> Range<usize> {
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
pub(crate) fn cursor_position(input: &str, rest: &str) -> (Range<usize>, usize) {
  let index = input.offset(rest);
  let Range { start, end } = range_for(input, index);
  let offset = input[start..index].chars().count() + 2;
  (start..end, offset)
}

impl DebugError<&str> {
  /// Displays a stack trace for the parsing input
  pub fn display<'a>(&'a self, input: &'a str) -> impl Debug + 'a {
    from_fn(move |f| {
      let mut iter = self.errors.iter();
      let Some((rest, kind)) = iter.next() else {
        return Ok(());
      };
      let (mut range, idx) = cursor_position(input, rest);
      writeln!(f, "`{}`", &input[range.clone()])?;
      write!(f, "{:>idx$} {kind:?}", '^')?;

      for (rest, kind) in iter {
        let (nrange, idx) = cursor_position(input, rest);
        if nrange == range {
          write!(f, "\n{:>idx$} {kind:?}", '^')?;
          continue;
        }
        range = nrange;

        writeln!(f, "\n\n`{}`", &input[range.clone()])?;
        write!(f, "{:>idx$} {kind:?}", '^')?;
      }
      Ok(())
    })
  }
}

impl<I: Display> Display for DebugError<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut iter = self.errors.iter();
    let Some((rest, kind)) = iter.next() else {
      return Ok(());
    };
    display_kind(rest, kind, f)?;

    for (rest, kind) in iter {
      f.write_str("\n\n")?;
      display_kind(rest, kind, f)?;
    }
    Ok(())
  }
}

impl<I: AsRef<str>> DisplayAs<Annotated<&str>> for DebugError<I> {
  fn write<W: Write>(&self, w: &mut AnnotationWriter<&str, W>) -> std::fmt::Result {
    for (rest, kind) in &self.errors {
      w.set_origin(rest)?;
      write!(w, "{kind}")?;
    }
    Ok(())
  }
}

impl<I: AsRef<str>> DisplayAs<Annotated<&str>> for Err<DebugError<I>> {
  fn write<W: Write>(&self, w: &mut AnnotationWriter<&str, W>) -> std::fmt::Result {
    match &self {
      Err::Incomplete(n) => {
        w.write_str("Parsing expected more characters\n")?;
        w.set_origin(&w.config.input[w.config.input.len()..])?;
        match n {
          Needed::Unknown => write!(w, "Expected more characters"),
          Needed::Size(x) => write!(w, "Expected {x} more characters"),
        }
      }
      Err::Failure(e) => {
        w.write_str("Parsing failed with:\n")?;
        e.write(w)
      }
      Err::Error(e) => {
        w.write_str("Parsing errored with:\n")?;
        e.write(w)
      }
    }
  }
}
