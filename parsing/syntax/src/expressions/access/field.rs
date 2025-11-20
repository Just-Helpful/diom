use super::Expression;
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Field<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}
