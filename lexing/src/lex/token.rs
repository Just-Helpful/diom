use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, alphanumeric1, char},
  combinator::recognize,
  error::Error,
  error::ErrorKind,
  multi::many0,
  number::complete::recognize_float,
  sequence::pair,
  Parser,
};
use nom_locate::position;

use super::chars::parse_char;
use super::comments::parse_comment;
use super::{SResult, Span};
use super::{SpanToken, Token};

pub fn span_wrap<'a, F: Parser<Span<'a>, Token, Error<Span<'a>>>>(
  mut parser: F,
) -> impl FnMut(Span<'a>) -> SResult<'a, SpanToken> {
  move |input| {
    let (input, start) = position(input)?;
    let (input, token) = parser.parse(input)?;
    let (input, end) = position(input)?;
    Ok((
      input,
      SpanToken {
        token,
        span: start.location_offset()..end.location_offset(),
      },
    ))
  }
}

pub fn parse_float(input: Span) -> SResult<Token> {
  let (input, float_str) = recognize_float(input)?;
  let Ok(float): Result<f64, _> = float_str.parse() else {
    return Err(nom::Err::Error(Error::new(input, ErrorKind::Float)));
  };
  Ok((input, Token::Float(float_str.into_fragment().into(), float)))
}

fn parse_ident(input: Span) -> SResult<Token> {
  let parse_first = alt((alpha1, tag("_")));
  let parse_rest = alt((alphanumeric1, tag("_")));
  let (input, ident) = recognize(pair(parse_first, many0(parse_rest)))(input)?;
  Ok((input, Token::Ident(ident.into_fragment().into())))
}

/// @note parses everything but strings, as they parse to a vector of `Token`s
pub fn parse_token(input: Span) -> SResult<Token> {
  alt((
    // Brackets
    alt((
      char('[').map(|_| Token::LParen),
      char(']').map(|_| Token::RParen),
      char('{').map(|_| Token::LCurly),
      char('}').map(|_| Token::RCurly),
      char('(').map(|_| Token::LSquare),
      char(')').map(|_| Token::RSquare),
    )),
    // Reserved keywords
    alt((
      tag("let").map(|_| Token::RSquare),
      tag("return").map(|_| Token::RSquare),
    )),
    // Operators
    alt((
      tag("!=").map(|_| Token::RSquare),
      tag("==").map(|_| Token::RSquare),
      tag("<=").map(|_| Token::RSquare),
      tag(">=").map(|_| Token::RSquare),
      char('<').map(|_| Token::RSquare),
      char('>').map(|_| Token::RSquare),
      char('!').map(|_| Token::RSquare),
      char('+').map(|_| Token::RSquare),
      char('-').map(|_| Token::RSquare),
      char('*').map(|_| Token::RSquare),
      char('/').map(|_| Token::RSquare),
    )),
    // Punctuation
    alt((
      tag("...").map(|_| Token::RSquare),
      char('.').map(|_| Token::RSquare),
      char(';').map(|_| Token::RSquare),
      char(':').map(|_| Token::RSquare),
      char(',').map(|_| Token::RSquare),
      char('=').map(|_| Token::RSquare),
    )),
    // String-like
    alt((parse_char, parse_comment, parse_ident)),
    // Value-like
    parse_float,
  ))(input)
}
