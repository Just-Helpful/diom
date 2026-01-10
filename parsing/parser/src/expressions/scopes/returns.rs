use super::super::parse_expression;
use crate::{
  errors::{PResult, SyntaxError},
  parsers::matches,
  In,
};
use diom_syntax::expressions::Return;
use diom_tokens::Token;
use nom::{combinator::consumed, sequence::preceded, Parser};

/// Parses a `return` expression, i.e.
/// ```_
/// let x = (return 2; 3)
/// ```
pub fn parse_return<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Return<In<'a>>, E> {
  let parser = preceded(matches(Token::Return), parse_expression);
  let (input, (info, value)) = consumed(parser).parse(input)?;
  Ok((
    input,
    Return {
      value: Box::new(value),
      info,
    },
  ))
}
