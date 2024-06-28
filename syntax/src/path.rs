use crate::{ident::Ident, InfoSource};

#[derive(InfoSource)]
pub struct Path<I> {
  pub names: Vec<Ident<I>>,
  pub info: I,
}
