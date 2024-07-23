use super::Expression;
use crate::InfoSource;

/// The syntax for applying a monad inline, looks like `?`.
///
/// For the `Monad` trait defined as so:
/// ```ignore
/// let Monad<T> {
///   then<R>(self)(f: (v: T): Monad<R>): Monad<R>,
///   result(v: T): Self,
/// };
/// ```
///
/// and an `Option` type defined as so:
/// ```ignore
/// let Option<T> {
///   Some(T),
///   None,
/// };
/// use Option.*;
/// let Option<T>.Monad<T> {
///   then: {
///     (Some(x))(f) = f(x),
///     (None)(_) = None,
///   },
///   result: Some,
/// };
/// ```
///
/// `?` can be used as follows:
///
/// ```ignore
/// let optn_x: Option<Float> = {
///   let x: Float = Some(5)?;
///   Monad.result(x + 1);
/// };
/// assert optn_x == Some(6);
///
/// let optn_x: Option<Float> = {
///   let x: Float = None?;
///   Monad.result(x + 1);
/// };
/// assert optn_x == None;
/// ```
#[derive(InfoSource, Clone)]
pub struct MonadThen<I> {
  pub value: Box<Expression<I>>,
  info: I,
}
