use crate::{patterns::Pattern, types::Type};

use super::Expression;

pub struct Argument<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub info: I,
}

pub struct FunctionArm<I> {
  pub arguments: Vec<Argument<I>>,
  pub annotation: Option<Type<I>>,
  pub returned: Box<Expression<I>>,
  pub info: I,
}

pub struct Function<I> {
  pub arms: Vec<FunctionArm<I>>,
  pub info: I,
}
