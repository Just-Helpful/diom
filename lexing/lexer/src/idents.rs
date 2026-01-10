use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, alphanumeric1},
  combinator::recognize,
  error::Error,
  multi::many0,
  Parser,
};

/// Parses an identifier, used for variable and type creation
pub fn parse_ident<'a>() -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
  let parse_first = alt((alpha1, tag("_")));
  let parse_rest = alt((alphanumeric1, tag("_")));
  recognize(parse_first.and(many0(parse_rest)))
}

#[cfg(test)]
mod test {
  use super::parse_ident;
  use crate::tests::TestResult;
  use nom::Parser;

  #[test]
  fn letter() -> TestResult<'static, ()> {
    let res = parse_ident().parse("x + 1".into())?;
    assert_eq!(res, (" + 1", "x"));
    Ok(())
  }

  #[test]
  fn variable() -> TestResult<'static, ()> {
    let res = parse_ident().parse("array_len + 1".into())?;
    assert_eq!(res, (" + 1", "array_len"));
    Ok(())
  }

  #[test]
  fn type_var() -> TestResult<'static, ()> {
    let res = parse_ident().parse("Bool): Option<T> = ...".into())?;
    assert_eq!(res, ("): Option<T> = ...", "Bool"));
    Ok(())
  }
}
