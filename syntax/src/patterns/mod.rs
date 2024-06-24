pub mod arrays;
use arrays::Array;
pub mod enums;
use enums::Enum;
pub mod rest;
use rest::Rest;
pub mod structs;
use structs::Struct;
use tuples::Tuple;

use crate::{ident::Ident, InfoSource};
pub mod tuples;

#[derive(InfoSource)]
pub enum Pattern<I> {
  Array(Array<I>),
  Enum(Enum<I>),
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Var(Ident<I>),
}
