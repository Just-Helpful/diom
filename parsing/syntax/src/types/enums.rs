use std::ops::Range;

use crate::{
  fmt::{bracket, OptionsDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use super::Type;

/// A type for combinations of possible types
///
/// ```ignore
/// type CharOption {
///   Some(Char),
///   None,
/// };
///
/// type Boolean: {
///   True,
///   False,
/// };
///
/// let c_optn = CharOption.Some('v');
/// let c_optn = CharOption.None;
///
/// let bool = Boolean.True;
/// let bool = Boolean.False;
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Enum<I> {
  pub name: Option<Ident<I>>,
  pub variants: Vec<(Ident<I>, Type<I>)>,
  pub info: I,
}

impl OptionsDisplay for Enum<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("enum", self.info.len()));
    if let Some(name) = &self.name {
      name.optn_fmt(w, depth + 1)?;
    }
    for (name, ty) in &self.variants {
      name.optn_fmt(w, depth + 1)?;
      ty.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
