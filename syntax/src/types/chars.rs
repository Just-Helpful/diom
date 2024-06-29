use crate::InfoSource;
/// The type for single characters
///
/// ```ignore
/// let SingleString: Char;
/// let c: Char = 'v';
/// ```
#[derive(InfoSource, Clone)]
pub struct Char<I> {
  pub info: I,
}
