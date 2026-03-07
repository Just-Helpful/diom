use super::LineWriter;
use crate::{formats::lines::Lines, Flush, Format, Write};
use std::{cell::RefCell, ops::Range, rc::Rc};

/// Displays the code spans associated with a particular code fragment.\
/// The code spans represent the syntax nodes that cover the fragment.
///
/// ## Example
///
/// ```ignore
/// let json = "{"a": [4, 5, 3], "b": 2}".parse()?;
/// json.display::<Spans>()
/// // "{"a": [4, 5, 3], "b": 2}"
/// //   ╰─╯   U  U  U   ╰─╯  U
/// //        ╰─list──╯
/// //  ╰────group────╯
/// ```
#[derive(Clone, Copy)]
pub struct Spans {
  pub single: char,
  pub open: char,
  pub close: char,
  pub fill: char,
}

impl Default for Spans {
  fn default() -> Self {
    Self::curly()
  }
}
impl Spans {
  /// A pure ASCII representation for span formatting
  #[inline]
  pub fn ascii() -> Self {
    Self {
      single: 'V',
      open: '\\',
      close: '/',
      fill: '_',
    }
  }

  /// A squared unicode representation
  #[inline]
  pub fn square() -> Self {
    Self {
      single: '⨆',
      open: '└',
      close: '┘',
      fill: '─',
    }
  }

  /// A rounded unicode representation
  #[inline]
  pub fn curly() -> Self {
    Self {
      single: 'U',
      open: '╰',
      close: '╯',
      fill: '─',
    }
  }
}

impl Format for Spans {
  type Writer<W: Write> = SpanWriter<W>;

  fn writer<W: Write>(&self, w: W) -> Self::Writer<W> {
    SpanWriter {
      config: self.clone(),
      lines: Rc::new(RefCell::new(Lines::default().writer(w))),
      depth: 0,
    }
  }
}

impl Spans {
  /// Displays a bracket with a given name and width.\
  /// This is intended to be used for span annotation.
  ///
  /// ## Examples
  ///
  /// ```
  /// # use diom_fmt::formats::spans::Spans;
  /// # fn test() {
  /// let span = Spans::default();
  /// assert_eq!(span.bracket_str("var", 0), "");
  /// assert_eq!(span.bracket_str("var", 1), "^");
  ///
  /// assert_eq!(span.bracket_str("var", 2), "()");
  /// assert_eq!(span.bracket_str("var", 4), "(  )");
  /// assert_eq!(span.bracket_str("var", 5), "(var)");
  /// assert_eq!(span.bracket_str("var", 7), "( var )");
  ///
  /// assert_eq!(span.bracket_str("float", 0), "");
  /// assert_eq!(span.bracket_str("float", 1), "^");
  /// assert_eq!(span.bracket_str("float", 5), "(   )");
  /// assert_eq!(span.bracket_str("float", 7), "(float)");
  /// assert_eq!(span.bracket_str("float", 8), "(float )");
  /// # }
  /// ```
  pub fn bracket_str(&self, name: &str, width: usize) -> String {
    let Spans {
      single,
      open,
      close,
      fill,
    } = self;
    if width == 0 {
      return "".into();
    }
    if width == 1 {
      return single.to_string();
    }

    let t_len = width - 2; // total
    if t_len < name.len() {
      return format!(
        "{open}{tfill}{close}",
        tfill = fill.to_string().repeat(t_len),
      );
    }

    let r_len = t_len - name.len(); //  remaining
    let f_len = r_len / 2; //           front
    let b_len = f_len + (r_len % 2); // back

    format!(
      "{open}{ffill}{name}{bfill}{close}",
      ffill = fill.to_string().repeat(f_len),
      bfill = fill.to_string().repeat(b_len),
    )
  }
}
pub struct SpanWriter<W> {
  /// The config for how to display spans
  config: Spans,
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
      config: self.config.clone(),
      lines: self.lines.clone(),
      depth: self.depth + 1,
    }
  }

  pub fn bracket(&mut self, name: impl AsRef<str>, range: &Range<usize>) -> std::fmt::Result {
    let Range { start, end } = *range;
    self.lines.borrow_mut().write_at(
      [start, self.depth],
      self.config.bracket_str(name.as_ref(), end - start),
    );
    Ok(())
  }
}
