use crate::{
  common::{PResult, SpanTokens, Token},
  errors::SyntaxError,
  ident::parse_ident,
  parsers::matches,
  In,
};
use diom_syntax::path::Path;
use nom::{
  combinator::{consumed, opt},
  multi::separated_list1,
  Parser,
};

pub fn parse_path<'a, E: SyntaxError<'a>>(input: SpanTokens<'a>) -> PResult<'a, Path<In<'a>>, E> {
  let (input, (info, segments)) =
    consumed(separated_list1(opt(matches(Token::Dot)), parse_ident)).parse(input)?;

  Ok((input, Path { segments, info }))
}
