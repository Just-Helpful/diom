use super::{arrays::parse_array, parse_type, structs::parse_struct, tuples::parse_tuple};
use crate::{
  errors::{PResult, SyntaxError},
  ident::parse_ident,
  parsers::matches,
  In,
};
use diom_syntax::types::{Type, TypeDef};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::consumed,
  sequence::{preceded, terminated},
  Parser,
};

pub fn parse_typedef<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, TypeDef<In<'a>>, E> {
  let parse_tag_type = alt((
    parse_array.map_opt(|a| a.name.clone().map(|name| (name, Type::Array(a)))),
    parse_struct.map_opt(|s| s.name.clone().map(|name| (name, Type::Struct(s)))),
    parse_tuple.map_opt(|t| t.name.clone().map(|name| (name, Type::Tuple(t)))),
  ));

  let parse_type = alt((
    terminated(parse_ident, matches(Token::Assign)).and(parse_type),
    parse_tag_type,
  ));

  let parser = preceded(matches(Token::Type), parse_type);

  let (input, (info, (name, value))) = consumed(parser).parse(input)?;
  Ok((
    input,
    TypeDef {
      info,
      name,
      value: Box::new(value),
    },
  ))
}
