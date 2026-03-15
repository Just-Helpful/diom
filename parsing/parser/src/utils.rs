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

/// Merges two spans into one
///
/// # Safety
///
/// Boths `a` and `b` must come from the same original input span
pub unsafe fn merge_spans<'a>(a: SpanTokens<'a>, b: SpanTokens<'a>) -> SpanTokens<'a> {
  let merged = merge_slices(a.tokens, b.tokens);
  SpanTokens::from(merged)
}
