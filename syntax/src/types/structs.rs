use crate::ident::Ident;

use super::Type;

pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<(Ident<I>, Type<I>)>,
  pub info: I,
}
