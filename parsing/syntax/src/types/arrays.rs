use super::Type;
use crate::{
  fmt::{bracket, OptionsDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

/// A type for arrays of items.
///
/// ```ignore
/// type String [Char; _];
/// type Nums = [Float];
///
/// let greeting: String = "Hello!";
/// let xs: Nums = [1, 2, 3];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub name: Option<Ident<I>>,
  pub item: Box<Type<I>>,
  pub info: I,
}

impl OptionsDisplay for Array<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("array", self.info.len()));
    if let Some(name) = &self.name {
      name.optn_fmt(w, depth + 1)?;
    }
    self.item.optn_fmt(w, depth + 1)
  }
}
