use crate::common::PResult;
use crate::errors::SyntaxError;
use crate::parsers::{single_item, IsExact};
use crate::In;
use diom_syntax::idents::{Ident, Method, Op};
use diom_tokens::Token;
use nom::combinator::consumed;
use nom::Parser;

pub fn parse_ident<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Ident<In<'a>>, E> {
  let mut parser = consumed(single_item()).map_res(|(info, tok)| match tok.token {
    Token::StringIdent(name) => Ok(Ident { name, info }),
    _ => Err(IsExact("Ident".into())),
  });

  parser.parse(input)
}

pub fn parse_op<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Op<In<'a>>, E> {
  let mut parser = consumed(single_item()).map_res(|(info, tok)| {
    Ok::<_, IsExact>(Op {
      sym: tok.token.try_into().map_err(|_| IsExact("Op".into()))?,
      info,
    })
  });

  parser.parse(input)
}

pub fn parse_method<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Method<In<'a>>, E> {
  let mut parser = consumed(single_item()).map_res(|(info, tok)| {
    Ok::<_, IsExact>(Method {
      name: tok.token.try_into().map_err(|_| IsExact("Ident".into()))?,
      info,
    })
  });

  parser.parse(input)
}

#[cfg(test)]
mod test {
  use super::parse_method;
  use diom_lexer::parse_tokens;
  use diom_syntax::idents::{Name, Symbol};
  use diom_tokens::SpanTokens;
  use nom::{error::Error, multi::many1, Parser};

  #[test]
  fn simple_ident() {
    let init = "x";
    let (input, tokens) = parse_tokens::<Error<_>>().parse(init).unwrap();
    assert_eq!(input, "");
    assert_eq!(tokens.len(), 1, "{tokens:?}.len() != 1");

    let (tokens, ident) = parse_method::<Error<_>>(SpanTokens::new(&tokens, init)).unwrap();
    assert_eq!(tokens.len(), 0);
    assert_eq!(ident.name, Name::Literal("x".into()));
  }

  #[test]
  fn operator_ident() {
    let init = "+ - / < & !";
    let (input, tokens) = parse_tokens::<Error<_>>().parse(init).unwrap();
    assert_eq!(input, "");
    assert_eq!(tokens.len(), 6, "{tokens:?}.len() != 6");

    let (tokens, idents) = many1(parse_method::<Error<_>>)
      .parse(SpanTokens::new(&tokens, init))
      .unwrap();
    assert_eq!(tokens.len(), 0);
    assert_eq!(
      idents
        .into_iter()
        .map(|ident| ident.name)
        .collect::<Vec<_>>(),
      vec![
        Name::Symbol(Symbol::Plus),
        Name::Symbol(Symbol::Minus),
        Name::Symbol(Symbol::Divide),
        Name::Symbol(Symbol::Lt),
        Name::Symbol(Symbol::And),
        Name::Symbol(Symbol::Not)
      ]
    );
  }
}
