use crate::{
  errors::{PResult, SyntaxError},
  idents::parse_ident,
  In,
};
use diom_syntax::patterns::Pattern;
use nom::{branch::alt, Parser};

pub mod arrays;
use arrays::parse_array;
pub mod ignored;
use ignored::parse_ignored;
pub mod rest;
use rest::parse_rest;
pub mod structs;
use structs::parse_struct;
pub mod tags;
use tags::parse_tagged;
pub mod tuples;
use tuples::parse_tuple;

pub fn parse_pattern<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Pattern<In<'a>>, E> {
  alt((
    parse_array.map(Pattern::Array),
    parse_ignored.map(Pattern::Ignored),
    parse_struct.map(Pattern::Struct),
    parse_tagged.map(Pattern::Tagged),
    parse_tuple.map(Pattern::Tuple),
    parse_ident.map(Pattern::Var),
  ))
  .parse(input)
}
