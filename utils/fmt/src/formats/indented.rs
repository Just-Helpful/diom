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
#[derive(Clone)]
pub struct Indented {
  /// The string to use for indentation, i.e. `"\t"`, `"  "`
  pub indent: Box<str>,
}

impl From<Box<str>> for Indented {
  #[inline]
  fn from(value: Box<str>) -> Self {
    Self { indent: value }
  }
}
impl From<String> for Indented {
  #[inline]
  fn from(value: String) -> Self {
    value.into_boxed_str().into()
  }
}
impl From<char> for Indented {
  #[inline]
  fn from(value: char) -> Self {
    value.to_string().into()
  }
}

impl Format for Indented {
  type Writer<W: Write> = IndentWriter<W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    IndentWriter {
      config: self.clone(),
      indent: 0,
      write: w,
    }
  }
}

/// A writer that allow for the display of indented text
pub struct IndentWriter<W> {
  /// The config used for indentation
  config: Indented,
  /// The current level of indentation
  pub indent: usize,
  /// The wrapped writer
  pub write: W,
}

impl<W: Write> Write for IndentWriter<W> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let mut lines = s.split('\n'); // like `.lines()`, but includes line after last newline
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

impl<W> Flush for IndentWriter<W> {
  fn flush(&mut self) -> std::fmt::Result {
    Ok(())
  }
}
