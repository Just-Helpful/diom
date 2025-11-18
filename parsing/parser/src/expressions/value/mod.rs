use super::parse_expression;
use crate::{Span, common::PResult};
use nom::{Parser, branch::alt};

mod base;
mod block;
mod compound;
mod group;

use base::parse_base;
use block::parse_block;
use compound::parse_compound;
use diom_syntax::expressions::Expression;
use diom_tokens::SpanTokens;
use group::parse_group;

/// Values that have clear start + end delimiters
pub fn parse_value(input: SpanTokens) -> PResult<Expression<Span>> {
  alt((
    parse_compound,
    parse_block.map(Expression::Block),
    parse_group.map(Expression::Group),
    parse_base,
  ))(input)
}
