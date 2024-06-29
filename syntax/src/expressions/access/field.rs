use crate::{ident::Ident, InfoSource};

use super::Expression;

#[derive(InfoSource, Clone)]
pub struct Field<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}
