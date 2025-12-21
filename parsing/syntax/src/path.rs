use crate::fmt::{bracket, MultiDisplay};
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}

impl MultiDisplay for Path<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("path", self.info.len()));
    for seg in &self.segments {
      seg.multi_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
