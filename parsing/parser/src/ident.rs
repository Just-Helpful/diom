use crate::common::{PResult, Token};
use crate::errors::SyntaxError;
use crate::{In, Item};
use diom_syntax::ident::{Ident, Name};
use nom::bytes::complete::take;
use nom::combinator::consumed;
use nom::error::ErrorKind;
use nom::Parser;

pub fn try_into_name<'a>(tok: Item<'a>) -> Result<Name, Token> {
  let name = match tok.as_ref() {
    Token::StringIdent(name) => Name::Literal(name.clone()),
    Token::Not => Name::Not,
    Token::And => Name::And,
    Token::Or => Name::Or,
    Token::Plus => Name::Plus,
    Token::Minus => Name::Minus,
    Token::Times => Name::Times,
    Token::Divide => Name::Divide,
    Token::Eq => Name::Eq,
    Token::Ne => Name::Ne,
    Token::Lt => Name::Lt,
    Token::Gt => Name::Gt,
    Token::LtEq => Name::LtEq,
    Token::GtEq => Name::GtEq,
    t => return Err(t.clone()),
  };

  Ok(name)
}

pub fn parse_ident<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Ident<In<'a>>, E> {
  let (input, (info, toks)) = consumed(take(1usize)).parse(input)?;
  let Ok(name) = try_into_name(toks[0].clone()) else {
    return Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Tag)));
  };
  Ok((input, Ident { name, info }))
}

#[cfg(test)]
mod test {
  use super::parse_ident;
  use diom_lexer::parse_tokens;
  use diom_syntax::ident::Name;
  use diom_tokens::SpanTokens;
  use nom::{error::Error, multi::many1, Parser};

  type SpanError<'a> = Error<SpanTokens<'a>>;

  #[test]
  fn simple_ident() {
    let (input, tokens) = parse_tokens("x".into()).unwrap();
    assert_eq!(input, "");
    assert_eq!(tokens.len(), 1, "{tokens:?}.len() != 1");

    let (tokens, ident) = parse_ident::<SpanError>((&tokens).into()).unwrap();
    assert_eq!(tokens, (&[]).into());
    assert_eq!(ident.name, Name::Literal("x".into()));
  }

  #[test]
  fn operator_ident() {
    let (input, tokens) = parse_tokens("+ - / < & !".into()).unwrap();
    assert_eq!(input, "");
    assert_eq!(tokens.len(), 6, "{tokens:?}.len() != 6");

    let (tokens, idents) = many1(parse_ident::<SpanError>)
      .parse((&tokens).into())
      .unwrap();
    assert_eq!(tokens, (&[]).into());
    assert_eq!(
      idents
        .into_iter()
        .map(|ident| ident.name)
        .collect::<Vec<_>>(),
      vec![
        Name::Plus,
        Name::Minus,
        Name::Divide,
        Name::Lt,
        Name::And,
        Name::Not
      ]
    );
  }
}
