use std::collections::HashMap;

use crate::ident::Ident;

use super::Value;

pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: HashMap<Ident<I>, Value<I>>,
  pub info: I,
}
