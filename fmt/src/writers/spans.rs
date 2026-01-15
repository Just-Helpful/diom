use super::LineWriter;
use crate::{writers::lines::Lines, Flush, Format, Write};
use std::{cell::RefCell, ops::Range, rc::Rc};

#[derive(Default, Clone, Copy)]
pub struct Spans;

impl Format for Spans {
  type Writer<W: Write> = SpanWriter<W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    SpanWriter {
      lines: Rc::new(RefCell::new(Lines::default().writer(w))),
      depth: 0,
    }
  }
}

pub struct SpanWriter<W> {
  /// The lines that the should be displayed
  ///
  /// ## Todo
  ///
  /// This currently requires runtime borrow checking but,\
  /// due to rust's guarantee that only one `&mut` reference exists,\
  /// this can theoretically be replaced by a type that supports\
  /// a "take-like" `Clone` which uses the following interface:
  ///
  /// ```_
  /// trait MutClone {
  ///   fn mut_clone(&mut self) -> Self;
  /// }
  /// ```
  ///
  /// Which can either:
  ///
  /// 1. include lifetimes.\
  ///   This make it *real* nasty to work with\
  ///   and I haven't found how to stop these\
  ///   lifetimes polluting the `CustomDisplay` trait.
  /// 2. be rather tricky to implement\
  ///   and likely include some unsafe tricks...
  lines: Rc<RefCell<LineWriter<W>>>,
  /// The depth of the current syntax node being written
  depth: usize,
}

impl<W> Write for SpanWriter<W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let mut lines = self.lines.borrow_mut();
    lines.seek_line(self.depth);
    lines.write_str(s)
  }
}

impl<W: Write> Flush for SpanWriter<W> {
  fn flush(&mut self) -> std::fmt::Result {
    self.lines.borrow_mut().flush()
  }
}

impl<W: Write> SpanWriter<W> {
  pub fn child(&mut self) -> SpanWriter<W> {
    SpanWriter {
      lines: self.lines.clone(),
      depth: self.depth + 1,
    }
  }

  pub fn bracket(&mut self, name: impl AsRef<str>, range: &Range<usize>) -> std::fmt::Result {
    let Range { start, end } = *range;
    self
      .lines
      .borrow_mut()
      .write_at([start, self.depth], bracket_str(name.as_ref(), end - start));
    Ok(())
  }
}

pub fn bracket_str(name: &str, width: usize) -> String {
  if width == 0 {
    return "".to_string();
  }
  if width == 1 {
    return '^'.to_string();
  }
  if width < name.len() + 2 {
    return String::from('(') + &" ".repeat(width - 2) + ")";
  }
  format!("({: ^1$})", name, width - 2)
}
