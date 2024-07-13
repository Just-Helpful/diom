use super::{SResult, Span};

/// performs a complete parse and returns the output
///
/// will panic if either:
/// 1. the parsing fails
/// 2. the parsing doesn't fully consume input
pub fn complete_parse<T>(
  mut parser: impl for<'a> FnMut(Span<'a>) -> SResult<'a, T>,
  input: &str,
) -> T {
  let (rest, res) = parser(Span::new(input)).unwrap();
  assert!(
    rest.len() == 0,
    "expected to parse all input,\nhowever parser left {:?}",
    rest.into_fragment()
  );
  res
}
