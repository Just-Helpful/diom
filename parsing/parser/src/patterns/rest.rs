use crate::{
  errors::{PResult, SyntaxError},
  ident::parse_ident,
  parsers::matches,
  In,
};
use diom_syntax::patterns::rest::Rest;
use diom_tokens::Token;
use nom::{
  combinator::{consumed, opt},
  sequence::preceded,
  Parser,
};

pub fn parse_rest<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Rest<In<'a>>, E> {
  let parser = preceded(matches(Token::Ellipses), opt(parse_ident));

  let (input, (info, name)) = consumed(parser).parse(input)?;
  Ok((input, Rest { name, info }))
}
