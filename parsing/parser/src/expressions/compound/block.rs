use super::super::parse_expression;
use crate::{
  errors::{PResult, SyntaxError},
  parsers::{group, matches},
  types::parse_typedef,
  In,
};
use diom_syntax::expressions::{Block, Statement};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, eof},
  error::context,
  multi::separated_list0,
  sequence::terminated,
  Parser,
};

pub fn parse_statement<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Statement<In<'a>>, E> {
  alt((
    parse_expression.map(Statement::Expression),
    parse_typedef.map(Statement::TypeDef),
  ))
  .parse(input)
}

pub fn parse_block<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Block<In<'a>>, E> {
  let parse_inner = context(
    "block inner",
    terminated(separated_list0(matches(Token::Semi), parse_statement), eof),
  );
  let parser = context(
    "block outer",
    group(Token::LParen, Token::RParen).and_then(parse_inner),
  );

  let (input, (info, statements)) = consumed(parser).parse(input)?;
  Ok((input, Block { statements, info }))
}
