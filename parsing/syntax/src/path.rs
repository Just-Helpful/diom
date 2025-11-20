use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}
