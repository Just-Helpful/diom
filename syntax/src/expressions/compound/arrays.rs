use crate::{ident::Ident, InfoSource};

use super::Expression;

#[derive(InfoSource)]
pub struct Array<I> {
  pub name: Option<Ident<I>>,
  pub contents: Vec<Expression<I>>,
  pub info: I,
}
