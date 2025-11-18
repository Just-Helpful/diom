use crate::{
  Span,
  common::{PResult, SpanTokens, Token},
  ident::parse_ident,
  parsers::opt_tag_group,
};
use diom_syntax::types::Array;
use nom::combinator::{complete, eof};

use super::parse_type;

pub fn parse_array(input: SpanTokens) -> PResult<Array<Span>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_ident, Token::LBrace, Token::RBrace)(input)?;
  let (inner, item) = complete(parse_type)(inner)?;
  eof(inner)?;

  Ok((
    input,
    Array {
      name,
      item: Box::new(item),
      info: span,
    },
  ))
}
