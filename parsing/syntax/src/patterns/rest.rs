use crate::fmt::{bracket, OptionsDisplay};
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

/// A pattern that captures the remaining contents of a structure.
/// For example:
/// ```ignore
/// let [1, *xs] = [1, 2, 3];
/// assert(xs == [2, 3]);
///
/// let {x: 1, *sx} = {x: 1, y: 2, z: 3};
/// assert(sx == {y: 2, z: 3});
///
/// let (1, *tx) = (1, True, "");
/// assert(tx == (True, ""));
/// ```
///
/// @note as diom doesn't allow empty structs, attempting to bind a rest<br>
/// parameter that would create one is a compiler error:
///
/// ```ignore
/// let {x: 1, *sx} = {x: 1};
/// // gives a compiler error (whilst type checking)
/// // that `sx` cannot be bound to an empty structure
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Rest<I> {
  pub name: Option<Ident<I>>,
  pub info: I,
}

impl OptionsDisplay for Rest<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("rest", self.info.len()));
    if let Some(name) = &self.name {
      name.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
