use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}

impl MultiDisplay for Struct<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("struct", self.info.len()));
    for (name, expr) in &self.fields {
      name.multi_fmt(w, depth + 1)?;
      expr.multi_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
