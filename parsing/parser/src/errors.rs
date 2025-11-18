use diom_tokens::SpanTokens;
use nom::{Err, error::Error};

/// The result type for parsing Diom syntax nodes from spanned tokens
pub type PResult<'a, T> = Result<(SpanTokens<'a>, T), Err<Error<SpanTokens<'a>>>>;
