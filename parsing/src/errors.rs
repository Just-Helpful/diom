use diom_tokens::SpanTokens;
use nom::{error::Error, Err};

/// The result type for parsing Diom syntax nodes from spanned tokens
pub type PResult<'a, T> = Result<(SpanTokens<'a>, T), Err<Error<SpanTokens<'a>>>>;
