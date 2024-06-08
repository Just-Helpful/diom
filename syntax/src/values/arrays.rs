use super::Value;

pub struct Array<I> {
  pub contents: Vec<Value<I>>,
  pub info: I,
}
