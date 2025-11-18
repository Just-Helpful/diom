use diom_syntax::patterns::Pattern;
use diom_tokens::SpanTokens;
use ignored::parse_ignored;
use nom::{Parser, branch::alt};

pub mod arrays;
use arrays::parse_array;
pub mod ignored;
pub mod rest;
use rest::parse_rest;
pub mod structs;
use structs::parse_struct;
pub mod tuples;
use tuples::parse_tuple;

use crate::{common::Span, errors::PResult, ident::parse_ident};

pub fn parse_pattern(input: SpanTokens) -> PResult<Pattern<Span>> {
  alt((
    parse_array.map(Pattern::Array),
    parse_ignored.map(Pattern::Ignored),
    parse_struct.map(Pattern::Struct),
    parse_tuple.map(Pattern::Tuple),
    parse_ident.map(Pattern::Var),
  ))(input)
}
