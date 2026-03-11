use crate::{Flush, Format, Updater};
use std::cell::RefCell;
use std::fmt::{Debug, Write};
use std::num::NonZero;
use std::rc::Rc;

/// A format that supports writing to multiple lines at once
#[derive(Clone, Copy)]
pub struct Lines {
  /// The character used for filling empty space
  pub fill: char,
}
impl Default for Lines {
  fn default() -> Self {
    Self { fill: ' ' }
  }
}
impl From<char> for Lines {
  fn from(value: char) -> Self {
    Self { fill: value }
  }
}

impl Format for Lines {
  type Writer<W: Write> = LineWriter<W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    LineWriter {
      config: *self,
      line: 0,
      col: 0,
      byte: 0,
      lines: Rc::new(RefCell::new(vec![])),
      write: Rc::new(RefCell::new(w)),
    }
  }
}

/// A variant of a `Write` implementation\
/// that supports writing and seeking to multiple lines of text
///
/// ## Todo
///
/// Increment to next line when a newline `\n` is written
pub struct LineWriter<W> {
  /// The config for filling empty lines
  config: Lines,
  /// The line number that is currently being written to
  line: usize,
  /// The column number that is currently being written to
  col: usize,
  /// The current byte offset of the column number.\
  /// We need to keep this as rust `str::split_at`\
  /// only supports byte offsets, not char offsets.
  byte: usize,
  /// The string content of each line
  lines: Rc<RefCell<Vec<String>>>,
  /// The wrapped writer
  write: Rc<RefCell<W>>,
}

impl<W> Clone for LineWriter<W> {
  fn clone(&self) -> Self {
    Self {
      config: self.config,
      line: self.line,
      col: self.col,
      byte: self.byte,
      lines: self.lines.clone(),
      write: self.write.clone(),
    }
  }
}

impl<W> LineWriter<W> {
  /// Pushes a single character to the end of the current line
  /// SAFETY: should be used when the cursor is at the end of the line
  unsafe fn push_char(&mut self, c: char) {
    self.lines.borrow_mut()[self.line].push(c);
    self.byte += c.len_utf8();
    self.col += 1;
  }

  /// Pushes a string to the end of the current line
  /// SAFETY: should be used when the cursor is at the end of the line
  unsafe fn push_str(&mut self, s: &str) {
    let len = s.chars().count();
    self.lines.borrow_mut()[self.line].push_str(s);
    self.byte += s.len();
    self.col += len;
  }

  #[inline]
  fn unchecked_seek_col(&mut self, loc: usize) {
    let line = &mut self.lines.borrow_mut()[self.line];
    let rem = match byte_at(&line, loc) {
      Ok(byte) => {
        self.byte = byte;
        self.col = loc;
        return; // within line
      }
      Err(rem) => rem,
    };

    line.push_str(&self.config.fill.to_string().repeat(rem.get()));
    self.byte = line.len();
    self.col = loc;
  }

  /// Seeks the cursor to a specific column in the current line
  #[inline]
  pub fn seek_col(&mut self, loc: usize) {
    if loc != self.col {
      self.unchecked_seek_col(loc);
    }
  }

  #[inline]
  fn unchecked_seek_line(&mut self, loc: usize) {
    // if the line number is outside the buffer
    let len = self.lines.borrow().len();
    if len <= loc {
      let len = loc - len;
      let mut lines = self.lines.borrow_mut();
      lines.extend(vec![String::from(""); len]);
      lines.push(self.config.fill.to_string().repeat(self.col));

      self.byte = self.config.fill.len_utf8() * self.col;
      self.line = loc;
      return;
    }
  }

  /// Seeks the cursor to a specific line in the block
  pub fn seek_line(&mut self, loc: usize) {
    if loc != self.line {
      self.unchecked_seek_line(loc);
      // By moving to a different line we might either be:
      // 1. off of the end of the line
      // 2. at the incorrect `self.byte`
      // this fixes both of these cases
      self.unchecked_seek_col(self.col);
    }
  }

