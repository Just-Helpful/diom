use std::ops::Range;

use crate::fmt::{bracket, OptionsDisplay};

use super::Expression;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Group<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl OptionsDisplay for Group<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("group", self.info.len()));
    Ok(())
  }
}
