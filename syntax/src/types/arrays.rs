use super::Type;

pub struct Array<I> {
  pub item: Box<Type<I>>,
  pub info: I,
}
