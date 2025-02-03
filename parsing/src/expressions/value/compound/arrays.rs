use diom_syntax::expressions::Array;
use diom_tokens::{SpanTokens, Token};
use nom::{combinator::eof, multi::separated_list0};

use crate::{
  errors::PResult,
  expressions::parse_expression,
  parsers::{group, token},
  Span,
};

pub fn parse_array(input: SpanTokens) -> PResult<Array<Span>> {
  let (input, (inner, span)) = group(Token::LBrace, Token::RBrace)(input)?;
  let (inner, contents) = separated_list0(token(Token::Comma), parse_expression)(inner)?;
  eof(inner)?;
  Ok((
    input,
    Array {
      contents,
      info: span,
    },
  ))
}
