use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::{any, Arbitrary, BoxedStrategy, Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Char<I> {
  #[map_ignore]
  pub value: char,
  pub info: I,
}

impl<I> Display for Char<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.value)
  }
}

impl DisplayAs<Spans> for Char<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("char", &self.info)
  }
}

impl Char<()> {
  /// Generates a generic strategy for generating `Char` expressions
  pub fn any() -> impl Strategy<Value = Self> {
    any::<char>().prop_map(|value| Char { value, info: () })
  }
}
impl Arbitrary for Char<()> {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}

#[cfg(test)]
mod tests {
  use super::Char;

  #[test]
  fn formatting() {
    let s = Char {
      value: 'a',
      info: (),
    };
    assert_eq!(s.to_string(), "'a'");

    let s = Char {
      value: '☃',
      info: (),
    };
    assert_eq!(s.to_string(), "'☃'");

    let s = Char {
      value: '\'',
      info: (),
    };
    assert_eq!(s.to_string(), r"'\''");
  }
}
