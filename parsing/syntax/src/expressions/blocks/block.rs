use super::Expression;
use crate::types::TypeDef;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Write, ops::Range};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum Statement<I> {
  Expression(Expression<I>),
  TypeDef(TypeDef<I>),
}

impl DisplayAs<Spans> for Statement<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Expression(e) => e.write(w),
      Self::TypeDef(d) => d.write(w),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Block<I> {
  pub statements: Vec<Statement<I>>,
  pub info: I,
}

impl DisplayAs<Spans> for Block<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("block", &self.info)?;
    self.statements.write(&mut w.child())
  }
}
