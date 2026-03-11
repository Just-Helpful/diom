use super::Expression;
use crate::{display::Sep, patterns::Pattern, types::Type};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Parameter<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub info: I,
}

impl<I> Display for Parameter<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.pattern.fmt(f)?;
    if let Some(annotation) = &self.annotation {
      f.write_char(':')?;
      annotation.fmt(f)?
    }
    Ok(())
  }
}

impl DisplayAs<Spans> for Parameter<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("param", &self.info)?;
    self.pattern.write(&mut w.child())?;
    self.annotation.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Parameters<I> {
  pub parameters: Vec<Parameter<I>>,
  pub info: I,
}

impl<I> Display for Parameters<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('(')?;
    Sep(&self.parameters, ',').fmt(f)?;
    f.write_char(')')
  }
}

impl DisplayAs<Spans> for Parameters<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("params", &self.info)?;
    self.parameters.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct FunctionArm<I> {
  pub parameters: Parameters<I>,
  pub annotation: Option<Type<I>>,
  pub returned: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for FunctionArm<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.parameters.fmt(f)?;
    if let Some(annotation) = &self.annotation {
      f.write_char(':')?;
      annotation.fmt(f)?
    }
    f.write_str("=>")?;
    self.returned.fmt(f)
  }
}

impl DisplayAs<Spans> for FunctionArm<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("arm", &self.info)?;
    self.parameters.write(&mut w.child())?;
    self.annotation.write(&mut w.child())?;
    self.returned.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Function<I> {
  pub arms: Vec<FunctionArm<I>>,
  pub info: I,
}

impl<I> Display for Function<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('{')?;
    Sep(&self.arms, ',').fmt(f)?;
    f.write_char('}')
  }
}

impl DisplayAs<Spans> for Function<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("function", &self.info)?;
    self.arms.write(&mut w.child())
  }
}
