//! Infix operators
//!
//! ## Warning
//!
//! These are only used during parsing!<br>
//! They will be translated into field calls.
use super::Expression;
use crate::{
  fmt::{bracket, MultiDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Infix<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub other: Box<Expression<I>>,
  pub info: I,
}

impl MultiDisplay for Infix<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("infix", self.info.len()));
    self.value.multi_fmt(w, depth + 1)?;
    self.name.multi_fmt(w, depth + 1)?;
    self.other.multi_fmt(w, depth + 1)?;
    Ok(())
  }
}
