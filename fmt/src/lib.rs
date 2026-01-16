//! Utilities for fine-grained control over displaying data.
use std::fmt::{Display, Formatter, Write};

mod blanket_impls;
mod updater;
pub use updater::Updater;
pub mod writers;
pub use writers::{LineWriter, Lines, SpanWriter, Spans};

/// A `Write` trait that can be flushed.\
/// This is similar to `std::io::Write`,\
/// but doesn't include vectorised writing.
pub trait Flush {
  fn flush(&mut self) -> std::fmt::Result {
    Ok(())
  }
}

// Default implementations for common types
impl<W: Flush> Flush for &mut W {
  fn flush(&mut self) -> std::fmt::Result {
    <W as Flush>::flush(self)
  }
}
impl Flush for String {}
impl<'a> Flush for Formatter<'a> {}

/// A type that can create custom writers.\
/// This can be used to create wrappers that handle:
///
/// 1. multiline writing
/// 2. indented writing for nested structures
/// 3. custom styling / colouring
pub trait Format {
  /// The custom writer this format generates
  type Writer<W: Write>: Write + Flush;

  /// Construct this format's `Writer` by wrapping `W`
  fn writer<W: Write>(&self, w: W) -> Self::Writer<W>;
}

/// A `Display` implementation that supports custom formats.\
/// The format `F` specifies how `Self` should be formatted.
pub trait DisplayAs<F: Format>: Sized {
  #[must_use]
  fn write<W: Write>(&self, w: &mut F::Writer<W>) -> std::fmt::Result;

  fn display<F0: Format + Default>(&self) -> As<F0, &Self>
  where
    Self: DisplayAs<F0>,
  {
    As(self, F0::default())
  }

  fn display_with<F0: Format>(&self, cfg: impl Into<F0>) -> As<F0, &Self>
  where
    Self: DisplayAs<F0>,
  {
    As(self, cfg.into())
  }
}

impl<F: Format, D: DisplayAs<F>> DisplayAs<F> for &D {
  fn write<W: Write>(&self, w: &mut F::Writer<W>) -> std::fmt::Result {
    DisplayAs::write(*self, w)
  }
}

/// Displays a `MultiDisplay`-able type with custom config
pub struct As<C: Format, D: DisplayAs<C>>(pub D, pub C);

impl<F: Format + Default, D: DisplayAs<F>> From<D> for As<F, D> {
  fn from(value: D) -> Self {
    Self(value, Default::default())
  }
}

impl<F: Format + Clone, D: DisplayAs<F>> Display for As<F, D> {
  fn fmt<'a, 'b>(&'a self, f: &'b mut std::fmt::Formatter) -> std::fmt::Result {
    let mut writer = self.1.writer(f);
    self.0.write(&mut writer)?;
    writer.flush()
  }
}
