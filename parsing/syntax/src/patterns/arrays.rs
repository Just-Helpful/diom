use super::{Pattern, Rest};
use crate::{
  display::{Optn, Sep},
  path::Path,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
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
