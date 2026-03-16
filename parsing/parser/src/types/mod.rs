//! # Diom type parsing
//!
//! Types in Diom are defined via 'uninitialised variable' syntax,
//! i.e. `let uint32: u32` defines `uint32` as a subtype of `u32`.
//!
//! I'm planning to support:
//!
//! 1. Named/Unnamed tuples
//! 2. Named/Unnamed structs
//! 3. Named/Unnamed enums
//! 4. Named/Unnamed functions
//! 5. Unique types
//! 6. Subtypes
//!
//! See the individual modules for more details
use crate::{
  errors::{PResult, SyntaxError},
  ident::parse_ident,
  In,
};
use diom_syntax::types::Type;
use nom::{branch::alt, error::context, Parser};

mod arrays;
use arrays::parse_array;
mod enums;
use enums::parse_enum;
mod functions;
use functions::parse_function;
mod structs;
use structs::parse_struct;
mod tuples;
use tuples::parse_tuple;
mod typedef;
pub use typedef::*;
mod tags;
pub use tags::*;

/// Parses a type in the Diom language.
///
/// Types are primarily used within:
/// 1. variable declerations
/// 2. function parameters
pub fn parse_type<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Type<In<'a>>, E> {
  alt((
    context("function type", parse_function).map(Type::Function),
    context("array type", parse_array).map(Type::Array),
    context("enum type", parse_enum).map(Type::Enum),
    context("struct type", parse_struct).map(Type::Struct),
    context("tuple type", parse_tuple).map(Type::Tuple),
    context("type var", parse_ident).map(Type::Var),
  ))
  .parse(input)
}
