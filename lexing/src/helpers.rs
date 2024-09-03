use super::{SResult, Span};
use diom_tokens::{SpanToken, Token};
use nom::{error::Error, Parser};
use nom_locate::position;

pub(crate) fn span_wrap<'a, F: Parser<Span<'a>, Token, Error<Span<'a>>>>(
  mut parser: F,
) -> impl FnMut(Span<'a>) -> SResult<'a, SpanToken> {
  move |input| {
    let (input, start) = position(input)?;
    let (input, token) = parser.parse(input)?;
    let (input, end) = position(input)?;
    Ok((
      input,
      SpanToken {
        token,
        span: start.location_offset()..end.location_offset(),
      },
    ))
  }
}
