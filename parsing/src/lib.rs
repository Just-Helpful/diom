//! Parsing to produce an AST for the Diom language
//!
//! All parsers here are complete and expect to be parsing the complete input
//! at once, i.e. they are parsing a slice and not an iterator.
mod common;
use common::PResult;
use diom_syntax::expressions::Expression;
use diom_tokens::SpanTokens;
use expressions::parse_expression;
use nom::Parser;
use std::ops::Range;

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

pub fn parse_node(input: SpanTokens) -> PResult<SyntaxNode<Span>> {
  (parse_expression.map(SyntaxNode::Expression)).parse(input)
}

type Span = Range<usize>;
