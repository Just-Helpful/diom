use crate::ident::Ident;
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

use super::Type;

/// The type for a combination of indexed fields
///
/// ```ignore
/// let Vec2 [Float, Float];
/// let Email: [String, String];
///
/// let vec2: Vec2 = Vec2 [1.2, 3.0];
/// let bobs_email: Email = ["bob.jones", "hotmail.com"];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Type<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Tuple<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("tuple", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}
