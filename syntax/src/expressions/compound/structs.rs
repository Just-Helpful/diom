use std::collections::HashMap;

use crate::ident::Ident;

use super::Expression;

pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: HashMap<Ident<I>, Expression<I>>,
  pub info: I,
}
