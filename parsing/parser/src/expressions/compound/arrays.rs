use crate::{
  errors::{PResult, SyntaxError},
  expressions::parse_expression,
  parsers::{group, token_separated_list},
  In,
};
use diom_syntax::expressions::Array;
use diom_tokens::Token;
use nom::{
  combinator::{consumed, eof},
  error::context,
  sequence::terminated,
  Parser,
};

pub fn parse_array<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Array<In<'a>>, E> {
  let parse_inner = context(
    "array inner",
    terminated(token_separated_list(Token::Comma, parse_expression), eof),
  );
  let parser = context(
    "array outer",
    group(Token::LBrace, Token::RBrace).and_then(parse_inner),
  );

  let (input, (info, contents)) = consumed(parser).parse(input)?;
  Ok((input, Array { contents, info }))
}
