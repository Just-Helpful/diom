use crate::InfoSource;

#[derive(InfoSource)]
pub struct Char<I> {
  pub value: char,
  pub info: I,
}
