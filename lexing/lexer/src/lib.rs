use diom_tokens::{SpanToken, Token};
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::{complete::char, complete::multispace0},
  combinator::{consumed, eof},
  error::Error,
  multi::separated_list0,
  number::complete::double,
  sequence::preceded,
  IResult, Parser,
};

pub mod chars;
pub mod comments;
pub mod idents;
pub mod keywords;
pub mod operators;
pub mod parentheses;
pub mod punctuation;
pub mod structure;

#[cfg(test)]
mod tests;

use chars::enclosed_char;
use comments::parse_comment;
use idents::parse_ident;

use crate::chars::parse_span_string;

/// @note parses everything but strings, as they parse to a vector of `Token`s
pub fn parse_token<'a>(input: &'a str) -> IResult<&'a str, Token, Error<&'a str>> {
  alt((
    // Brackets
    alt((
      char('(').map(|_| Token::LParen),
      char(')').map(|_| Token::RParen),
      char('{').map(|_| Token::LCurly),
      char('}').map(|_| Token::RCurly),
      char('[').map(|_| Token::LBrace),
      char(']').map(|_| Token::RBrace),
    )),
    // Reserved keywords
    alt((
      tag("let").map(|_| Token::Let),
      tag("return").map(|_| Token::Return),
    )),
    // Functions
    tag("=>").map(|_| Token::Function),
    // Operators
    alt((
      tag("!=").map(|_| Token::Ne),
      tag("==").map(|_| Token::Eq),
      tag("<=").map(|_| Token::LtEq),
      tag(">=").map(|_| Token::GtEq),
      char('<').map(|_| Token::Lt),
      char('>').map(|_| Token::Gt),
      char('!').map(|_| Token::Not),
      char('&').map(|_| Token::And),
      char('|').map(|_| Token::Or),
      char('+').map(|_| Token::Plus),
      char('-').map(|_| Token::Minus),
      char('*').map(|_| Token::Times),
      char('/').map(|_| Token::Divide),
    )),
    // Punctuation
    alt((
      tag("...").map(|_| Token::Ellipses),
      char('.').map(|_| Token::Dot),
      char(';').map(|_| Token::Semi),
      char(':').map(|_| Token::Colon),
      char(',').map(|_| Token::Comma),
      char('=').map(|_| Token::Assign),
    )),
    // String-like
    alt((
      enclosed_char().map(Token::Char),
      parse_comment().map(Box::from).map(Token::Comment),
      parse_ident().map(Box::from).map(Token::StringIdent),
    )),
    // Value-like
    double.map(Token::Float),
  ))
  .parse(input)
}

fn span_wrap<'a>(
  parser: impl Parser<&'a str, Output = Token, Error = Error<&'a str>>,
) -> impl Parser<&'a str, Output = SpanToken<'a>, Error = Error<&'a str>> {
  consumed(parser).map(|(origin, token)| SpanToken { token, origin })
}

pub fn parse_tokens(input: &str) -> IResult<&str, Vec<SpanToken<'_>>> {
  let parse_item = alt((
    span_wrap(parse_token).map(|tok| vec![tok]),
    parse_span_string,
  ));
  preceded(multispace0, separated_list0(multispace0, parse_item))
    .map(|itemss| itemss.into_iter().flatten().collect())
    .parse(input)
}
