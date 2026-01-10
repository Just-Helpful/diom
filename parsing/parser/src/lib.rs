//! Parsing to produce an AST for the Diom language
//!
//! All parsers here are complete and expect to be parsing the complete input
//! at once, i.e. they are parsing a slice and not an iterator.
mod common;
use common::PResult;
use diom_syntax::expressions::Expression;
use diom_tokens::{SpanToken, SpanTokens};
use expressions::parse_expression;
use nom::Parser;

use crate::errors::SyntaxError;

pub mod errors;
pub mod expressions;
pub mod ident;
pub mod parsers;
pub mod path;
pub mod patterns;
pub mod types;

/// Top level syntax nodes
pub enum SyntaxNode<I> {
  Expression(Expression<I>),
}

pub fn parse_node<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, SyntaxNode<In<'a>>, E> {
  parse_expression.map(SyntaxNode::Expression).parse(input)
}

type In<'a> = SpanTokens<'a>;
type Item<'a> = SpanToken<'a>;
