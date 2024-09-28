use super::Expression;
use crate::types::TypeDef;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub enum Statement<I> {
  Expression(Expression<I>),
  TypeDef(TypeDef<I>),
}

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Block<I> {
  pub statements: Vec<Statement<I>>,
  pub info: I,
}
