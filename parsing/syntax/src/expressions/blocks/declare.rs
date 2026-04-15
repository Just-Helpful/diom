use super::Expression;
use crate::{
  patterns::{Pattern, PatternConfig},
  types::{Type, TypeConfig},
  Ptr,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{option, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// Decleration should allow for pattern matching in its syntax
///
/// ```_
/// let Vec2D { x: Float, y: Float };
/// let Vec2D {x, y} = vec0;
/// ```
///
/// In the case where it's possible for the pattern to not match during runtime
/// (most commonly for `enum`-like values), the `let` statement returns a
/// `Boolean` value (`True` if the `let` statement matches, `False` otherwise).
/// For example:
///
/// ```_
/// assert (let Some(x) = Some(5)) == True
/// assert (let Some(x) = None) == False
/// ```
///
/// If the type checker can prove that this value will always be `True`,
/// then it will allow the return value to remain unused, otherwise if the
/// return value is not used, it'll throw an compiler error.
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Declare<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub value: Ptr<Expression<I>>,
  pub info: I,
}

impl<I> Display for Declare<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("let ")?;
    self.pattern.fmt(f)?;
    if let Some(annotation) = &self.annotation {
      f.write_char(':')?;
      annotation.fmt(f)?
    }
    f.write_char('=')?;
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for Declare<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("declare", &self.info)?;
    self.pattern.write(&mut w.child())?;
    self.annotation.write(&mut w.child())?;
    self.value.write(&mut w.child())
  }
}

#[derive(Default, Clone, Copy)]
pub struct DeclareConfig(
  /// The config used to generate patterns
  pub PatternConfig,
  /// The config used to generate types
  pub TypeConfig,
);
impl Declare<()> {
  /// Generates a generic strategy for generating `Call` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: DeclareConfig,
  ) -> impl Strategy<Value = Self> {
    (Pattern::any(args.0), option::of(Type::any(args.1)), item).prop_map(
      |(pattern, annotation, value)| Declare {
        pattern,
        annotation,
        value: Ptr::new(value),
        info: (),
      },
    )
  }
}
