use crate::{ident::Ident, InfoSource};

use super::{Struct, Tuple};

#[derive(InfoSource)]
pub enum Variant<I> {
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Enum(Enum<I>),
}

#[derive(InfoSource)]
pub struct Enum<I> {
  pub variant: Ident<I>,
  pub value: Box<Variant<I>>,
  pub info: I,
}
