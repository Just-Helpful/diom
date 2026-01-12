use super::Expression;
use crate::fmt::{bracket, OptionsDisplay};
use crate::{patterns::Pattern, types::Type};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Argument<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub info: I,
}

impl OptionsDisplay for Argument<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("argument", self.info.len()),
    );
    self.pattern.optn_fmt(w, depth + 1)?;
    if let Some(ty) = &self.annotation {
      ty.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct FunctionArm<I> {
  pub arguments: Vec<Argument<I>>,
  pub annotation: Option<Type<I>>,
  pub returned: Box<Expression<I>>,
  pub info: I,
}

impl OptionsDisplay for FunctionArm<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("arm", self.info.len()));
    for arg in &self.arguments {
      arg.optn_fmt(w, depth + 1)?;
    }
    if let Some(ty) = &self.annotation {
      ty.optn_fmt(w, depth + 1)?;
    }
    self.returned.optn_fmt(w, depth + 1)
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Function<I> {
  pub arms: Vec<FunctionArm<I>>,
  pub info: I,
}

impl OptionsDisplay for Function<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("function", self.info.len()),
    );
    for arm in &self.arms {
      arm.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
