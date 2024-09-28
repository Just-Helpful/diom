use crate::ident::Ident;
use diom_info::{InfoMap, InfoSource};

#[derive(Clone, InfoSource, InfoMap)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}
