use std::ops::Range;

use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use crate::fmt::{bracket, MultiDisplay};

/// The type for floating point numbers
///
/// ```ignore
/// let Number: Float;
/// let x: Float = 1;
/// let x: Float = 1.0;
/// let x: Float = 1.0e1;
/// let x: Float = -1e-1;
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Float<I> {
  pub info: I,
}

impl MultiDisplay for Float<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("", self.info.len()));
    Ok(())
  }
}
