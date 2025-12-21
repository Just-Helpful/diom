use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub contents: Vec<Expression<I>>,
  pub info: I,
}

impl MultiDisplay for Array<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("array", self.info.len()));
    for expr in &self.contents {
      expr.multi_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
