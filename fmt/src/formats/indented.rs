use crate::{Flush, Format};
use std::fmt::Write;

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

impl<'a> Format for Indented<'a> {
  type Writer<W: Write> = IndentWriter<'a, W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    IndentWriter {
      config: self.clone(),
      indent: 0,
      write: w,
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
  write: W,
}

impl<'a, W: Write> Write for IndentWriter<'a, W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let mut lines = s.lines();
    let Some(line) = lines.next() else {
      return Ok(());
    };

    self.write.write_str(line)?;
    for line in lines {
      self.write.write_char('\n')?;
      self
        .write
        .write_str(&self.config.indent.repeat(self.indent))?;
      self.write.write_str(line)?;
    }
    Ok(())
  }
}

impl<'a, W> Flush for IndentWriter<'a, W> {
  fn flush(&mut self) -> std::fmt::Result {
    Ok(())
  }
}
