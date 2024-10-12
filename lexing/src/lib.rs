use diom_tokens::{SpanToken, Token};
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{char, multispace0},
  error::Error,
  multi::many0,
  number::complete::double,
  sequence::delimited,
  Finish, IResult, Parser,
};
use nom_locate::LocatedSpan;
use std::{ops::Deref, str::FromStr};

pub mod chars;
use chars::{parse_char, parse_string};
pub mod comments;
use comments::parse_comment;
mod helpers;
use helpers::span_wrap;
pub mod idents;
use idents::parse_ident;
#[cfg(test)]
mod tests;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type SResult<'a, O> = IResult<Span<'a>, O, Error<Span<'a>>>;

/// @note parses everything but strings, as they parse to a vector of `Token`s
pub fn parse_token(input: Span) -> SResult<Token> {
  alt((
    // Brackets
    alt((
      char('(').map(|_| Token::LParen),
      char(')').map(|_| Token::RParen),
      char('{').map(|_| Token::LCurly),
      char('}').map(|_| Token::RCurly),
      char('[').map(|_| Token::LSquare),
      char(']').map(|_| Token::RSquare),
    )),
    // Reserved keywords
    alt((
      tag("let").map(|_| Token::Let),
      tag("return").map(|_| Token::Return),
    )),
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
    alt((parse_char, parse_comment, parse_ident)),
    // Value-like
    double.map(Token::Float),
  ))(input)
}

pub fn parse_tokens(input: Span) -> SResult<Vec<SpanToken>> {
  let parse_item = delimited(
    multispace0,
    alt((span_wrap(parse_token).map(|tok| vec![tok]), parse_string)),
    multispace0,
  );
  many0(parse_item)
    .map(|itemss| itemss.into_iter().flatten().collect())
    .parse(input)
}

#[derive(Clone)]
pub struct TokensWrapper(Vec<SpanToken>);

impl Deref for TokensWrapper {
  type Target = [SpanToken];
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl FromStr for TokensWrapper {
  type Err = Error<LocatedSpan<String>>;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    parse_tokens(Span::new(s))
      .finish()
      .map(|(_rest, output)| TokensWrapper(output))
      .map_err(|Error { input, code }| {
        let input = unsafe {
          LocatedSpan::new_from_raw_offset(
            input.location_offset(),
            input.location_line(),
            input.to_string(),
            (),
          )
        };
        Error { input, code }
      })
  }
}

#[cfg(test)]
mod test {
  use super::{Token::*, *};
  use nom::error::Error;

  pub type TResult = Result<(), nom::Err<Error<Span<'static>>>>;

  #[test]
  fn assignment() -> TResult {
    let input = "\
    let x = 0;\n\
    x = x + 1;\n\
    x = 2 * x;\n\
    x\n\
    ";
    let (rest, tokens) = parse_tokens(Span::new(input))?;
    assert_eq!(rest.into_fragment(), "");
    assert_eq!(
      Vec::from_iter(tokens.into_iter().map(Token::from)),
      vec![
        Let,
        StringIdent("x".into()),
        Assign,
        Float(0.0),
        Semi,
        StringIdent("x".into()),
        Assign,
        StringIdent("x".into()),
        Plus,
        Float(1.0),
        Semi,
        StringIdent("x".into()),
        Assign,
        Float(2.0),
        Times,
        StringIdent("x".into()),
        Semi,
        StringIdent("x".into())
      ]
    );
    Ok(())
  }

  #[test]
  fn arrays() -> TResult {
    let input = "\
    let xs = [0, 1, 2];\n\
    xs[0] = xs[0] + xs[2];\n\
    xs[1] = xs[1] + 2;\n\
    xs[2] = xs[2] * xs[0];\n\
    assert(xs == [2, 3, 4])\
    ";
    let (rest, tokens) = parse_tokens(Span::new(input))?;
    assert_eq!(rest.into_fragment(), "");
    assert_eq!(
      Vec::from_iter(tokens.into_iter().map(Token::from)),
      vec![
        Let,
        StringIdent("xs".into()),
        Assign,
        LSquare,
        Float(0.0),
        Comma,
        Float(1.0),
        Comma,
        Float(2.0),
        RSquare,
        Semi,
        StringIdent("xs".into()),
        LSquare,
        Float(0.0),
        RSquare,
        Assign,
        StringIdent("xs".into()),
        LSquare,
        Float(0.0),
        RSquare,
        Plus,
        StringIdent("xs".into()),
        LSquare,
        Float(2.0),
        RSquare,
        Semi,
        StringIdent("xs".into()),
        LSquare,
        Float(1.0),
        RSquare,
        Assign,
        StringIdent("xs".into()),
        LSquare,
        Float(1.0),
        RSquare,
        Plus,
        Float(2.0),
        Semi,
        StringIdent("xs".into()),
        LSquare,
        Float(2.0),
        RSquare,
        Assign,
        StringIdent("xs".into()),
        LSquare,
        Float(2.0),
        RSquare,
        Times,
        StringIdent("xs".into()),
        LSquare,
        Float(0.0),
        RSquare,
        Semi,
        StringIdent("assert".into()),
        LParen,
        StringIdent("xs".into()),
        Eq,
        LSquare,
        Float(2.0),
        Comma,
        Float(3.0),
        Comma,
        Float(4.0),
        RSquare,
        RParen,
      ]
    );
    Ok(())
  }
}
