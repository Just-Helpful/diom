use super::{Pattern, Rest};
use crate::{
  display::{Optn, Sep},
  ident::Ident,
  path::Path,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pattern: Pattern<I>,
  pub info: I,
}

impl<I> Display for StructField<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)?;
    self.pattern.fmt(f)
  }
}

impl DisplayAs<Spans> for StructField<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("field", &self.info)?;
    self.name.write(&mut w.child())?;
    self.pattern.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

impl<I> Display for StructItem<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Field(v) => v.fmt(f),
      Self::Rest(r) => r.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for StructItem<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Field(f) => f.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<StructItem<I>>,
  pub info: I,
}

impl<I> Display for Struct<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
    Sep(&self.fields, ',').fmt(f)
  }
}

impl DisplayAs<Spans> for Struct<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("struct", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}
