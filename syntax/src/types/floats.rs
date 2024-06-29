use crate::InfoSource;

/// The type for floating point numbers
///
/// ```ignore
/// let Number: Float;
/// let x: Float = 1;
/// let x: Float = 1.0;
/// let x: Float = 1.0e1;
/// let x: Float = -1e-1;
/// ```
#[derive(InfoSource, Clone)]
pub struct Float<I> {
  pub info: I,
}
