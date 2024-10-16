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
use diom_syntax::types::Type;
use diom_tokens::SpanTokens;
use nom::{branch::alt, Parser};

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

use crate::{errors::PResult, ident::parse_ident, Span};

/// Parses a type in the Diom language.
///
/// Types are primarily used within:
/// 1. variable declerations
/// 2. function arguments
pub fn parse_type(input: SpanTokens) -> PResult<Type<Span>> {
  alt((
    parse_function.map(Type::Function),
    parse_array.map(Type::Array),
    parse_enum.map(Type::Enum),
    parse_struct.map(Type::Struct),
    parse_tuple.map(Type::Tuple),
    parse_ident.map(Type::Var),
  ))(input)
}
