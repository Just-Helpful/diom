use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use crate::{patterns::Pattern, types::Type};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Argument<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub info: I,
}

impl MultiDisplay for Argument<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("argument", self.info.len()),
    );
    self.pattern.multi_fmt(w, depth + 1)?;
    if let Some(ty) = &self.annotation {
      ty.multi_fmt(w, depth + 1)?;
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

impl MultiDisplay for FunctionArm<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("arm", self.info.len()));
    for arg in &self.arguments {
      arg.multi_fmt(w, depth + 1)?;
    }
    if let Some(ty) = &self.annotation {
      ty.multi_fmt(w, depth + 1)?;
    }
    self.returned.multi_fmt(w, depth + 1)
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Function<I> {
  pub arms: Vec<FunctionArm<I>>,
  pub info: I,
}

impl MultiDisplay for Function<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("function", self.info.len()),
    );
    for arm in &self.arms {
      arm.multi_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
