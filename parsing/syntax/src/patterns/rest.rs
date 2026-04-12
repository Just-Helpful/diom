use crate::{display::Optn, idents::Ident};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{
  option,
  prelude::{Arbitrary, BoxedStrategy, Strategy},
};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// A pattern that captures the remaining contents of a structure.
/// For example:
/// ```_
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
/// ```_
/// let {x: 1, *sx} = {x: 1};
/// // gives a compiler error (whilst type checking)
/// // that `sx` cannot be bound to an empty structure
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Rest<I> {
  pub name: Option<Ident<I>>,
  pub info: I,
}

impl<I> Display for Rest<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('*')?;
    Optn(&self.name).fmt(f)
  }
}

impl DisplayAs<Spans> for Rest<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("rest", &self.info)?;
    self.name.write(&mut w.child())
  }
}

impl Rest<()> {
  /// Generates a generic strategy for generating `Rest` patterns
  pub fn any() -> impl Strategy<Value = Self> {
    option::of(Ident::any()).prop_map(|name| Rest { name, info: () })
  }
}
impl Arbitrary for Rest<()> {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}
