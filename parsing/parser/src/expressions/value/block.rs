use super::parse_expression;
use crate::{errors::PResult, parsers::token, types::parse_typedef, Span};
use diom_syntax::expressions::{Block, Statement};
use diom_tokens::{SpanTokens, Token};
use nom::{branch::alt, multi::separated_list0, Parser};

pub fn parse_statement(input: SpanTokens) -> PResult<Statement<Span>> {
  alt((
    parse_expression.map(Statement::Expression),
    parse_typedef.map(Statement::TypeDef),
  ))(input)
}

pub fn parse_block(input: SpanTokens) -> PResult<Block<Span>> {
  let (input, lbrac) = token(&Token::LCurly)(input)?;
  let (input, statements) = separated_list0(token(Token::Semi), parse_statement)(input)?;
  let (input, rbrac) = token(&Token::RCurly)(input)?;

  Ok((
    input,
    Block {
      info: lbrac.span.start..rbrac.span.end,
      statements,
    },
  ))
}
