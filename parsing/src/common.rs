use std::ops::Range;

pub type Span = Range<usize>;

pub use crate::errors::PResult;
pub use diom_tokens::{SpanToken, SpanTokens, Token};
