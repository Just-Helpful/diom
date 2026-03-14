use diom_tokens::{SpanToken, Token};
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::{complete::char, complete::multispace0},
  combinator::consumed,
  multi::separated_list0,
  number::complete::double,
  sequence::preceded,
  Parser,
};

pub mod chars;
pub mod comments;
pub mod errors;
pub mod idents;
pub mod keywords;
pub mod operators;
pub mod parentheses;
pub mod punctuation;
pub mod structure;

#[cfg(test)]
mod tests;

use crate::errors::SyntaxError;
use chars::{enclosed_char, parse_span_string};
use comments::parse_comment;
use idents::parse_ident;

type In<'a> = &'a str;

/// @note parses everything but strings, as they parse to a vector of `Token`s
pub fn parse_token<'a, E: SyntaxError<'a>>() -> impl Parser<In<'a>, Output = Token, Error = E> {
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
      tag("type").map(|_| Token::Type),
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
      char('?').map(|_| Token::Monad),
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
}

fn span_wrap<'a, E: SyntaxError<'a>>(
  parser: impl Parser<In<'a>, Output = Token, Error = E>,
) -> impl Parser<In<'a>, Output = SpanToken<'a>, Error = E> {
  consumed(parser).map(|(origin, token)| SpanToken { token, origin })
}

pub fn parse_tokens<'a, E: SyntaxError<'a>>(
) -> impl Parser<In<'a>, Output = Vec<SpanToken<'a>>, Error = E> {
  let parse_item = alt((
    span_wrap(parse_token()).map(|tok| vec![tok]),
    parse_span_string(),
  ));
  preceded(multispace0, separated_list0(multispace0, parse_item))
    .map(|itemss| itemss.into_iter().flatten().collect())
}
