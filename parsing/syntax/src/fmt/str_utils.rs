use std::num::NonZero;

/// Returns the byte at the given character index\
/// or the remainder of `i` after the bytes have been exhausted.
pub fn byte_at(s: &str, mut i: usize) -> Result<usize, NonZero<usize>> {
  let mut bytes = s
    .char_indices()
    .map(|idx| idx.0)
    .chain(std::iter::once(s.len()));

  loop {
    let Some(byte) = bytes.next() else {
      return Err(NonZero::try_from(i + 1).expect("i is a `usize`"));
    };
    if i == 0 {
      return Ok(byte);
    }
    i -= 1;
  }
}

#[cfg(test)]
mod tests {
  use std::num::NonZero;

  use super::byte_at;

  #[test]
  fn test_byte_at() {
    assert_eq!(byte_at("", 0), Ok(0));
    assert_eq!(byte_at("s", 0), Ok(0));
    assert_eq!(byte_at("s", 1), Ok(1));

    for i in 0..(2 << 24) {
      let Some(c) = char::from_u32(i) else { continue };
      assert_eq!(byte_at(&c.to_string(), 1), Ok(c.len_utf8()));
    }

    for i in 1..(2 << 12) {
      assert_eq!(byte_at("", i), Err(NonZero::try_from(i).unwrap()))
    }
  }
}
