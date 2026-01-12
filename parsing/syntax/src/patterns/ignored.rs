use crate::fmt::{bracket, OptionsDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Ignored<I> {
  pub info: I,
}

impl OptionsDisplay for Ignored<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("", self.info.len()));
    Ok(())
  }
}
