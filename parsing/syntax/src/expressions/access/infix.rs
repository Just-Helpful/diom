//! Infix operators
//!
//! ## Warning
//!
//! These are only used during parsing!<br>
//! They will be translated into field calls.
use super::Expression;
use crate::idents::Method;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Debug, Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Infix<I> {
  pub value: Box<Expression<I>>,
  pub name: Method<I>,
  pub other: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for Infix<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char(' ')?;
    self.name.fmt(f)?;
    f.write_char(' ')?;
    self.other.fmt(f)
  }
}

impl DisplayAs<Spans> for Infix<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("infix", &self.info)?;
    self.value.write(&mut w.child())?;
    self.name.write(&mut w.child())?;
    self.other.write(&mut w.child())
  }
}

impl Infix<()> {
  /// Generates a generic strategy for generating `Infix` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>> + Clone) -> impl Strategy<Value = Self> {
    (item.clone(), Method::any(), item).prop_map(|(value, name, other)| Infix {
      value: Box::new(value),
      name,
      other: Box::new(other),
      info: (),
    })
  }
}
