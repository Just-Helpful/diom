use super::Expression;
use crate::{types::TypeDef, InfoSource};

#[derive(InfoSource, Clone)]
pub enum Statement<I> {
  Expression(Expression<I>),
  TypeDef(TypeDef<I>),
}

#[derive(InfoSource, Clone)]
pub struct Block<I> {
  pub statements: Vec<Statement<I>>,
  pub info: I,
}
