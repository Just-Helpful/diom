use super::Expression;
use crate::fmt::{bracket, OptionsDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Index<I> {
  pub value: Box<Expression<I>>,
  pub key: Vec<Expression<I>>,
  pub info: I,
}

impl OptionsDisplay for Index<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("index", self.info.len()));
    self.value.optn_fmt(w, depth + 1)?;
    for key in &self.key {
      key.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
