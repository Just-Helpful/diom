use super::Expression;

pub struct Return<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}
