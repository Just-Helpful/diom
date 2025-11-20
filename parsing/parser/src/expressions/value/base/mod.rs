use diom_syntax::expressions::Expression;
use diom_tokens::SpanTokens;

mod chars;
use chars::parse_char;
mod floats;
use floats::parse_float;
use nom::{branch::alt, Parser};

use crate::{common::PResult, ident::parse_ident, Span};

pub fn parse_base(input: SpanTokens) -> PResult<Expression<Span>> {
  alt((
    parse_char.map(Expression::Char),
    parse_float.map(Expression::Float),
    parse_ident.map(Expression::Var),
  ))(input)
}
