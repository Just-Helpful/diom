use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Return<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl MultiDisplay for Return<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("return", self.info.len()));
    self.value.multi_fmt(w, depth + 1)?;
    Ok(())
  }
}
