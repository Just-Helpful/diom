pub use diom_info_derive::{InfoMap, InfoRef, InfoSource};

pub mod boxes;
pub mod options;
pub mod tuples;
pub mod vecs;

/// A type that has information attached to it
pub trait InfoSource {
  /// The information type attached to `Self`
  type Info;
}

/// Gets a reference to the underlying information of a source
pub trait InfoRef: InfoSource {
  /// A getter for the information attached to `self`
  fn info(&self) -> &Self::Info;
}

/// Modifies the underlying information for a source.<br>
/// This map can change the type of `self` and hence needs to consume.
pub trait InfoMap: InfoSource {
  /// A generic version of `Self`.<br>
  /// This should fulfill `GenericSelf<T> == Self` for some `T`.
  type GenericSelf<T>;

  /// Modifies the information attached to `self`
  fn map<R>(self, f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R>;
}
