use std::ops::Range;

use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use crate::fmt::{bracket, OptionsDisplay};

/// The type for single characters
///
/// ```ignore
/// let SingleString: Char;
/// let c: Char = 'v';
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Char<I> {
  pub info: I,
}

impl OptionsDisplay for Char<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("", self.info.len()));
    Ok(())
  }
}
