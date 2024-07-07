use super::tokens::{SpanToken, Token};
use nom::{
  branch::alt, bytes::complete::tag, character::complete::char, error::Error, multi::many0,
  number::complete::double, IResult, Parser,
};
use nom_locate::LocatedSpan;

mod chars;
use chars::{parse_char, parse_string};
mod comments;
use comments::parse_comment;
mod helpers;
mod idents;
use idents::parse_ident;

type Span<'a> = LocatedSpan<&'a str>;
type SResult<'a, O> = IResult<Span<'a>, O, Error<Span<'a>>>;

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
    double.map(Token::Float),
  ))(input)
}

pub fn parse_tokens(input: Span) -> SResult<Vec<SpanToken>> {
  let parse_item = alt((
    helpers::span_wrap(parse_token).map(|tok| vec![tok]),
    parse_string,
  ));
  many0(parse_item)
    .map(|itemss| itemss.into_iter().flatten().collect())
    .parse(input)
}
