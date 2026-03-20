use crate::{
  common::PResult, errors::SyntaxError, idents::parse_ident, patterns::parse_pattern, In,
};
use diom_syntax::patterns::Tagged;
use nom::{combinator::consumed, Parser};

pub fn parse_tagged<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Tagged<In<'a>>, E> {
  let parser = parse_ident.and(parse_pattern);
  let (input, (info, (name, value))) = consumed(parser).parse(input)?;

  Ok((
    input,
    Tagged {
      name,
      value: Box::new(value),
      info,
    },
  ))
}
