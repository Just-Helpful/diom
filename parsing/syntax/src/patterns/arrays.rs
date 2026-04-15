use super::{Pattern, Rest};
use crate::{display::Sep, Slice};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy, prop_oneof};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

impl<I> Display for ArrayItem<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Item(i) => i.fmt(f),
      Self::Rest(r) => r.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for ArrayItem<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Item(i) => i.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

impl ArrayItem<()> {
  /// Generates a generic strategy for generating `ArrayItem`s
  pub fn any(item: impl Strategy<Value = Pattern<()>>) -> impl Strategy<Value = Self> {
    prop_oneof![Rest::any().prop_map(Self::Rest), item.prop_map(Self::Item)]
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Array<I> {
  pub items: Slice<ArrayItem<I>>,
  pub info: I,
}

impl<I> Display for Array<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('[')?;
    Sep(&self.items, ',').fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Array<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.items.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct ArrayConfig(
  /// The maximum number of items in an array
  pub usize,
);
impl Default for ArrayConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Array<()> {
  /// Generates a generic strategy for generating `Ident`s
  pub fn any(
    item: impl Strategy<Value = Pattern<()>>,
    args: ArrayConfig,
  ) -> impl Strategy<Value = Self> {
    vec(ArrayItem::any(item), 0..args.0)
      .prop_map(Slice::from_iter)
      .prop_map(|items| Array { items, info: () })
  }
}
