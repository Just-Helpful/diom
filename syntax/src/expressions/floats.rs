use crate::InfoSource;

#[derive(InfoSource, Clone)]
pub struct Float<I> {
  pub value: f64,
  pub info: I,
}
