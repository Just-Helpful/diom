//! Infix operators
//!
//! ## Warning
//!
//! These are only used during parsing!<br>
//! They will be translated into field calls.
use super::Expression;
use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Debug, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Infix<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub other: Box<Expression<I>>,
  pub info: I,
}

impl DisplayAs<Spans> for Infix<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("infix", &self.info)?;
    self.value.write(&mut w.child())?;
    self.name.write(&mut w.child())?;
    self.other.write(&mut w.child())
  }
}
