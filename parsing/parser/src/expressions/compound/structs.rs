use crate::{
  errors::{PResult, SyntaxError},
  expressions::parse_expression,
  ident::parse_ident,
  parsers::{group, matches},
  In,
};
use diom_syntax::{
  expressions::{Expression, Struct},
  ident::Ident,
};
use diom_tokens::Token;
use nom::{
  combinator::{consumed, eof, opt},
  error::context,
  multi::separated_list0,
  sequence::{preceded, terminated},
  Parser,
};

/// Parses a single struct field
/// @todo maybe expand this to allow for `Foo { x }` like declerations
/// where `x` is a variable defined before `Foo`
pub fn parse_struct_field<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, (Ident<In<'a>>, Expression<In<'a>>), E> {
  let parse_value = preceded(matches::<E>(Token::Colon), parse_expression);
  let parser = parse_ident.and(opt(parse_value));

  let (input, (field, value)) = context("struct field", parser).parse(input)?;
  let value = value.unwrap_or_else(|| Expression::Var(field.clone()));
  Ok((input, (field, value)))
}

pub fn parse_struct<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Struct<In<'a>>, E> {
  let parse_inner = terminated(
    separated_list0(matches(Token::Comma), parse_struct_field),
    eof,
  );
  let parser = group(Token::LCurly, Token::RCurly).and_then(parse_inner);

  let (input, (info, fields)) = consumed(parser).parse(input)?;
  Ok((input, Struct { fields, info }))
}
