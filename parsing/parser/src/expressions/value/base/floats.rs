use crate::{errors::PResult, parsers::token, Span};
use diom_syntax::expressions::Float;
use diom_tokens::{SpanTokens, Token};

pub fn parse_float(input: SpanTokens) -> PResult<Float<Span>> {
  let (input, v) = token(&Token::Float(0.0))(input)?;
  Ok((
    input,
    Float {
      info: v.span.clone(),
      value: v.token.try_into().unwrap(),
    },
  ))
}
