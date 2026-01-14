use super::LineWriter;
use std::{cell::RefCell, fmt::Display, ops::Range, rc::Rc};

#[derive(Default, Clone)]
pub struct SpanWriter {
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
  lines: Rc<RefCell<LineWriter>>,
  /// The depth of the current syntax node being written
  depth: usize,
}

impl<'a> SpanWriter {
  pub fn child(&mut self) -> SpanWriter {
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

impl<'a> Display for SpanWriter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.lines.borrow().fmt(f)
  }
}
