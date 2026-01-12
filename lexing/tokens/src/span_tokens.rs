use nom::FindSubstring;
use nom::Input;
use nom::Offset;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::{
  iter::{Cloned, Enumerate},
  ops::Deref,
  slice,
};

use super::{SpanToken, Token};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpanTokens<'a>(pub &'a [SpanToken<'a>]);

impl<'a, T: AsRef<[SpanToken<'a>]> + ?Sized> From<&'a T> for SpanTokens<'a> {
  /// Allows conversion from containers of `SpanToken`s
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// SpanTokens::from(&[LBrace, Float(3.0), RBrace].map(SpanToken::from));
  /// SpanTokens::from(&vec![LBrace.into(), Float(3.0).into(), RBrace.into()]);
  /// ```
  fn from(value: &'a T) -> Self {
    SpanTokens(value.as_ref())
  }
}

impl SpanTokens<'_> {
  /// Splits off the first token, returning the remaining section of the tokens
  pub fn split_first<'a>(&'a self) -> Option<(&'a SpanToken<'a>, SpanTokens<'a>)> {
    let (first, rest) = self.0.split_first()?;
    Some((first, SpanTokens(rest)))
  }

  /// Splits off the last token, returning the starting section of the tokens
  pub fn split_last<'a>(&'a self) -> Option<(&'a SpanToken<'a>, SpanTokens<'a>)> {
    let (last, initial) = self.0.split_last()?;
    Some((last, SpanTokens(initial)))
  }

  /// Returns the pointer range in the source `str`\
  /// that this collections of tokens was parsed from.
  fn str_ptr_range(&self) -> Option<Range<*const u8>> {
    let (fst, lst) = (self.first()?, self.last()?);
    let start_self = fst.origin.as_ptr();
    let end_self = lst.origin.as_bytes().as_ptr_range().end;
    Some(Range {
      start: start_self,
      end: end_self,
    })
  }

  /// Finds the char range within the origin `str` `source`.\
  /// This will return `None` when `self` is not contained in `source`.
  ///
  /// # Safety
  ///
  /// Both `self` and `source` must be within the same allocation.
  pub unsafe fn str_range(&self, source: impl AsRef<str>) -> Option<Range<usize>> {
    let source = source.as_ref();
    let ptrs_self = self.str_ptr_range()?;
    let ptrs_src = source.as_bytes().as_ptr_range();

    // Safety: inherited by the `str_range` function\
    // Given that we're using `offset_from` for `u8`s,\
    // We can relax the "aligned the same" requirement.
    let range = index_range(ptrs_src, ptrs_self)?;
    let start = source[..range.start].chars().count();
    let len = source[range].chars().count();
    Some(Range {
      start,
      end: start + len,
    })
  }
}

/// Finds the index range that finds the `items` range within `slice`.\
/// Returns `None` if `items` range in **not** within `slice`.
///
/// # Safety
///
/// The pointers in `slice` and `items` must be:
///
/// 1. the same pointers or within the same allocation
/// 2. both aligned to the same alignment `sizeof::<T>()`
///
/// See the safety requirements for ptr's offset_from
unsafe fn index_range<T: Sized>(
  slice: Range<*const T>,
  items: Range<*const T>,
) -> Option<Range<usize>> {
  // Handle `items` range being outside `slice`
  let incl_slice = RangeInclusive::new(slice.start, slice.end);
  if !incl_slice.contains(&items.start) || !incl_slice.contains(&items.end) {
    return None;
  }

  // Safety: inherited by the `index_range` function
  let start = items.start.offset_from(slice.start) as usize;
  let end = items.end.offset_from(slice.start) as usize;

  Some(Range { start, end })
}

impl<'a> Deref for SpanTokens<'a> {
  type Target = [SpanToken<'a>];
  /// Simplifies access to the underlying `SpanToken`s
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// let array = [Float(3.0)].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// // this is possible due to `Deref`
  /// assert_eq!(tokens[0], Float(3.0).into());
  /// // as is this
  /// assert_eq!(tokens.len(), 1);
  /// ```
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<'a> Input for SpanTokens<'a> {
  type Item = SpanToken<'a>;
  type Iter = Cloned<slice::Iter<'a, SpanToken<'a>>>;
  type IterIndices = Enumerate<Cloned<slice::Iter<'a, SpanToken<'a>>>>;

  /// Allows for use of `many0`, `many1`, `separated_list0` and other parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Input;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// assert_eq!(SpanTokens::from(&array[0..1]).input_len(), 1);
  /// assert_eq!(SpanTokens::from(&array[0..3]).input_len(), 3);
  /// assert_eq!(SpanTokens::from(&array[0..5]).input_len(), 5);
  /// ```
  fn input_len(&self) -> usize {
    self.0.len()
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Input;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// assert_eq!(tokens.take(1usize), (&array[0..1]).into());
  /// assert_eq!(tokens.take(3usize), (&array[0..3]).into());
  /// assert_eq!(tokens.take(5usize), (&array[0..5]).into());
  /// ```
  fn take(&self, index: usize) -> Self {
    Self(&self.0[0..index])
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Input;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// assert_eq!(tokens.take_from(1usize), (&array[1..]).into());
  /// assert_eq!(tokens.take_from(3usize), (&array[3..]).into());
  /// assert_eq!(tokens.take_from(5usize), (&array[5..]).into());
  /// ```
  fn take_from(&self, index: usize) -> Self {
    Self(&self.0[index..])
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::{Input, error::Error, bytes::complete::take};
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// let (tail, init) = tokens.take_split(1usize);
  /// assert_eq!(init, (&array[0..1]).into());
  /// assert_eq!(tail, (&array[1..]).into());
  /// let (tail, init) = tokens.take_split(3usize);
  /// assert_eq!(init, (&array[0..3]).into());
  /// assert_eq!(tail, (&array[3..]).into());
  /// let (tail, init) = tokens.take_split(5usize);
  /// assert_eq!(init, (&array[0..5]).into());
  /// assert_eq!(tail, (&array[5..]).into());
  ///
  /// let (_tokens, taken) = take::<_, _, Error<_>>(1usize)(tokens).unwrap();
  /// assert_eq!(_tokens, (&array[1..]).into());
  /// assert_eq!(taken, (&array[0..1]).into());
  /// ```
  fn take_split(&self, count: usize) -> (Self, Self) {
    let (init, tail) = self.0.split_at(count);
    (Self(tail), Self(init))
  }

  #[inline]
  fn iter_indices(&self) -> Self::IterIndices {
    self.0.iter().cloned().enumerate()
  }

  #[inline]
  fn iter_elements(&self) -> Self::Iter {
    self.0.iter().cloned()
  }

  #[inline]
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.0.iter().cloned().position(predicate)
  }

  #[inline]
  fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
    if self.0.len() >= count {
      Ok(count)
    } else {
      Err(nom::Needed::new(count - self.0.len()))
    }
  }
}

impl<T: AsRef<Token>> FindSubstring<T> for SpanTokens<'_> {
  fn find_substring(&self, substr: T) -> Option<usize> {
    let substr = substr.as_ref();
    for (i, tok) in self.0.iter().enumerate() {
      if tok.as_ref().matches(substr) {
        return Some(i);
      }
    }
    None
  }
}

impl Offset for SpanTokens<'_> {
  fn offset(&self, second: &Self) -> usize {
    let self_ptr = self.0.as_ptr();
    let snd_ptr = second.0.as_ptr();
    // Safety: nom is zero-copy, so `self_ptr` and `snd_ptr` come from a shared slice
    unsafe { snd_ptr.offset_from(self_ptr) as usize }
  }
}
