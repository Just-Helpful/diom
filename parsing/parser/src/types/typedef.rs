use super::{arrays::parse_array, parse_type, structs::parse_struct, tuples::parse_tuple};
use crate::{Span, errors::PResult, ident::parse_ident, parsers::token};
use diom_info_traits::InfoRef;
use diom_syntax::types::{Type, TypeDef};
use diom_tokens::{SpanTokens, Token};
use nom::{
  Parser,
  branch::alt,
  sequence::{pair, preceded},
};

pub fn parse_typedef(input: SpanTokens) -> PResult<TypeDef<Span>> {
  let (input, tok) = token(Token::Let)(input)?;
  let (_, name) = parse_ident(input)?;
  let (input, value) = alt((
    preceded(pair(parse_ident, token(Token::Colon)), parse_type),
    parse_array.map(Type::Array),
    parse_struct.map(Type::Struct),
    parse_tuple.map(Type::Tuple),
  ))(input)?;
  Ok((
    input,
    TypeDef {
      info: tok.span.start..value.info().end,
      name,
      value: Box::new(value),
    },
  ))
}
