use diom_info::{InfoMap, InfoRef, InfoSource};

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
