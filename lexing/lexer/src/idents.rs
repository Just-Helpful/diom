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
  Ok((input, Token::StringIdent(ident.into_fragment().into())))
}

#[cfg(test)]
mod test {
  use super::{parse_ident, Token::*};
  use crate::tests::TResult;

  #[test]
  fn letter() -> TResult<()> {
    let (rest, ident) = parse_ident("x + 1".into())?;
    assert_eq!(rest.into_fragment(), " + 1");
    assert_eq!(ident, StringIdent("x".into()));
    Ok(())
  }

  #[test]
  fn variable() -> TResult<()> {
    let (rest, ident) = parse_ident("array_len + 1".into())?;
    assert_eq!(rest.into_fragment(), " + 1");
    assert_eq!(ident, StringIdent("array_len".into()));
    Ok(())
  }

  #[test]
  fn type_var() -> TResult<()> {
    let (rest, ident) = parse_ident("Bool): Option<T> = ...".into())?;
    assert_eq!(rest.into_fragment(), "): Option<T> = ...");
    assert_eq!(ident, StringIdent("Bool".into()));
    Ok(())
  }
}
