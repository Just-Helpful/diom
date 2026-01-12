use super::{Pattern, Rest};
use crate::fmt::{bracket, OptionsDisplay};
use crate::path::Path;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum TupleItem<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

impl OptionsDisplay for TupleItem<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      TupleItem::Field(f) => f.optn_fmt(w, depth),
      TupleItem::Rest(r) => r.optn_fmt(w, depth),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<TupleItem<I>>,
  pub info: I,
}

impl OptionsDisplay for Tuple<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("tuple", self.info.len()));
    if let Some(name) = &self.name {
      name.optn_fmt(w, depth + 1)?;
    }
    for field in &self.fields {
      field.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
