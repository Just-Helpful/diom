use crate::InfoSource;

#[derive(InfoSource)]
pub struct Ignored<I> {
  pub info: I,
}
