use crate::{
  errors::{PResult, SyntaxError},
  parsers::matches,
  In,
};
use diom_syntax::expressions::Char;
use diom_tokens::Token;
use nom::{combinator::consumed, Parser};

pub fn parse_char<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Char<In<'a>>, E> {
  let parser = matches(Token::Char('_'));
  let (input, (info, c)) = consumed(parser).parse(input)?;
  Ok((
    input,
    Char {
      info,
      value: c.token.try_into().expect("we've parsed a `Char` token"),
    },
  ))
}
