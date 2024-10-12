use super::{SResult, Span};
use nom::{combinator::eof, error::Error};

/// The result type used in testing
pub type TResult<T> = Result<T, nom::Err<Error<Span<'static>>>>;

/// performs a complete parse and returns the output
///
/// will panic if either:
/// 1. the parsing fails
/// 2. the parsing doesn't fully consume input
pub fn complete_parse<T>(
  mut parser: impl for<'a> FnMut(Span<'a>) -> SResult<'a, T>,
  input: &'static str,
) -> TResult<T> {
  let (rest, res) = parser(input.into())?;
  eof(rest)?;
  Ok(res)
}
