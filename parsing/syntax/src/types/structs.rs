use std::ops::Range;

use crate::{
  fmt::{bracket, OptionsDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use super::Type;

/// The type for a combination of named fields.
///
/// ```ignore
/// let Vec2 {
///   x: Float,
///   y: Float,
/// };
/// let Email: {
///   name: [Char],
///   domain: [Char],
/// };
///
/// let vec2: Vec2 = Vec2 { x: 1.2, y: 3.0 };
/// let bobs_email: Email = { name: "bob.jones", domain: "hotmail.com" };
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<(Ident<I>, Type<I>)>,
  pub info: I,
}

impl OptionsDisplay for Struct<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("struct", self.info.len()));
    if let Some(name) = &self.name {
      name.optn_fmt(w, depth + 1)?;
    }
    for (name, ty) in &self.fields {
      name.optn_fmt(w, depth + 1)?;
      ty.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
