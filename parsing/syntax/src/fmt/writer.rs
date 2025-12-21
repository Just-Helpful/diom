use crate::fmt::str_utils::byte_at;

use super::Updater;
use std::fmt::{Debug, Display, Write};

/// A variant of a `Write` implementation\
/// that supports writing and seeking to multiple lines of text
///
/// ## TODO
///
/// Increment to next line when a newline `\n` is written
#[derive(Clone, Debug, Default)]
pub struct MultiWriter<const FILL: char = ' '> {
  /// The line number that is currently being written to
  line: usize,
  /// The column number that is currently being written to
  col: usize,
  /// The current byte offset of the column number.\
  /// We need to keep this as rust `str::split_at`\
  /// only supports byte offsets, not char offsets.
  byte: usize,
  /// The string content of each line
  lines: Vec<String>,
}

impl<const FILL: char> MultiWriter<FILL> {
  /// Pushes a single character to the end of the current line
  /// SAFETY: should be used when the cursor is at the end of the line
  unsafe fn push_char(&mut self, c: char) {
    self.lines[self.line].push(c);
    self.byte += c.len_utf8();
    self.col += 1;
  }

  /// Pushes a string to the end of the current line
  /// SAFETY: should be used when the cursor is at the end of the line
  unsafe fn push_str(&mut self, s: &str) {
    let len = s.chars().count();
    self.lines[self.line].push_str(s);
    self.byte += s.len();
    self.col += len;
  }

  /// Seeks the cursor to a specific point in the line
  #[inline]
  fn seek_line(&mut self, loc: usize) {
    if loc == self.col {
      return; // early return on no change
    }

    let line = &mut self.lines[self.line];
    let rem = match byte_at(&line, loc) {
      Ok(byte) => {
        self.byte = byte;
        self.col = loc;
        return; // within line
      }
      Err(rem) => rem,
    };

    line.push_str(&FILL.to_string().repeat(rem.get()));
    self.byte = line.len();
    self.col = loc;
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
    if self.lines.len() <= j {
      let len = j - self.lines.len();
      self.lines.extend(vec![String::from(""); len]);
      self.lines.push(FILL.to_string().repeat(i));

      self.byte = FILL.len_utf8() * i;
      self.line = j;
      self.col = i;
      return;
    }

    self.line = j;
    self.seek_line(i);
  }
}

impl<const FILL: char> Write for MultiWriter<FILL> {
  fn write_char(&mut self, c: char) -> std::fmt::Result {
    let line = &mut self.lines[self.line];
    if self.byte == line.len() {
      unsafe { self.push_char(c) }
      return Ok(());
    }

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
    let len = s.chars().count();
    let line = &mut self.lines[self.line];
    if self.byte == line.len() {
      unsafe { self.push_str(s) }
      return Ok(());
    }

    let tail = line.split_off(self.byte);
    let byte = tail
      .char_indices()
      .nth(len)
      .map_or(tail.len(), |(byte, _)| byte);

    line.push_str(s);
    line.push_str(&tail[byte..]);
    self.byte += s.len();
    self.col += len;
    Ok(())
  }
}

impl<const FILL: char> MultiWriter<FILL> {
  /// Writes `text` to the specified cursor position, overwriting text present
  pub fn write_at(&mut self, loc: impl Updater<[usize; 2]> + Debug, text: impl AsRef<str>) {
    self.seek(loc);
    self.write_str(text.as_ref()).unwrap();
  }

  /// Writes the lines within the buffer to the given formatter\
  /// This isn't implemented with `Display` to avoid confusion.
  pub fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut iter = self.lines.iter().rev();
    let Some(line) = iter.next() else {
      return Ok(());
    };

    Display::fmt(&line, f)?;
    for line in iter {
      f.write_char('\n')?;
      Display::fmt(&line, f)?;
    }
    Ok(())
  }
}
