use super::{parse_pattern, parse_rest};
use crate::{
  errors::{PResult, SyntaxError},
  ident::parse_ident,
  parsers::{group, matches},
  path::parse_path,
  In,
};
use diom_syntax::patterns::{
  structs::{Struct, StructField, StructItem},
  Pattern,
};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, eof, opt},
  multi::separated_list1,
  sequence::{preceded, terminated},
  Parser,
};

pub fn parse_struct_field<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, StructField<In<'a>>, E> {
  let parse_sub = preceded(matches(Token::Colon), parse_pattern);
  let parser = parse_ident.and(opt(parse_sub)).map(|(name, pat)| {
    let pat = pat.unwrap_or_else(|| Pattern::Var(name.clone()));
    (name, pat)
  });

  let (input, (info, (name, pattern))) = consumed(parser).parse(input)?;
  Ok((
    input,
    StructField {
      name,
      pattern,
      info,
    },
  ))
}

pub fn parse_struct_item<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, StructItem<In<'a>>, E> {
  alt((
    parse_struct_field.map(StructItem::Field),
    parse_rest.map(StructItem::Rest),
  ))
  .parse(input)
}

pub fn parse_struct<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Struct<In<'a>>, E> {
  let parse_inner = terminated(
    separated_list1(matches(Token::Comma), parse_struct_item),
    eof,
  );
  let parser = opt(parse_path).and(group(Token::LCurly, Token::RCurly).and_then(parse_inner));

  let (input, (info, (name, fields))) = consumed(parser).parse(input)?;
  Ok((input, Struct { name, fields, info }))
}
