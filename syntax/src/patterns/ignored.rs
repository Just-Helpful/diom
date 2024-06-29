use crate::InfoSource;

#[derive(InfoSource, Clone)]
pub struct Ignored<I> {
  pub info: I,
}
