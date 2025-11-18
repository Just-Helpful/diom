use crate::{Span, errors::PResult, ident::parse_ident, parsers::token};
use diom_info_traits::InfoRef;
use diom_syntax::patterns::rest::Rest;
use diom_tokens::{SpanTokens, Token};
use nom::combinator::opt;

pub fn parse_rest(input: SpanTokens) -> PResult<Rest<Span>> {
  let (input, mut rest) = token(Token::Ellipses)(input)?;
  let (input, name) = opt(parse_ident)(input)?;
  if let Some(name) = &name {
    rest.span.end = name.info().end;
  }
  Ok((
    input,
    Rest {
      name,
      info: rest.span,
    },
  ))
}
