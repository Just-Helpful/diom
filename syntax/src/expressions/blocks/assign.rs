use super::Expression;
use crate::{ident::Ident, InfoSource};

#[derive(InfoSource, Clone)]
pub struct Assign<I> {
  pub name: Ident<I>,
  pub value: Box<Expression<I>>,
  pub info: I,
}
