//! # AST nodes for Diom
//!
//! The nodes here fall into 2 broad categories:
//! 1. nodes for Diom typing
//! 2. nodes for Diom values
mod display;
pub mod expressions;
pub mod idents;
pub mod path;
pub mod patterns;
pub mod types;
pub mod var;

/// The ptr type used for indirection in syntax nodes
pub type Ptr<T> = Box<T>;
/// The owned slice used for indirection in syntax nodes
pub type Slice<T> = Vec<T>;
