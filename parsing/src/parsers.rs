use diom_lexing::{
  token::{SpanToken, Token},
  tokens::SpanTokens,
};
use nom::{
  error::{Error, ErrorKind},
  InputIter, Slice,
};

use crate::PResult;

pub fn token(tok: Token) -> impl Fn(SpanTokens) -> PResult<SpanToken> {
  let tok = tok.into();
  move |input| match input.iter_elements().next() {
    Some(t) if t == &tok => Ok((input.slice(1..), t.clone())),
    _ => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
  }
}

pub fn token_of(
  toks: impl IntoIterator<Item = Token>,
) -> impl Fn(SpanTokens) -> PResult<SpanToken> {
  let toks: Vec<SpanToken> = toks.into_iter().map(SpanToken::from).collect();
  move |input| match input.iter_elements().next() {
    Some(t) if toks.iter().any(|tok| t == tok) => Ok((input.slice(1..), t.clone())),
    _ => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
  }
}
