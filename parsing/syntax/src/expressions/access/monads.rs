use super::Expression;
use crate::Ptr;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// The syntax for unwrapping a monad inline, looks like `?`.
///
/// For the `Monad` trait defined as so:
/// ```_
/// let Monad<T> {
///   then<R>(self)(f: (v: T): Monad<R>): Monad<R>,
///   result(v: T): Self,
/// };
/// ```
///
/// and an `Option` type defined as so:
/// ```_
/// let Option<T>: Some [T] | None;
///
/// use Option.*;
/// let Option<T>.Monad<T> {
///   then: {
///     (Some [x])(f) = f(x),
///     (None)(_) = None,
///   },
///   result(x): Some [x],
/// };
/// ```
///
/// `?` can be used as follows:
///
/// ```_
/// let optn_x: Option<Float> = {
///   let x: Float = Some [5]?;
///   Some [x + 1]
/// };
/// assert optn_x == Some [6];
///
/// let optn_x: Option<Float> = {
///   let x: Float = None?;
///   Some [x + 1]
/// };
/// assert optn_x == None;
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct MonadThen<I> {
  pub value: Ptr<Expression<I>>,
  pub info: I,
}

impl<I> Display for MonadThen<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char('?')
  }
}

impl DisplayAs<Spans> for MonadThen<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("monad unwrap", &self.info)?;
    self.value.write(&mut w.child())
  }
}

impl MonadThen<()> {
  /// Generates a generic strategy for generating `MonadThen` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>>) -> impl Strategy<Value = Self> {
    item.prop_map(|value| MonadThen {
      value: Ptr::new(value),
      info: (),
    })
  }
}

/// The syntax for wrapping a value in a monad, looks like `!`.
///
/// For the `Monad` trait defined as so:
/// ```_
/// let Monad<T> {
///   then<R>(self)(f: (v: T): Monad<R>): Monad<R>,
///   result(v: T): Self,
/// };
/// ```
///
/// and an `Option` type defined as so:
/// ```_
/// let Option<T>: Some [T] | None;
///
/// use Option.*;
/// let Option<T>.Monad<T> {
///   then: {
///     (Some [x])(f) = f(x),
///     (None)(_) = None,
///   },
///   result(x): Some [x],
/// };
/// ```
///
/// `!` can be used as follows:
///
/// ```_
/// let optn_x: Option<Float> = {
///   5!
/// };
/// assert optn_x == Some [5];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct MonadResult<I> {
  pub value: Ptr<Expression<I>>,
  pub info: I,
}

impl<I> Display for MonadResult<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char('!')
  }
}

impl DisplayAs<Spans> for MonadResult<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("monad result", &self.info)?;
    self.value.write(&mut w.child())
  }
}

impl MonadResult<()> {
  /// Generates a generic strategy for generating `MonadResult` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>>) -> impl Strategy<Value = Self> {
    item.prop_map(|value| MonadResult {
      value: Ptr::new(value),
      info: (),
    })
  }
}
