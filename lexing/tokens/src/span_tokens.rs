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
pub struct SpanTokens<'a> {
  pub tokens: &'a [SpanToken<'a>],
  pub origin: &'a str,
}

impl<'a> SpanTokens<'a> {
  /// Allows conversion from containers of `SpanToken`s
  pub fn new(tokens: &'a (impl AsRef<[SpanToken<'a>]> + ?Sized), origin: &'a str) -> Self {
    SpanTokens {
      tokens: tokens.as_ref(),
      origin,
    }
  }
}

impl<'a, T: AsRef<[SpanToken<'a>]> + ?Sized> From<(&'a T, &'a str)> for SpanTokens<'a> {
  /// Allows conversion from containers of `SpanToken`s
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// SpanTokens::new(&[LBrace, Float(3.0), RBrace].map(SpanToken::from), "");
  /// SpanTokens::new(&vec![LBrace.into(), Float(3.0).into(), RBrace.into()], "");
  /// ```
  fn from(value: (&'a T, &'a str)) -> Self {
    SpanTokens {
      tokens: value.0.as_ref(),
      origin: value.1,
    }
  }
}

impl<'a> AsRef<str> for SpanTokens<'a> {
  fn as_ref(&self) -> &str {
    &self.origin
  }
}

impl SpanTokens<'_> {
  /// Splits off the first token, returning the remaining section of the tokens
  pub fn split_first<'a>(&'a self) -> Option<(&'a SpanToken<'a>, SpanTokens<'a>)> {
    let (first, rest) = self.tokens.split_first()?;
    let origin = &self.origin[first.origin.len()..];
    Some((first, SpanTokens::new(rest, origin)))
  }

  /// Splits off the last token, returning the starting section of the tokens
  pub fn split_last<'a>(&'a self) -> Option<(&'a SpanToken<'a>, SpanTokens<'a>)> {
    let (last, initial) = self.tokens.split_last()?;
    let origin = &self.origin[..self.origin.len() - last.origin.len()];
    Some((last, SpanTokens::new(initial, origin)))
  }

  /// Returns the pointer range in the source `str`\
  /// that this collections of tokens was parsed from.
  fn str_ptr_range(&self) -> Option<Range<*const u8>> {
    let (fst, lst) = (self.first()?, self.last()?);
    let start = fst.origin.as_ptr();
    let end = lst.origin.as_bytes().as_ptr_range().end;
    Some(Range { start, end })
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
  /// let tokens = SpanTokens::new(&array, "");
  ///
  /// // this is possible due to `Deref`
  /// assert_eq!(tokens[0], Float(3.0).into());
  /// // as is this
  /// assert_eq!(tokens.len(), 1);
  /// ```
  fn deref(&self) -> &Self::Target {
    self.tokens
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
  /// assert_eq!(SpanTokens::new(&array[0..1], "").input_len(), 1);
  /// assert_eq!(SpanTokens::new(&array[0..3], "").input_len(), 3);
  /// assert_eq!(SpanTokens::new(&array[0..5], "").input_len(), 5);
  /// ```
  fn input_len(&self) -> usize {
    self.tokens.len()
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Input;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::new(&array, "");
  ///
  /// assert_eq!(tokens.take(1usize).tokens, &array[0..1]);
  /// assert_eq!(tokens.take(3usize).tokens, &array[0..3]);
  /// assert_eq!(tokens.take(5usize).tokens, &array[0..5]);
  /// ```
  fn take(&self, index: usize) -> Self {
    let tokens = &self.tokens[0..index];
    let e_idx = tokens.last().map_or(0, |token| {
      self.origin.offset(token.origin) + token.origin.len()
    });
    Self {
      tokens,
      origin: &self.origin[..e_idx],
    }
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Input;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::new(&array, "");
  ///
  /// assert_eq!(tokens.take_from(1usize).tokens, &array[1..]);
  /// assert_eq!(tokens.take_from(3usize).tokens, &array[3..]);
  /// assert_eq!(tokens.take_from(5usize).tokens, &array[5..]);
  /// ```
  fn take_from(&self, index: usize) -> Self {
    let tokens = &self.tokens[index..];
    let s_idx = tokens
      .first()
      .map_or(0, |token| self.origin.offset(token.origin));
    Self {
      tokens,
      origin: &self.origin[s_idx..],
    }
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::{Input, error::Error, bytes::complete::take};
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::new(&array, "");
  ///
  /// let (tail, init) = tokens.take_split(1usize);
  /// assert_eq!(init.tokens, &array[0..1]);
  /// assert_eq!(tail.tokens, &array[1..]);
  /// let (tail, init) = tokens.take_split(3usize);
  /// assert_eq!(init.tokens, &array[0..3]);
  /// assert_eq!(tail.tokens, &array[3..]);
  /// let (tail, init) = tokens.take_split(5usize);
  /// assert_eq!(init.tokens, &array[0..5]);
  /// assert_eq!(tail.tokens, &array[5..]);
  ///
  /// let (_tokens, taken) = take::<_, _, Error<_>>(1usize)(tokens).unwrap();
  /// assert_eq!(_tokens.tokens, &array[1..]);
  /// assert_eq!(taken.tokens, &array[0..1]);
  /// ```
  fn take_split(&self, count: usize) -> (Self, Self) {
    let (init, tail) = self.tokens.split_at(count);
    let s_idx = tail
      .first()
      .map_or(0, |token| self.origin.offset(token.origin));
    let e_idx = init.last().map_or(0, |token| {
      self.origin.offset(token.origin) + token.origin.len()
    });
    let tail = Self {
      tokens: tail,
      origin: &self.origin[s_idx..],
    };
    let init = Self {
      tokens: init,
      origin: &self.origin[..e_idx],
    };
    (tail, init)
  }

  #[inline]
  fn iter_indices(&self) -> Self::IterIndices {
    self.tokens.iter().cloned().enumerate()
  }

  #[inline]
  fn iter_elements(&self) -> Self::Iter {
    self.tokens.iter().cloned()
  }

  #[inline]
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.tokens.iter().cloned().position(predicate)
  }

  #[inline]
  fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
    if self.tokens.len() >= count {
      Ok(count)
    } else {
      Err(nom::Needed::new(count - self.tokens.len()))
    }
  }
}

impl<T: AsRef<Token>> FindSubstring<T> for SpanTokens<'_> {
  fn find_substring(&self, substr: T) -> Option<usize> {
    let substr = substr.as_ref();
    for (i, tok) in self.tokens.iter().enumerate() {
      if tok.as_ref().matches(substr) {
        return Some(i);
      }
    }
    None
  }
}

impl Offset for SpanTokens<'_> {
  fn offset(&self, second: &Self) -> usize {
    let self_ptr = self.tokens.as_ptr();
    let snd_ptr = second.tokens.as_ptr();
    // Safety: nom is zero-copy, so `self_ptr` and `snd_ptr` come from a shared slice
    unsafe { snd_ptr.offset_from(self_ptr) as usize }
  }
}
