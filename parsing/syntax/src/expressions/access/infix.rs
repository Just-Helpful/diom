//! Infix operators
//!
//! ## Warning
//!
//! These are only used during parsing!<br>
//! They will be translated into field calls.
use super::Expression;
use crate::{
  fmt::{CustomDisplay, SpanWriter},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Debug, ops::Range};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Infix<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub other: Box<Expression<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Infix<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("infix", &self.info)?;
    self.value.write(&mut w.child())?;
    self.name.write(&mut w.child())?;
    self.other.write(&mut w.child())
  }
}
