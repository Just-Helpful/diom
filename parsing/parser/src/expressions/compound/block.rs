use super::super::parse_expression;
use crate::{
  errors::{PResult, SyntaxError},
  parsers::{group, token_separated_list},
  types::parse_typedef,
  In,
};
use diom_syntax::expressions::{Block, Statement};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, eof},
  error::context,
  sequence::terminated,
  Parser,
};

pub fn parse_statement<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Statement<In<'a>>, E> {
  alt((
    context("expression", parse_expression).map(Statement::Expression),
    context("type def", parse_typedef).map(Statement::TypeDef),
  ))
  .parse(input)
}

pub fn parse_block<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Block<In<'a>>, E> {
  let parse_inner = context(
    "block inner",
    terminated(token_separated_list(Token::Semi, parse_statement), eof),
  );
  let parser = context(
    "block outer",
    group(Token::LParen, Token::RParen).and_then(parse_inner),
  );

  let (input, (info, statements)) = consumed(parser).parse(input)?;
  Ok((input, Block { statements, info }))
}
