use crate::common::{PResult, SpanTokens, Token};
use crate::Span;
use diom_syntax::ident::{Ident, Name};
use nom::bytes::complete::take;
use nom::error::{Error, ErrorKind};

pub fn parse_ident(input: SpanTokens) -> PResult<Ident<Span>> {
  let (input, toks) = take(1usize)(input)?;
  let tok = &toks[0];
  let name = match &tok.token {
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
    _ => return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
  };
  Ok((
    input,
    Ident {
      name,
      info: tok.span.clone(),
    },
  ))
}

#[cfg(test)]
mod test {
  use std::ops::Deref;

  use super::parse_ident;
  use diom_lexer::parse_tokens;
  use diom_syntax::ident::Name;
  use nom::multi::many1;

  #[test]
  fn simple_ident() {
    let (input, tokens) = parse_tokens("x".into()).unwrap();
    assert_eq!(input.deref(), &"");
    assert_eq!(tokens.len(), 1, "{tokens:?}.len() != 1");

    let (tokens, ident) = parse_ident((&tokens).into()).unwrap();
    assert_eq!(tokens, (&[]).into());
    assert_eq!(ident.name, Name::Literal("x".into()));
  }

  #[test]
  fn operator_ident() {
    let (input, tokens) = parse_tokens("+ - / < & !".into()).unwrap();
    assert_eq!(input.deref(), &"");
    assert_eq!(tokens.len(), 6, "{tokens:?}.len() != 6");

    let (tokens, idents) = many1(parse_ident)((&tokens).into()).unwrap();
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
