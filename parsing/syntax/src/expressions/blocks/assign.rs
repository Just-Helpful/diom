use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Assign<I> {
  pub reference: Box<Expression<I>>,
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl MultiDisplay for Assign<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("assign", self.info.len()));
    self.reference.multi_fmt(w, depth + 1)?;
    self.value.multi_fmt(w, depth + 1)?;
    Ok(())
  }
}
