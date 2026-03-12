use crate::{display::Sep, ident::Ident};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{
  collection::vec,
  prelude::{Arbitrary, BoxedStrategy, Strategy},
};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}

impl<I> Display for Path<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Sep(&self.segments, '.').fmt(f)
  }
}

impl DisplayAs<Spans> for Path<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("path", &self.info)?;
    self.segments.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct PathConfig(
  /// The max number of segments in a path
  pub usize,
);
impl Default for PathConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Path<()> {
  /// Generates a generic strategy for generating `Path`s
  pub fn any(args: PathConfig) -> impl Strategy<Value = Self> {
    vec(Ident::any(), 0..args.0).prop_map(|segments| Path { segments, info: () })
  }
}
impl Arbitrary for Path<()> {
  type Parameters = PathConfig;
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
    Self::any(args).boxed()
  }
}
