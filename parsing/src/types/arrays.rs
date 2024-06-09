use std::ops::Range;

use diom_lexing::{token::Token, tokens::SpanTokens};
use diom_syntax::types::Array;
use nom::combinator::opt;

use crate::{ident::parse_ident, parsers::token, PResult};

use super::parse_type;

pub fn parse_array(input: SpanTokens) -> PResult<Array<Range<usize>>> {
  let (input, name) = opt(parse_ident)(input)?;
  let (input, brac) = token(Token::LSquare)(input)?;
  let mut start = brac.span.start;
  if let Some(ref ident) = name {
    start = ident.info.start;
  }

  let (input, item) = parse_type(input)?;

  let (input, brac) = token(Token::RSquare)(input)?;
  let end = brac.span.end;

  Ok((
    input,
    Array {
      item: Box::new(item),
      info: start..end,
    },
  ))
}
