use std::fmt::Display;

mod blanket_impls;
mod updater;
pub use updater::Updater;
mod writers;
pub use writers::{LineWriter, SpanWriter};

/// A multiline `Display` trait that supports configurable options.
///
/// # Note
///
/// This doesn't use formatter options as they're currently unstable...
pub trait CustomDisplay<W: Display = LineWriter>: Sized {
  #[must_use]
  fn write(&self, w: &mut W) -> std::fmt::Result;

  fn display<W0: Display + Default>(&self) -> DisplayWith<W0, &Self>
  where
    Self: CustomDisplay<W0>,
  {
    DisplayWith(self, W0::default())
  }

  fn display_with<W0: Display>(&self, f: W0) -> DisplayWith<W0, &Self>
  where
    Self: CustomDisplay<W0>,
  {
    DisplayWith(self, f)
  }
}

impl<W: Display, D: CustomDisplay<W>> CustomDisplay<W> for &D {
  fn write(&self, w: &mut W) -> std::fmt::Result {
    CustomDisplay::write(*self, w)
  }
}

/// Displays a `MultiDisplay`-able type with custom options
pub struct DisplayWith<W: Display, D: CustomDisplay<W>>(pub D, pub W);

impl<W: Display + Default, D: CustomDisplay<W>> From<D> for DisplayWith<W, D> {
  fn from(value: D) -> Self {
    Self(value, Default::default())
  }
}

impl<W: Display + Clone, D: CustomDisplay<W>> Display for DisplayWith<W, D> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut w = self.1.clone();
    self.0.write(&mut w)?;
    w.fmt(f)
  }
}
