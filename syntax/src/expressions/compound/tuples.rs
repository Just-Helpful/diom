use crate::{ident::Ident, InfoSource};

use super::Expression;

#[derive(InfoSource)]
pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Expression<I>>,
  pub info: I,
}
