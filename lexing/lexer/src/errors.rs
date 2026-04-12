use std::num::ParseIntError;

use crate::In;
use nom::error::{ContextError, FromExternalError, ParseError};

/// A trait alias for syntax errors used in parsing Diom syntax nodes
pub trait TokensError<'a>:
  ParseError<In<'a>> + ContextError<In<'a>> + FromExternalError<In<'a>, ParseIntError> + 'a
{
}

impl<
    'a,
    E: ParseError<In<'a>> + ContextError<In<'a>> + FromExternalError<In<'a>, ParseIntError> + 'a,
  > TokensError<'a> for E
{
}
