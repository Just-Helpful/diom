use std::ops::Range;

use crate::{
  fmt::{bracket, MultiDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use super::Type;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Argument<I> {
  pub name: Ident<I>,
  pub annotation: Type<I>,
  pub info: I,
}

impl MultiDisplay for Argument<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("argument", self.info.len()),
    );
    self.name.multi_fmt(w, depth + 1)?;
    self.annotation.multi_fmt(w, depth + 1)
  }
}

/// The type for a callable function
///
/// ```ignore
/// # function types can be simplified a bit
/// type Binary = (x: Float): (y: Float): Float;
/// type Binary = (x: Float)(y: Float): Float;
///
/// let add: Binary = (x)(y) => x + y;
/// let add: Binary = (x) => {(y) => {x + y}};
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Function<I> {
  pub arguments: Vec<Argument<I>>,
  pub returned: Box<Type<I>>,
  pub info: I,
}

impl MultiDisplay for Function<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("function", self.info.len()),
    );
    for arg in &self.arguments {
      arg.multi_fmt(w, depth + 1)?;
    }
    self.returned.multi_fmt(w, depth + 1)
  }
}
