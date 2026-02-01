use crate::{
  errors::{PResult, SyntaxError},
  parsers::matches,
  In,
};
use diom_syntax::expressions::Float;
use diom_tokens::Token;
use nom::{combinator::consumed, Parser};

pub fn parse_float<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Float<In<'a>>, E> {
  let parser = matches(Token::Float(0.0));
  let (input, (info, v)) = consumed(parser).parse(input)?;
  Ok((
    input,
    Float {
      info,
      value: v.token.try_into().unwrap(),
    },
  ))
}
