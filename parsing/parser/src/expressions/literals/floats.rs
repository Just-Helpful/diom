use crate::{errors::PResult, parsers::matches, In};
use diom_syntax::expressions::Float;
use diom_tokens::Token;
use nom::{combinator::consumed, error::ParseError, Parser};

pub fn parse_float<'a, E: ParseError<In<'a>>>(input: In<'a>) -> PResult<'a, Float<In<'a>>, E> {
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
