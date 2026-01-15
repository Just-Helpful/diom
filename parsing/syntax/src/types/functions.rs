use super::Type;
use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Write, ops::Range};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Argument<I> {
  pub name: Ident<I>,
  pub annotation: Type<I>,
  pub info: I,
}

impl DisplayAs<Spans> for Argument<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("argument", &self.info)?;
    self.name.write(&mut w.child())?;
    self.annotation.write(&mut w.child())
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

impl DisplayAs<Spans> for Function<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("function", &self.info)?;
    self.arguments.write(&mut w.child())?;
    self.returned.write(&mut w.child())
  }
}
