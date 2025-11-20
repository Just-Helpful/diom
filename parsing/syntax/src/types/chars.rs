use diom_info_traits::{InfoMap, InfoRef, InfoSource};

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
