use super::{Pattern, Rest};
use crate::fmt::{bracket, OptionsDisplay};
use crate::{ident::Ident, path::Path};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pattern: Pattern<I>,
  pub info: I,
}

impl OptionsDisplay for StructField<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("field", self.info.len()));
    self.name.optn_fmt(w, depth + 1)?;
    self.pattern.optn_fmt(w, depth + 1)?;
    Ok(())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

impl OptionsDisplay for StructItem<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      StructItem::Field(f) => f.optn_fmt(w, depth),
      StructItem::Rest(r) => r.optn_fmt(w, depth),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<StructItem<I>>,
  pub info: I,
}

impl OptionsDisplay for Struct<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("struct", self.info.len()));
    if let Some(n) = &self.name {
      n.optn_fmt(w, depth + 1)?;
    }
    for field in &self.fields {
      field.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
