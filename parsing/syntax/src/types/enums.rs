use std::{
  fmt::{Display, Write},
  ops::Range,
};

use crate::display::Sep;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy};

use super::{Tagged, Type};

/// A type for combinations of possible types
///
/// ```_
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
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Enum<I> {
  pub variants: Vec<Tagged<I>>,
  pub info: I,
}

impl<I> Display for Enum<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('{')?;
    Sep(&self.variants, ',').fmt(f)?;
    f.write_char('}')
  }
}

impl DisplayAs<Spans> for Enum<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("enum", &self.info)?;
    self.variants.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct EnumConfig(
  /// The maximum number of variants in an enum
  pub usize,
);
impl Default for EnumConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Enum<()> {
  /// Generates a generic strategy for generating `Enum` types
  pub fn any(
    item: impl Strategy<Value = Type<()>>,
    args: EnumConfig,
  ) -> impl Strategy<Value = Self> {
    vec(Tagged::any(item), 0..args.0).prop_map(|variants| Enum { variants, info: () })
  }
}
