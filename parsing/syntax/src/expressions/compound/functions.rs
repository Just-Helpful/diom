use super::Expression;
use crate::{patterns::Pattern, types::Type};
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Argument<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Argument<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("argument", &self.info)?;
    self.pattern.write(&mut w.child())?;
    self.annotation.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct FunctionArm<I> {
  pub arguments: Vec<Argument<I>>,
  pub annotation: Option<Type<I>>,
  pub returned: Box<Expression<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for FunctionArm<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("arm", &self.info)?;
    self.arguments.write(&mut w.child())?;
    self.annotation.write(&mut w.child())?;
    self.returned.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Function<I> {
  pub arms: Vec<FunctionArm<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Function<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("function", &self.info)?;
    self.arms.write(&mut w.child())
  }
}
