use crate::{path::Path, InfoSource};

use super::{Struct, Tuple};

#[derive(InfoSource, Clone)]
pub enum Variant<I> {
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Enum(Enum<I>),
}

#[derive(InfoSource, Clone)]
pub struct Enum<I> {
  pub variant: Path<I>,
  pub value: Box<Variant<I>>,
  pub info: I,
}
