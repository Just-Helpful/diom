use crate::{
  Span,
  common::{PResult, SpanTokens, Token},
  ident::parse_ident,
  parsers::{group, token},
};
use diom_info_traits::InfoRef;
use diom_syntax::types::{Argument, Function};
use nom::{combinator::eof, multi::separated_list0, sequence::preceded};

use super::parse_type;

fn parse_argument(input: SpanTokens) -> PResult<Argument<Span>> {
  let (input, name) = parse_ident(input)?;
  let (input, annotation) = parse_type(input)?;
  Ok((
    input,
    Argument {
      info: name.info().start..annotation.info().end,
      name,
      annotation,
    },
  ))
}

pub fn parse_function(input: SpanTokens) -> PResult<Function<Span>> {
  let (input, (inner, mut span)) = group(Token::LParen, Token::RParen)(input)?;
  let (inner, arguments) = separated_list0(token(Token::Comma), parse_argument)(inner)?;
  eof(inner)?;

  let (input, returned) = preceded(token(Token::Colon), parse_type)(input)?;
  span.end = returned.info().end;

  Ok((
    input,
    Function {
      arguments,
      returned: Box::new(returned),
      info: span,
    },
  ))
}
