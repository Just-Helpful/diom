use crate::ident::Ident;

use super::Expression;

pub struct Field<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}
