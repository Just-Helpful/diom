use super::token::{SpanToken, Token};
use nom::{error::Error, IResult};
use nom_locate::LocatedSpan;

mod chars;
mod comments;
mod token;
pub use token::parse_token;
mod tokens;
pub use tokens::parse_tokens;

type Span<'a> = LocatedSpan<&'a str>;
type SResult<'a, O> = IResult<Span<'a>, O, Error<Span<'a>>>;
