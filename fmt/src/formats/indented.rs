use crate::{Flush, Format};
use std::{cell::RefCell, fmt::Write, rc::Rc};

/// Displays a multiline data structure with indents
///
/// ## Example
///
/// ```ignore
/// let json = "{"a": [4, 5, 3], "b": 2}".parse()?;
/// json.display();
/// // {"a": [4, 5, 3], "b": 2}
/// json.display::<Idented>();
/// // {
/// //   "a": [
/// //     4,     
/// //     5,     
/// //     3,     
/// //   ],
/// //   "b": 2,
/// // }
/// ```
#[derive(Clone, Copy)]
pub struct Indented<'a> {
  /// The string to use for indentation, i.e. `"\t"`, `"  "`
  pub indent: &'a str,
}

impl<'a> From<&'a str> for Indented<'a> {
  fn from(value: &'a str) -> Self {
    Indented { indent: value }
  }
}

impl<'a> Format for Indented<'a> {
  type Writer<W: Write> = IndentWriter<'a, W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    IndentWriter {
      config: self.clone(),
      indent: 0,
      write: Rc::new(RefCell::new(w)),
    }
  }
}

/// A writer that allow for the display of indented text
pub struct IndentWriter<'a, W> {
  /// The config used for indentation
  config: Indented<'a>,
  /// The current level of indentation
  indent: usize,
  /// The wrapped writer
  write: Rc<RefCell<W>>,
}

impl<'a, W> Clone for IndentWriter<'a, W> {
  fn clone(&self) -> Self {
    Self {
      config: self.config,
      indent: self.indent,
      write: self.write.clone(),
    }
  }
}

impl<'a, W> IndentWriter<'a, W> {
  /// Produces a child writer that is indented by 1 level
  pub fn child<'b: 'a>(&'a mut self) -> IndentWriter<'a, W> {
    let mut clone = self.clone();
    clone.indent += 1;
    clone
  }

  /// Produces the current width of indent in bytes
  pub fn indent_width(&self) -> usize {
    self.config.indent.len() * self.indent
  }
}

impl<'a, W: Write> Write for IndentWriter<'a, W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let mut lines = s.lines();
    let Some(line) = lines.next() else {
      return Ok(());
    };

    let mut writer = self.write.borrow_mut();
    writer.write_str(line)?;
    for line in lines {
      writer.write_char('\n')?;
      writer.write_str(&self.config.indent.repeat(self.indent))?;
      writer.write_str(line)?;
    }
    Ok(())
  }
}

impl<'a, W> Flush for IndentWriter<'a, W> {
  fn flush(&mut self) -> std::fmt::Result {
    Ok(())
  }
}
