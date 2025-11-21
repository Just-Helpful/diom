use super::Expression;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Index<I> {
  pub value: Box<Expression<I>>,
  pub key: Vec<Expression<I>>,
  pub info: I,
}
