use crate::{
  errors::{PResult, SyntaxError},
  expressions::parse_expression,
  parsers::{group, token_separated_list},
  In,
};
use diom_syntax::expressions::Array;
use diom_tokens::Token;
use nom::{combinator::eof, Parser};

pub fn parse_array<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Array<In<'a>>, E> {
  let parse_inner = token_separated_list(Token::Comma, parse_expression).and(eof);
  let mut parse_array = group(Token::LBrace, Token::RBrace).and_then(parse_inner);

  let (input, (contents, info)) = parse_array.parse(input)?;
  Ok((input, Array { contents, info }))
}
