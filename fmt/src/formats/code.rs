use std::cell::RefCell;
use std::ops::{Range, RangeFrom};
use std::{fmt::Write, rc::Rc};

use crate::{Flush, Format, IndentWriter, Indented};

/// Displays formatted code fragments
#[derive(Clone, Copy)]
pub struct Code<'a> {
  /// The maximum length for code lines
  pub max_len: usize,
  /// The character used for indenting code
  pub indent: &'a str,
}

impl<'a> Default for Code<'a> {
  fn default() -> Self {
    Self {
      max_len: 80,
      indent: "  ",
    }
  }
}

impl<'a> Format for Code<'a> {
  type Writer<W: Write> = CodeWriter<'a, W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    let write = Indented::from(self.indent).writer(w);
    CodeWriter {
      config: self.clone(),
      write,
      offset: 0,
      spans: Rc::new(RefCell::new(vec![Default::default()])),
      index: 0,
      line: Rc::new(RefCell::new(String::from(""))),
    }
  }
}

#[derive(Clone)]
pub enum SpanRange {
  Open(RangeFrom<usize>),
  Closed(Range<usize>),
}
impl Default for SpanRange {
  fn default() -> Self {
    Self::Open(0..)
  }
}

#[derive(Clone, Default)]
pub struct Span {
  /// The index of this spans' parent span
  pub parent: usize,
  /// Whether this span has been split across multiple lines
  pub split: bool,
  /// The range that this span occupies
  pub range: SpanRange,
}

/// # Implementation
///  
/// We want to keep a structure of spans that closely mirror the syntax nodes\
/// and have a way of annotating which of those spans are "linked"\
/// (i.e. when a span is moved to a newline, all "linked" spans should be too)
///
/// We could maybe use a slight heuristic of:
/// 1. all siblings of a writer are linked to the writer
/// 2. all spans that have the same char after them are linked
/// (Though this might not provide enough control...)
///
/// # Invariants
///
/// When one span `i` occurs before span `j` in `self.spans`, either:
/// - span `j` is a descendant of span `i`
/// - span `i` is closed
///
/// For the last closed span in `self.spans`, `SpanRange::Closed(Range {start, end})`\
/// `end - self.offset` must be less than `self.config.max_len`.
///
/// A closed span cannot contain any other spans
///
/// # Todo
///
/// Avoid using `Rc<RefCell<...>>` for internals.\
/// we could maybe do something where children `take` the current spans\
/// and then return them to their parent when they're `flush`-ed or `drop`-ed\
/// because we kind of have an invariant of one mut reference to a child at once.
pub struct CodeWriter<'a, W> {
  /// How to display code snippets
  config: Code<'a>,
  /// The wrapped writer
  write: IndentWriter<'a, W>,
  /// The offset the writer has reached in its output string
  offset: usize,
  /// All spans held by the writer
  spans: Rc<RefCell<Vec<Span>>>,
  /// The index of the current span being modified
  index: usize,
  /// The current line to potentially be broken up
  line: Rc<RefCell<String>>,
}

impl<'a, W> CodeWriter<'a, W> {
  /// Splits the current `line` with the span at index `index`
  fn split_at(&mut self, index: usize) -> std::fmt::Result {
    debug_assert!(!self.spans.borrow()[index].split);
    self.spans.borrow_mut()[index].split = true;
    todo!("Work out what we can flush to our writer after splitting")
  }

  /// Splits the current `line` based on the formatting config
  fn try_split(&mut self) -> std::fmt::Result {
    while self.line.borrow().len() > self.config.max_len {
      // @todo we might not want to start splitting from the root node...
      // honestly this is just a matter of "run it on code, see how it feels"
      let Some(idx) = self.spans.borrow().iter().position(|span| {
        todo!(
          "
        work out if a span should be split based on prior rules...
        "
        )
      }) else {
        break;
      };
      self.split_at(idx)?
    }
    Ok(())
  }
}

impl<'a, W> Write for CodeWriter<'a, W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    self.line.borrow_mut().write_str(s)?;
    self.try_split()
  }
}

impl<'a, W> Flush for CodeWriter<'a, W> {
  fn flush(&mut self) -> std::fmt::Result {
    todo!(
      "
    1. close the currently indexed span
    2. based on prior decisions write the closed span
    3. if we're the root span, flush the changes to `self.write`
    "
    );
  }
}
