use crate::common::{PResult, SpanToken, SpanTokens, Token};
use crate::errors::SyntaxError;
use crate::In;
use nom::combinator::eof;
use nom::error::{context, ParseError};
use nom::{
  combinator::opt,
  error::{Error, ErrorKind},
  Err,
};
use nom::{Input, Parser};
use std::num::NonZero;
use std::ops::Deref;

pub fn token<'a, E: SyntaxError<'a>>(
  tok: Token,
) -> impl Fn(SpanTokens<'a>) -> PResult<SpanToken, E> {
  move |input| match input.first() {
    Some(t) if t.deref() == &tok => Ok((input.take_from(1usize), t.clone())),
    Some(_) => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Tag))),
    None => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof))),
  }
}

pub fn matches<'a, E: ParseError<SpanTokens<'a>>>(
  tok: Token,
) -> impl Fn(SpanTokens<'a>) -> PResult<SpanToken, E> {
  move |input| match input.first() {
    Some(t) if t.matches(tok.as_ref()) => Ok((input.take_from(1usize), t.clone())),
    Some(_) => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Tag))),
    None => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof))),
  }
}

pub fn group<'a, E: SyntaxError<'a>>(
  lbrac: Token,
  rbrac: Token,
) -> impl Fn(In<'a>) -> PResult<In<'a>, E> {
  move |input: In<'a>| {
    let (input, _) = context("group open", matches(lbrac.clone())).parse(input)?;
    let mut scope = 1usize;

    // bracket counting and scope detection
    let idx = input.into_iter().position(|tok| {
      if tok.matches(lbrac.as_ref()) {
        scope += 1;
        return false;
      }
      if tok.matches(rbrac.as_ref()) {
        scope -= 1;
      }
      scope == 0
    });

    // if we are in scope `scope` by the end of the input
    // then we need at least `scope` more closing brackets...
    let Some(i) = idx else {
      return Err(nom::Err::Incomplete(nom::Needed::Size(
        NonZero::new(scope).unwrap(),
      )));
    };

    let (input, inner) = input.take_split(i);
    let (input, _) = context("group close", matches(rbrac.clone())).parse(input)?;
    Ok((input, inner))
  }
}

pub fn token_separated_list<'a, I, R, E: ParseError<I>>(
  tok: Token,
  parser: impl Fn(I) -> Result<(I, R), Err<E>>,
) -> impl Fn(I) -> Result<(I, Vec<R>), Err<E>>
where
  I: Input<Item = SpanToken<'a>> + Clone,
{
  move |mut input| {
    let mut result = vec![];

    while let Ok((tail, init)) = input.split_at_position::<_, Error<_>>(|t| tok.matches(t)) {
      let (init, value) = opt(&parser).parse(init)?;
      let Some(value) = value else { break };
      eof(init)?;

      result.push(value);
      input = tail.take_from(1usize); // skip split token
    }

    // allow for trailing separator
    if input.input_len() == 0 {
      return Ok((input, result));
    }

    let (input, value) = parser(input)?;
    result.push(value);
    Ok((input, result))
  }
}
