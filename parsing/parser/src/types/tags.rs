use crate::{common::PResult, errors::SyntaxError, idents::parse_ident, types::parse_type, In};
use diom_syntax::{types::Tagged, Ptr};
use nom::{combinator::consumed, Parser};

pub fn parse_tagged<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Tagged<In<'a>>, E> {
  let parser = parse_ident.and(parse_type);
  let (input, (info, (name, value))) = consumed(parser).parse(input)?;

  Ok((
    input,
    Tagged {
      name,
      value: Ptr::new(value),
      info,
    },
  ))
}
