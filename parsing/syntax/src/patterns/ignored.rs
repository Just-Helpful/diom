use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::{Arbitrary, Just};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Ignored<I> {
  pub info: I,
}

impl<I> Display for Ignored<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('_')
  }
}

impl DisplayAs<Spans> for Ignored<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("ignored", &self.info)
  }
}

impl Ignored<()> {
  /// Generates a generic strategy for generating `Ignored` patterns
  pub fn any() -> Just<Self> {
    Just(Ignored { info: () })
  }
}
impl Arbitrary for Ignored<()> {
  type Parameters = ();
  type Strategy = Just<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any()
  }
}
