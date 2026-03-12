use super::{Pattern, Rest};
use crate::{
  display::{Optn, Sep},
  path::{Path, PathConfig},
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, option, prelude::Strategy, prop_oneof};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
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

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}

impl<I> Display for Array<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
    Sep(&self.items, ',').fmt(f)
  }
}

impl DisplayAs<Spans> for Array<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.name.write(&mut w.child())?;
    self.items.write(&mut w.child())
  }
}

pub struct ArrayConfig(
  /// The config for the name of the array
  pub PathConfig,
  /// The maximum number of items in an array
  pub usize,
);
impl Default for ArrayConfig {
  fn default() -> Self {
    Self(Default::default(), 50)
  }
}
impl Array<()> {
  /// Generates a generic strategy for generating `Ident`s
  pub fn any(
    item: impl Strategy<Value = Pattern<()>>,
    args: ArrayConfig,
  ) -> impl Strategy<Value = Self> {
    (
      option::of(Path::any(args.0)),
      vec(ArrayItem::any(item), 0..args.1),
    )
      .prop_map(|(name, items)| Array {
        name,
        items,
        info: (),
      })
  }
}
