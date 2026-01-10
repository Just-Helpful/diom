use crate::{
  errors::{PResult, SyntaxError},
  parsers::token,
  In,
};
use diom_syntax::patterns::ignored::Ignored;
use diom_tokens::Token;
use nom::{combinator::recognize, Parser};

pub fn parse_ignored<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Ignored<In<'a>>, E> {
  let (input, info) = recognize(token(Token::StringIdent("_".into()))).parse(input)?;
  Ok((input, Ignored { info }))
}
