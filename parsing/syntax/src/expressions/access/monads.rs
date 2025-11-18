use super::Expression;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

/// The syntax for unwrapping a monad inline, looks like `?`.
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
/// ```ignore
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
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct MonadThen<I> {
    pub value: Box<Expression<I>>,
    pub info: I,
}

/// The syntax for wrapping a value in a monad, looks like `!`.
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
/// ```ignore
/// let optn_x: Option<Float> = {
///   5!
/// };
/// assert optn_x == Some [5];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct MonadResult<I> {
    pub value: Box<Expression<I>>,
    pub info: I,
}
