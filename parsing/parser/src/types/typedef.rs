use super::parse_type;
use crate::{
  errors::{PResult, SyntaxError},
  idents::parse_ident,
  parsers::matches,
  types::parse_tagged,
  In,
};
use diom_syntax::types::{Alias, NewType, TypeDef};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, cut},
  error::context,
  sequence::{delimited, preceded},
  Parser,
};

pub fn parse_alias<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Alias<In<'a>>, E> {
  let parser =
    delimited(matches(Token::Type), parse_ident, matches(Token::Assign)).and(cut(parse_type));
  let parser = context("type alias", parser);
  let (input, (info, (name, value))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Alias {
      name,
      value: Box::new(value),
      info,
    },
  ))
}

pub fn parse_newtype<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, NewType<In<'a>>, E> {
  let parser = preceded(matches(Token::Type), parse_tagged);
  let parser = context("newtype", parser);
  let (input, (info, tag)) = consumed(parser).parse(input)?;
  Ok((input, NewType { tag, info }))
}

pub fn parse_typedef<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, TypeDef<In<'a>>, E> {
  alt((
    parse_newtype.map(TypeDef::New),
    parse_alias.map(TypeDef::Alias),
  ))
  .parse(input)
}

#[cfg(test)]
mod tests {
  use diom_syntax::{
    idents::Ident,
    types::{Alias, Type, TypeDef},
  };
  use diom_tokens::SpanTokens;

  use crate::tests::utils::{quick_lex, quick_parse};

  /// Tests that `Typedef("!", "!")` formats and parses correctly
  #[test]
  fn typedef_eq_safe() {
    let def = TypeDef::Alias(Alias {
      name: Ident {
        name: "_".into(),
        info: (),
      },
      value: Box::new(Type::Var(Ident {
        name: "_".into(),
        info: (),
      })),
      info: (),
    });

    let code = format!("({def})");
    let tokens = quick_lex(&code);
    quick_parse(&code, SpanTokens::new(&tokens, &code));
  }

  /// Tests that a typedef with a tuple type parses
  #[test]
  fn typedef_tuple() {
    let code = "(type ! = [!,!])";
    let tokens = quick_lex(code);
    quick_parse(code, SpanTokens::new(&tokens, &code));
  }
}
