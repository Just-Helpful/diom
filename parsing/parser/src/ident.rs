use crate::common::PResult;
use crate::errors::SyntaxError;
use crate::parsers::{single_item, IsExact};
use crate::In;
use diom_syntax::ident::{Ident, Name};
use nom::combinator::consumed;
use nom::error::ErrorKind;
use nom::Parser;

pub fn parse_name<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Name, E> {
  let (input_, tok) = single_item().parse(input)?;
  let name = tok.token.try_into().map_err(|_| {
    nom::Err::Error(E::from_external_error(
      input,
      ErrorKind::Tag,
      IsExact("Ident".into()),
    ))
  })?;
  Ok((input_, name))
}

pub fn parse_ident<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Ident<In<'a>>, E> {
  consumed(parse_name)
    .map(|(info, name)| Ident { name, info })
    .parse(input)
}

#[cfg(test)]
mod test {
  use super::parse_ident;
  use diom_lexer::parse_tokens;
  use diom_syntax::ident::Name;
  use diom_tokens::SpanTokens;
  use nom::{error::Error, multi::many1, Parser};

  #[test]
  fn simple_ident() {
    let init = "x";
    let (input, tokens) = parse_tokens::<Error<_>>().parse(init).unwrap();
    assert_eq!(input, "");
    assert_eq!(tokens.len(), 1, "{tokens:?}.len() != 1");

    let (tokens, ident) = parse_ident::<Error<_>>(SpanTokens::new(&tokens, init)).unwrap();
    assert_eq!(tokens.len(), 0);
    assert_eq!(ident.name, Name::Literal("x".into()));
  }

  #[test]
  fn operator_ident() {
    let init = "+ - / < & !";
    let (input, tokens) = parse_tokens::<Error<_>>().parse(init).unwrap();
    assert_eq!(input, "");
    assert_eq!(tokens.len(), 6, "{tokens:?}.len() != 6");

    let (tokens, idents) = many1(parse_ident::<Error<_>>)
      .parse(SpanTokens::new(&tokens, init))
      .unwrap();
    assert_eq!(tokens.len(), 0);
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
