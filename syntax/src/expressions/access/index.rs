use super::Expression;

pub struct Index<I> {
  pub value: Box<Expression<I>>,
  pub key: Vec<Expression<I>>,
  pub info: I,
}
