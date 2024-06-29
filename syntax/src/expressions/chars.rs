use crate::InfoSource;

#[derive(InfoSource, Clone)]
pub struct Char<I> {
  pub value: char,
  pub info: I,
}
