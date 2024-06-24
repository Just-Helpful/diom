use crate::{ident::Ident, InfoSource};

use super::Pattern;

#[derive(InfoSource)]
pub struct Enum<I> {
  pub path: Vec<Ident<I>>,
  pub variant: Box<Pattern<I>>,
  pub info: I,
}
