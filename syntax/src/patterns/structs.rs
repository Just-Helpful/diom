use crate::ident::Ident;

use super::{Pattern, Rest};

pub enum StructField<I> {
  Field(Ident<I>, Pattern<I>),
  Rest(Rest<I>),
}

pub struct Struct<I> {
  pub fields: Vec<StructField<I>>,
}
