use nom::{Compare, CompareResult, InputIter, InputLength, InputTake, Slice};
use std::{
  iter::Enumerate,
  ops::{Deref, RangeFrom},
  slice,
};

use super::token::SpanToken;

#[derive(Clone, Copy)]
pub struct SpanTokens<'a>(&'a [SpanToken]);

impl<'a> Deref for SpanTokens<'a> {
  type Target = [SpanToken];
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

// impl<'a> AsBytes for Tokens<'a> {
//   fn as_bytes(&self) -> &[u8] {
//     todo!()
//   }
// }

impl<'a, 'b> Compare<SpanTokens<'b>> for SpanTokens<'a> {
  fn compare(&self, t: SpanTokens<'b>) -> nom::CompareResult {
    // pretty much the same as from nom's implementation on &[u8]
    let any_diff = self.0.iter().zip(t.0.iter()).any(|(s, t)| s != t);

    if any_diff {
      return CompareResult::Error;
    }
    if self.len() < t.len() {
      return CompareResult::Incomplete;
    }

    CompareResult::Ok
  }

  fn compare_no_case(&self, t: SpanTokens<'b>) -> CompareResult {
    self.compare(t)
  }
}

impl<'a> Slice<RangeFrom<usize>> for SpanTokens<'a> {
  fn slice(&self, range: RangeFrom<usize>) -> Self {
    Self(self.0.slice(range))
  }
}

impl<'a> InputTake for SpanTokens<'a> {
  fn take(&self, count: usize) -> Self {
    Self(&self.0[0..count])
  }

  fn take_split(&self, count: usize) -> (Self, Self) {
    let (init, tail) = self.0.split_at(count);
    (Self(init), Self(tail))
  }
}

impl<'a> InputLength for SpanTokens<'a> {
  fn input_len(&self) -> usize {
    self.0.len()
  }
}

impl<'a> InputIter for SpanTokens<'a> {
  type Item = &'a SpanToken;
  type Iter = Enumerate<slice::Iter<'a, SpanToken>>;
  type IterElem = slice::Iter<'a, SpanToken>;

  #[inline]
  fn iter_indices(&self) -> Self::Iter {
    self.0.iter().enumerate()
  }

  #[inline]
  fn iter_elements(&self) -> Self::IterElem {
    self.0.iter()
  }

  #[inline]
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.0.iter().position(predicate)
  }

  #[inline]
  fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
    if self.0.len() >= count {
      Ok(count)
    } else {
      Err(nom::Needed::Unknown)
    }
  }
}
