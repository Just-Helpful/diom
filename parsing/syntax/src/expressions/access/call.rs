use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Debug, ops::Range};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Call<I> {
  pub value: Box<Expression<I>>,
  pub args: Vec<Expression<I>>,
  pub info: I,
}

impl MultiDisplay for Call<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("call", self.info.len()));
    self.value.multi_fmt(w, depth + 1)?;
    for arg in &self.args {
      arg.multi_fmt(w, depth + 1)?
    }
    Ok(())
  }
}
