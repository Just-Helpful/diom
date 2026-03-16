use diom_tokens::SpanTokens;

/// Merges two slices into one
///
/// # Safety
///
/// Boths `a` and `b` must come from the same original slice
const unsafe fn merge_slices<'a, T>(a: &'a [T], b: &'a [T]) -> &'a [T] {
  let start = a.as_ptr();
  let end = b.as_ptr().add(b.len());
  let len = end.offset_from(start) as usize;
  std::slice::from_raw_parts(start, len)
}

/// Merges two strs into one
///
/// # Safety
///
/// Boths `a` and `b` must come from the same original str
const unsafe fn merge_strs<'a>(a: &'a str, b: &'a str) -> &'a str {
  let bytes = merge_slices(a.as_bytes(), b.as_bytes());
  str::from_utf8_unchecked(bytes)
}

/// Merges two spans into one
///
/// # Safety
///
/// Boths `a` and `b` must come from the same original input span
pub unsafe fn merge_spans<'a>(a: SpanTokens<'a>, b: SpanTokens<'a>) -> SpanTokens<'a> {
  let tokens = merge_slices(a.tokens, b.tokens);
  let origin = merge_strs(a.origin, b.origin);
  SpanTokens::new(tokens, origin)
}
