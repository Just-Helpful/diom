use crate::{Flush, Format};
use std::fmt::Write;

pub struct Indented<'a>(&'a str);

impl<'a> Format for Indented<'a> {
  type Writer<W: Write> = IndentedWriter<'a, W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    IndentedWriter {
      single: self.0,
      indent: 0,
      write: w,
    }
  }
}

/// A writer that allow for the display of indented text
pub struct IndentedWriter<'a, W> {
  /// The character used to represent a single indent
  single: &'a str,
  /// The current level of indentation
  indent: usize,
  /// The wrapped writer
  write: W,
}

impl<'a, W: Write> Write for IndentedWriter<'a, W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let mut lines = s.lines();
    let Some(line) = lines.next() else {
      return Ok(());
    };

    self.write.write_str(line)?;
    for line in lines {
      self.write.write_char('\n')?;
      self.write.write_str(&self.single.repeat(self.indent))?;
      self.write.write_str(line)?;
    }
    Ok(())
  }
}

impl<'a, W> Flush for IndentedWriter<'a, W> {
  fn flush(&mut self) -> std::fmt::Result {
    Ok(())
  }
}
