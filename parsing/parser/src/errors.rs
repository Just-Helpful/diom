use crate::In;
use diom_tokens::SpanTokens;
use nom::error::{ContextError, FromExternalError, ParseError};
pub use nom::{
  error::{Error, ErrorKind},
  Err,
};

pub enum Never {}

/// A trait alias for syntax errors used in parsing Diom syntax nodes
pub trait SyntaxError<'a, E = Never>:
  ParseError<In<'a>> + ContextError<In<'a>> + FromExternalError<In<'a>, E> + 'a
{
}

impl<'a, E0, E: ParseError<In<'a>> + ContextError<In<'a>> + FromExternalError<In<'a>, E0> + 'a>
  SyntaxError<'a, E0> for E
{
}

/// The result type for parsing Diom syntax nodes from spanned tokens
pub type PResult<'a, T, E = Error<SpanTokens<'a>>> = Result<(In<'a>, T), Err<E>>;
