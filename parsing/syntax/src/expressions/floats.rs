use crate::fmt::{bracket, MultiDisplay};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Float<I> {
  #[map_ignore]
  pub value: f64,
  pub info: I,
}

impl MultiDisplay for Float<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("float", self.info.len()));
    Ok(())
  }
}
