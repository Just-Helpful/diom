use crate::{ident::Ident, InfoSource};

#[derive(InfoSource, Clone)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}
