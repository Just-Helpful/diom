use std::fmt::{Display, Write};

mod updater;
pub use updater::Updater;
mod writer;
pub use writer::MultiWriter;
mod str_utils;

/// A multiline `Display` trait that supports configurable options.
///
/// # Note
///
/// This doesn't use formatter options as they're currently unstable...
pub trait MultiDisplay<W: Write + Display = MultiWriter> {
  type Options;

  #[must_use]
  fn multi_fmt(&self, w: &mut W, options: Self::Options) -> std::fmt::Result;

  /// Displays a `MultiDisplay`-able type with default options
  fn display(&self) -> DisplayWith<W, &Self>
  where
    Self: Sized,
    Self::Options: Default,
  {
    DisplayWith(self, Default::default())
  }

  /// Displays a `MultiDisplay`-able type with custom options
  fn display_with(&self, options: Self::Options) -> DisplayWith<W, &Self>
  where
    Self: Sized,
  {
    DisplayWith(self, options)
  }
}

impl<W: Write + Display, D: MultiDisplay<W>> MultiDisplay<W> for &D {
  type Options = D::Options;
  fn multi_fmt(&self, w: &mut W, options: Self::Options) -> std::fmt::Result {
    MultiDisplay::multi_fmt(*self, w, options)
  }
}

/// Displays a `MultiDisplay`-able type with custom options
pub struct DisplayWith<W: Write + Display, D: MultiDisplay<W>>(pub D, pub D::Options);

impl<W: Write + Display, D: MultiDisplay<W, Options: Default>> From<D> for DisplayWith<W, D> {
  fn from(value: D) -> Self {
    Self(value, Default::default())
  }
}

impl<W: Write + Display + Default, D: MultiDisplay<W, Options: Clone>> Display
  for DisplayWith<W, D>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut w = W::default();
    self.0.multi_fmt(&mut w, self.1.clone())?;
    w.fmt(f)
  }
}

pub fn bracket(name: &str, width: usize) -> String {
  if width == 0 {
    return "".to_string();
  }
  if width == 1 {
    return '╵'.to_string();
  }
  if width < name.len() + 2 {
    return String::from('╰') + &"─".repeat(width - 2) + "╯";
  }
  String::from('╰') + &format!("{:─^1$}", name, width - 2) + "╯"
}
