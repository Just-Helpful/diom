use std::fmt::Debug;
use std::ops::Deref;

/// A wrapper type that provides definitions for:
/// - `expression` layers
/// - `pattern` layers
/// - `type` layers
///
/// - `ident`ifiers and `path`s
pub trait SyntaxScope: Debug + Clone {
  /// The type for references to single nodes
  type Single<T: Debug + Clone>: Debug + Clone + Deref<Target = T>;

  type Multi<T: Debug + Clone>: Debug + Clone + (for<'a> IterRef<'a, T>) + (for<'a> IterMut<'a, T>);
}

/// Allows iterator conversion from `&Self`\
/// That provides ref access to iterator items.\
/// *For example:* `&Vec<T>: IntoIterator<Item = &T>`
pub trait IterRef<'a, T: 'a>: 'a
where
  &'a Self: IntoIterator<Item = &'a T> + 'a,
{
}
impl<'a, T: 'a, I: 'a> IterRef<'a, T> for I where &'a I: IntoIterator<Item = &'a T> {}

/// Allows iterator conversion from `&mut Self`\
/// That provides mut access to iterator items.\
/// *For example:* `&mut Vec<T>: IntoIterator<Item = &mut T>`
pub trait IterMut<'a, T: 'a>: 'a
where
  &'a mut Self: IntoIterator<Item = &'a mut T> + 'a,
{
}
impl<'a, T: 'a, I: 'a> IterMut<'a, T> for I where &'a mut I: IntoIterator<Item = &'a mut T> {}
