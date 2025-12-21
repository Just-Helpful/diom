use std::ops::Range;

use crate::{
  fmt::{bracket, MultiDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

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

impl MultiDisplay for Tuple<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("tuple", self.info.len()));
    if let Some(name) = &self.name {
      name.multi_fmt(w, depth + 1)?;
    }
    for ty in &self.fields {
      ty.multi_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
