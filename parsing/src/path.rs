use crate::{
  common::{PResult, SpanTokens, Token},
  ident::parse_ident,
  parsers::token,
  Span,
};
use diom_info::InfoRef;
use diom_syntax::path::Path;
use nom::{combinator::opt, multi::separated_list1};

pub fn parse_path(input: SpanTokens) -> PResult<Path<Span>> {
  let (input, segments) = separated_list1(opt(token(Token::Dot)), parse_ident)(input)?;
  let start = segments
    .first()
    .expect("`seperated_list1` should produce a Vec with at least 1 element")
    .info()
    .start;
  let end = segments
    .last()
    .expect("`seperated_list1` should produce a Vec with at least 1 element")
    .info()
    .end;

  Ok((
    input,
    Path {
      segments,
      info: start..end,
    },
  ))
}
