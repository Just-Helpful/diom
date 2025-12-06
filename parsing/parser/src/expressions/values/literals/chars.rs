use crate::{errors::PResult, parsers::token, Span};
use diom_syntax::expressions::Char;
use diom_tokens::{SpanTokens, Token};

pub fn parse_char(input: SpanTokens) -> PResult<Char<Span>> {
  let (input, c) = token(&Token::Char('_'))(input)?;
  Ok((
    input,
    Char {
      info: c.span.clone(),
      value: c.token.try_into().expect("we've parsed a `Char` token"),
    },
  ))
}
