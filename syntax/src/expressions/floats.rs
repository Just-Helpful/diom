use crate::InfoSource;

#[derive(InfoSource)]
pub struct Float<I> {
  pub value: f64,
  pub info: I,
}
