use crate::In;
use diom_tokens::SpanTokens;
use nom::error::{ContextError, FromExternalError, ParseError};
pub use nom::{
  error::{Error, ErrorKind},
  Err,
};

/// An error raised when building prefix, infix and postfix operations
pub enum BuildError {}

/// A trait alias for syntax errors used in parsing Diom syntax nodes
pub trait SyntaxError<'a>:
  ParseError<In<'a>> + ContextError<In<'a>> + FromExternalError<In<'a>, BuildError> + 'a
{
}

impl<
    'a,
    E: ParseError<In<'a>> + ContextError<In<'a>> + FromExternalError<In<'a>, BuildError> + 'a,
  > SyntaxError<'a> for E
{
}

/// The result type for parsing Diom syntax nodes from spanned tokens
pub type PResult<'a, T, E = Error<SpanTokens<'a>>> = Result<(In<'a>, T), Err<E>>;