  /// Moves the cursor to a given position in the lines
  /// `loc` can be one of:
  /// - `[]`: the cursor doesn't move
  /// - `col`: the cursor moves to column `col`
  /// - `[col]`: the cursor moves to column `col`
  /// - `[col, line]`: the cursors moves to column `col` and line `line`
  pub fn seek(&mut self, loc: impl Updater<[usize; 2]> + Debug) {
    // work out the updated line and column number
    let [i, j] = loc.update([self.col, self.line]);

    // if the line number is outside the buffer
    if self.lines.borrow().len() <= j {
      let len = j - self.lines.borrow().len();
      self.lines.borrow_mut().extend(vec![String::from(""); len]);
      let fill = self.config.fill.to_string().repeat(i);
      self.lines.borrow_mut().push(fill);

      self.byte = self.config.fill.len_utf8() * i;
      self.line = j;
      self.col = i;
      return;
    }

    self.line = j;
    self.seek_col(i);
  }
}

impl<W> Write for LineWriter<W> {
  fn write_char(&mut self, c: char) -> std::fmt::Result {
    let len = self.lines.borrow()[self.line].len();
    if self.byte == len {
      // SAFETY: cursor is at the end of the line
      unsafe { self.push_char(c) }
      return Ok(());
    }

    let line = &mut self.lines.borrow_mut()[self.line];
    let tail = line.split_off(self.byte);
    let byte = tail
      .char_indices()
      .nth(1)
      .map_or(tail.len(), |(byte, _)| byte);

    line.push(c);
    line.push_str(&tail[byte..]);
    self.byte += c.len_utf8();
    self.col += 1;
    Ok(())
  }

  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let len = self.lines.borrow()[self.line].len();
    if self.byte == len {
      // SAFETY: cursor is at the end of the line
      unsafe { self.push_str(s) }
      return Ok(());
    }

    let line = &mut self.lines.borrow_mut()[self.line];
    let nchars = s.chars().count();
    let tail = line.split_off(self.byte);
    let byte = tail
      .char_indices()
      .nth(nchars)
      .map_or(tail.len(), |(byte, _)| byte);

    line.push_str(s);
    line.push_str(&tail[byte..]);
    self.byte += s.len();
    self.col += nchars;
    Ok(())
  }
}

impl<W> LineWriter<W> {
  /// Writes `text` to the specified cursor position, overwriting text present
  pub fn write_at(&mut self, loc: impl Updater<[usize; 2]> + Debug, text: impl AsRef<str>) {
    self.seek(loc);
    self.write_str(text.as_ref()).unwrap();
  }
}

impl<W: Write> Flush for LineWriter<W> {
  fn flush(&mut self) -> std::fmt::Result {
    let lines = self.lines.borrow();
    let mut iter = lines.iter().rev();
    let Some(line) = iter.next() else {
      return Ok(());
    };

    self.write.borrow_mut().write_str(&line)?;
    for line in iter {
      self.write.borrow_mut().write_char('\n')?;
      self.write.borrow_mut().write_str(&line)?;
    }
    Ok(())
  }
}

/// Returns the byte at the given character index\
/// or the remainder of `i` after the bytes have been exhausted.
pub fn byte_at(s: &str, mut i: usize) -> Result<usize, NonZero<usize>> {
  let mut bytes = s
    .char_indices()
    .map(|idx| idx.0)
    .chain(std::iter::once(s.len()));

  loop {
    let Some(byte) = bytes.next() else {
      return Err(NonZero::try_from(i + 1).expect("i is a `usize`"));
    };
    if i == 0 {
      return Ok(byte);
    }
    i -= 1;
  }
}

#[cfg(test)]
mod tests {
  use std::num::NonZero;

  use super::byte_at;

  #[test]
  fn test_byte_at() {
    assert_eq!(byte_at("", 0), Ok(0));
    assert_eq!(byte_at("s", 0), Ok(0));
    assert_eq!(byte_at("s", 1), Ok(1));

    for i in 0..(2 << 24) {
      let Some(c) = char::from_u32(i) else { continue };
      assert_eq!(byte_at(&c.to_string(), 1), Ok(c.len_utf8()));
    }

    for i in 1..(2 << 12) {
      assert_eq!(byte_at("", i), Err(NonZero::try_from(i).unwrap()))
    }
  }
}
