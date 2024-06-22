use super::Expression;

pub struct Block<I> {
  pub content: Vec<Expression<I>>,
  pub info: I,
}
