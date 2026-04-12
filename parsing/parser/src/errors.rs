use crate::{
  expressions::BuildError,
  parsers::{IsApprox, IsExact},
  In,
};
use diom_tokens::SpanTokens;
use nom::error::{ContextError, FromExternalError, ParseError};
pub use nom::{error::Error, Err};
use std::fmt::Debug;

/// A trait alias for syntax errors used in parsing Diom syntax nodes
pub trait SyntaxError<'a>:
  Debug
  + ParseError<In<'a>>
  + ContextError<In<'a>>
  + FromExternalError<In<'a>, BuildError>
  + FromExternalError<In<'a>, IsExact>
  + FromExternalError<In<'a>, IsApprox>
  + 'a
{
}

impl<
    'a,
    E: Debug
      + ParseError<In<'a>>
      + ContextError<In<'a>>
      + FromExternalError<In<'a>, BuildError>
      + FromExternalError<In<'a>, IsExact>
      + FromExternalError<In<'a>, IsApprox>
      + 'a,
  > SyntaxError<'a> for E
{
}

/// The result type for parsing Diom syntax nodes from spanned tokens
pub type PResult<'a, T, E = Error<SpanTokens<'a>>> = Result<(In<'a>, T), Err<E>>;
