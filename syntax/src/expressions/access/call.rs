use super::Expression;

pub struct Call<I> {
  pub function: Box<Expression<I>>,
  pub arguments: Vec<Expression<I>>,
  pub info: I,
}
