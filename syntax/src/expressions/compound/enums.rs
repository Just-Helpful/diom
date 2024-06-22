use crate::ident::Ident;

use super::{Struct, Tuple};

pub enum SubEnum<I> {
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Enum(Enum<I>),
}

pub struct Enum<I> {
  pub variant: Ident<I>,
  pub value: Box<SubEnum<I>>,
  pub info: I,
}
