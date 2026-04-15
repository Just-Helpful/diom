use super::{Pattern, Rest};
use crate::{display::Sep, Seq};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy, prop_oneof};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub enum TupleItem<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

impl<I> Display for TupleItem<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Field(i) => i.fmt(f),
      Self::Rest(r) => r.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for TupleItem<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Field(f) => f.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

impl TupleItem<()> {
  /// Generates a generic strategy for generating `TupleItem` nodes
  pub fn any(item: impl Strategy<Value = Pattern<()>>) -> impl Strategy<Value = Self> {
    prop_oneof![Rest::any().prop_map(Self::Rest), item.prop_map(Self::Field)]
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Tuple<I> {
  pub fields: Seq<TupleItem<I>>,
  pub info: I,
}

impl<I> Display for Tuple<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('[')?;
    Sep(&self.fields, ',').fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Tuple<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("tuple", &self.info)?;
    self.fields.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct TupleConfig(
  /// The maximum number of items in a tuple
  pub usize,
);
impl Default for TupleConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Tuple<()> {
  /// Generates a generic strategy for generating `Tuple` patterns
  pub fn any(
    item: impl Strategy<Value = Pattern<()>>,
    args: TupleConfig,
  ) -> impl Strategy<Value = Self> {
    vec(TupleItem::any(item), 0..args.0)
      .prop_map(Seq::from_iter)
      .prop_map(|fields| Tuple { fields, info: () })
  }
}
