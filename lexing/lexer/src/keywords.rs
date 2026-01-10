//! Reserved keywords in the language.
//! This includes both plain text keywords, function arrows, and assignment operators.
use nom::{bytes::complete::tag, error::Error, Compare, Input, Parser};

/// The let keyword, used for initial creation and assignment of variables
pub fn let_keyword<I: Input + (for<'a> Compare<&'a str>)>(
) -> impl Parser<I, Output = I, Error = Error<I>> {
  tag("let")
}
/// The type keyword, used to define type aliases and data structures
pub fn type_keyword<I: Input + (for<'a> Compare<&'a str>)>(
) -> impl Parser<I, Output = I, Error = Error<I>> {
  tag("type")
}
/// The return keyword, used to early return from blocks
pub fn return_keyword<I: Input + (for<'a> Compare<&'a str>)>(
) -> impl Parser<I, Output = I, Error = Error<I>> {
  tag("return")
}

/// The function arrow, used to define anonymous functions
pub fn function_arrow<I: Input + (for<'a> Compare<&'a str>)>(
) -> impl Parser<I, Output = I, Error = Error<I>> {
  tag("=>")
}
