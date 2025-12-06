use super::parse_expression;
use crate::{common::PResult, Span};
use nom::branch::alt;

pub mod compound;
pub mod literals;

use compound::parse_compound_value;
use diom_syntax::expressions::Expression;
use diom_tokens::SpanTokens;
use literals::parse_literal_value;

/// Values that have clear start + end delimiters
pub fn parse_value(input: SpanTokens) -> PResult<Expression<Span>> {
  alt((parse_literal_value, parse_compound_value))(input)
}
