use std::ops::Range;

use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use crate::fmt::{bracket, MultiDisplay};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Char<I> {
  #[map_ignore]
  pub value: char,
  pub info: I,
}

impl MultiDisplay for Char<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("char", self.info.len()));
    Ok(())
  }
}
