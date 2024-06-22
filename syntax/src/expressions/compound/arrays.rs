use super::Expression;

pub struct Array<I> {
  pub contents: Vec<Expression<I>>,
  pub info: I,
}
