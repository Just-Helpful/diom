use crate::{ident::Ident, InfoSource};

use super::Expression;

#[derive(InfoSource)]
pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}
