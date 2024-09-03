use super::{SResult, Span};
use diom_tokens::Token;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, alphanumeric1},
  combinator::recognize,
  multi::many0,
  sequence::pair,
};

/// Parses an identifier, used for variable and type creation
pub fn parse_ident(input: Span) -> SResult<Token> {
  let parse_first = alt((alpha1, tag("_")));
  let parse_rest = alt((alphanumeric1, tag("_")));
  let (input, ident) = recognize(pair(parse_first, many0(parse_rest)))(input)?;
  Ok((input, Token::Ident(ident.into_fragment().into())))
}

#[cfg(test)]
mod test {
  use super::{Token::*, *};

  #[test]
  fn letter() {
    let (rest, ident) = parse_ident(Span::new("x + 1")).unwrap();
    assert_eq!(rest.into_fragment(), " + 1");
    assert_eq!(ident, Ident("x".into()))
  }

  #[test]
  fn variable() {
    let (rest, ident) = parse_ident(Span::new("array_len + 1")).unwrap();
    assert_eq!(rest.into_fragment(), " + 1");
    assert_eq!(ident, Ident("array_len".into()))
  }

  #[test]
  fn type_var() {
    let (rest, ident) = parse_ident(Span::new("Bool): Option<T> = ...")).unwrap();
    assert_eq!(rest.into_fragment(), "): Option<T> = ...");
    assert_eq!(ident, Ident("Bool".into()))
  }
}
