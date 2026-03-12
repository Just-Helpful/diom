use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::{any, Arbitrary, BoxedStrategy, Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Float<I> {
  #[map_ignore]
  pub value: f64,
  pub info: I,
}

impl<I> Display for Float<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for Float<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("float", &self.info)
  }
}

impl Float<()> {
  /// Generates a generic strategy for generating `Float` expressions
  pub fn any() -> impl Strategy<Value = Self> {
    any::<f64>().prop_map(|value| Float { value, info: () })
  }
}
impl Arbitrary for Float<()> {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}
