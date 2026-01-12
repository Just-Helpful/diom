use super::{Pattern, Rest};
use crate::fmt::{bracket, OptionsDisplay};
use crate::path::Path;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

impl OptionsDisplay for ArrayItem<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      ArrayItem::Item(i) => i.optn_fmt(w, depth),
      ArrayItem::Rest(r) => r.optn_fmt(w, depth),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}

impl OptionsDisplay for Array<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("array", self.info.len()));
    if let Some(name) = &self.name {
      name.optn_fmt(w, depth + 1)?;
    }
    for item in &self.items {
      item.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
