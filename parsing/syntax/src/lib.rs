//! # AST nodes for Diom
//!
//! The nodes here fall into 2 broad categories:
//! 1. nodes for Diom typing
//! 2. nodes for Diom values

use std::rc::Rc;
mod display;
pub mod expressions;
pub mod idents;
pub mod path;
pub mod patterns;
pub mod types;
pub mod var;

/// The ptr type used for indirection in syntax nodes
pub type Ptr<T> = Rc<T>;
/// The owned sequence used for indirection in syntax nodes
pub type Seq<T> = Vec<T>;

/// Creates a syntax node pointer from a `Box`
pub fn from_box<T: ?Sized>(value: Box<T>) -> Ptr<T> {
  value.into()
}
/// Creates a `Box` from a syntax node pointer
/// @todo maybe use a `try_into` here instead of panicking?
pub fn into_box<T: ?Sized>(mut ptr: Ptr<T>) -> Box<T> {
  let inner_mut = Rc::get_mut(&mut ptr).expect("No other borrows occur");
  // Safety:
  // after `into_box`, `inner_mut` is dropped and cannot be re-used
  // `Ptr` is allocated from the `Global` allocator
  unsafe { Box::from_raw(inner_mut) }
}
