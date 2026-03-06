use crate::common::{PResult, SpanToken, Token};
use crate::errors::SyntaxError;
use crate::{In, Item};
use nom::combinator::{eof, verify};
use nom::error::{context, ParseError};
use nom::{
  combinator::opt,
  error::{Error, ErrorKind},
};
use nom::{Input, Parser};
use std::num::NonZero;

#[inline]
pub fn single_item<'a, E: SyntaxError<'a>>() -> impl Parser<In<'a>, Output = Item<'a>, Error = E> {
  move |input: In<'a>| match input.first() {
    Some(t) => Ok((input.take_from(1usize), t.clone())),
    None => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof))),
  }
}

pub fn token<'a, E: SyntaxError<'a>>(
  tok: impl ExactMatch<Token>,
) -> impl Parser<In<'a>, Output = Item<'a>, Error = E> {
  verify(single_item(), move |item| tok.exact(item))
}

pub trait ExactMatch<T> {
  fn exact(&self, other: &T) -> bool;
}
impl ExactMatch<Token> for Token {
  #[inline]
  fn exact(&self, other: &Token) -> bool {
    self == other
  }
}
impl<const N: usize> ExactMatch<Token> for [Token; N] {
  #[inline]
  fn exact(&self, other: &Token) -> bool {
    self.into_iter().any(|tok| tok == other)
  }
}

pub fn matches<'a, E: SyntaxError<'a>>(
  tok: impl ApproxMatch<Token>,
) -> impl Parser<In<'a>, Output = Item<'a>, Error = E> {
  verify(single_item(), move |item| tok.approx(item))
}

pub trait ApproxMatch<T> {
  fn approx(&self, other: &T) -> bool;
}
impl ApproxMatch<Token> for Token {
  #[inline]
  fn approx(&self, other: &Token) -> bool {
    self.matches(other)
  }
}
impl<const N: usize> ApproxMatch<Token> for [Token; N] {
  #[inline]
  fn approx(&self, other: &Token) -> bool {
    self.into_iter().any(|tok| tok.matches(other))
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
  mut parser: impl Parser<I, Output = R, Error = E>,
) -> impl Parser<I, Output = Vec<R>, Error = E>
where
  I: Input<Item = SpanToken<'a>> + Clone,
{
  move |mut input: I| {
    let mut result = vec![];

    while let Ok((tail, init)) = input.split_at_position::<_, Error<_>>(|t| tok.matches(t)) {
      let (init, value) = opt(|input| parser.parse(input)).parse(init)?;
      let Some(value) = value else { break };
      eof(init)?;

      result.push(value);
      input = tail.take_from(1usize); // skip split token
    }

    // allow for trailing separator
    if input.input_len() == 0 {
      return Ok((input, result));
    }

    let (input, value) = parser.parse(input)?;
    result.push(value);
    Ok((input, result))
  }
}
