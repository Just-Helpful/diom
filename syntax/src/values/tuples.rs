use crate::ident::Ident;

use super::Value;

pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Value<I>>,
  pub info: I,
}
