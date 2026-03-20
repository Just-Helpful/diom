use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Var<I> {
  #[map_ignore]
  pub name: Box<str>,
  pub info: I,
}

impl<I> Display for Var<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)
  }
}

impl DisplayAs<Spans> for Var<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("var", &self.info)
  }
}

impl Var<()> {
  pub fn any() -> impl Strategy<Value = Self> {
    "[_a-zA-Z][_a-zA-Z0-9]*".prop_map(|name| Var {
      name: name.into(),
      info: (),
    })
  }
}
