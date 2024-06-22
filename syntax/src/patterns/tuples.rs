use super::{Pattern, Rest};

pub enum TupleField<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

pub struct Tuple<I> {
  pub fields: Vec<TupleField<I>>,
  pub info: I,
}
