use crate::ident::Ident;

use super::Expression;

pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Expression<I>>,
  pub info: I,
}
