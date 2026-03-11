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

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<TupleItem<I>>,
  pub info: I,
}

impl<I> Display for Tuple<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
    Sep(&self.fields, ',').fmt(f)
  }
}

impl DisplayAs<Spans> for Tuple<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("tuple", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}
