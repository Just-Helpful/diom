//! Parentheses for expression grouping and data structures.
use nom::{
  character::{complete::char, multispace0},
  combinator::value,
  error::ParseError,
  sequence::preceded,
  AsChar, Input, Parser,
};

/// Parses a left parenthesis, used for grouping of expressions and statements
pub fn parse_lparen<T: Input<Item: AsChar>, E: ParseError<T>>(
) -> impl Parser<T, Output = (), Error = E> {
  preceded(multispace0(), value((), char('(')))
}
/// Parses a right parenthesis, used for grouping of expressions and statements
pub fn parse_rparen<T: Input<Item: AsChar>, E: ParseError<T>>(
) -> impl Parser<T, Output = (), Error = E> {
  preceded(multispace0(), value((), char(')')))
}
/// Parses a left brace, used for construction of tuples and arrays
pub fn parse_lbrace<T: Input<Item: AsChar>, E: ParseError<T>>(
) -> impl Parser<T, Output = (), Error = E> {
  preceded(multispace0(), value((), char('[')))
}
/// Parses a right brace, used for construction of tuples and arrays
pub fn parse_rbrace<T: Input<Item: AsChar>, E: ParseError<T>>(
) -> impl Parser<T, Output = (), Error = E> {
  preceded(multispace0(), value((), char(']')))
}
/// Parses a left curly brace, used for construction of structs
pub fn parse_lcurly<T: Input<Item: AsChar>, E: ParseError<T>>(
) -> impl Parser<T, Output = (), Error = E> {
  preceded(multispace0(), value((), char('{')))
}
/// Parses a right curly brace, used for construction of structs
pub fn parse_rcurly<T: Input<Item: AsChar>, E: ParseError<T>>(
) -> impl Parser<T, Output = (), Error = E> {
  preceded(multispace0(), value((), char('}')))
}
