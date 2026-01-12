use super::Expression;
use crate::fmt::{bracket, OptionsDisplay};
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}

impl OptionsDisplay for Struct<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("struct", self.info.len()));
    for (name, expr) in &self.fields {
      name.optn_fmt(w, depth + 1)?;
      expr.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
