use std::ops::Range;

use diom_lexing::{
  token::{SpanToken, Token},
  tokens::SpanTokens,
};
use diom_syntax::ident::Ident;

use crate::{parsers::token, PResult};

pub fn parse_ident(input: SpanTokens) -> PResult<Ident<Range<usize>>> {
  let (input, tok) = token(Token::Ident("".into()))(input)?;
  let SpanToken {
    token: Token::Ident(name),
    span,
  } = tok
  else {
    panic!("Parsing an `Ident` token should return an `Ident`")
  };
  Ok((input, Ident { name, info: span }))
}
