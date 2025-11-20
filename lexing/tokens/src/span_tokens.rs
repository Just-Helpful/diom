use nom::{FindSubstring, InputIter, InputLength, InputTake, Slice, UnspecializedInput};
use std::{
  iter::Enumerate,
  ops::{Deref, Range, RangeFrom, RangeTo},
  slice,
};

use super::{SpanToken, Token};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpanTokens<'a>(&'a [SpanToken]);

impl<'a, T: AsRef<[SpanToken]> + ?Sized> From<&'a T> for SpanTokens<'a> {
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
  pub fn split_first(&'_ self) -> Option<(&'_ SpanToken, SpanTokens<'_>)> {
    let (first, rest) = self.0.split_first()?;
    Some((first, SpanTokens(rest)))
  }

  /// Splits off the last token, returning the starting section of the tokens
  pub fn split_last(&'_ self) -> Option<(&'_ SpanToken, SpanTokens<'_>)> {
    let (last, initial) = self.0.split_last()?;
    Some((last, SpanTokens(initial)))
  }
}

impl Deref for SpanTokens<'_> {
  type Target = [SpanToken];
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

impl Slice<Range<usize>> for SpanTokens<'_> {
  /// Accesses sub-ranges of tokens
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Slice;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// assert_eq!(tokens.slice(0..1), (&array[0..1]).into());
  /// assert_eq!(tokens.slice(2..4), (&array[2..4]).into());
  /// ```
  fn slice(&self, range: Range<usize>) -> Self {
    Self(self.0.slice(range))
  }
}

impl Slice<RangeFrom<isize>> for SpanTokens<'_> {
  fn slice(&self, range: RangeFrom<isize>) -> Self {
    let mut s = range.start;
    if s < 0 {
      s += self.len() as isize;
    }
    Self(self.0.slice((s as usize)..))
  }
}

impl Slice<RangeTo<isize>> for SpanTokens<'_> {
  fn slice(&self, range: RangeTo<isize>) -> Self {
    let mut e = range.end;
    if e < 0 {
      e += self.len() as isize;
    }
    Self(self.0.slice(..(e as usize)))
  }
}

impl Slice<RangeFrom<usize>> for SpanTokens<'_> {
  /// Simplifies skipping of a single `SpanToken`
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::Slice;
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// assert_eq!(tokens.slice(1usize..), SpanTokens::from(&array[1..]));
  /// assert_eq!(tokens.slice(3usize..), SpanTokens::from(&array[3..]));
  /// assert_eq!(tokens.slice(5usize..), SpanTokens::from(&[]));
  /// ```
  fn slice(&self, range: RangeFrom<usize>) -> Self {
    Self(self.0.slice(range))
  }
}

impl InputTake for SpanTokens<'_> {
  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::{InputTake};
  /// let array = [
  ///   LBrace, Float(1.0), Comma, Float(2.0), RBrace
  /// ].map(SpanToken::from);
  /// let tokens = SpanTokens::from(&array);
  ///
  /// assert_eq!(tokens.take(1usize), (&array[0..1]).into());
  /// assert_eq!(tokens.take(3usize), (&array[0..3]).into());
  /// assert_eq!(tokens.take(5usize), (&array[0..5]).into());
  /// ```
  fn take(&self, count: usize) -> Self {
    Self(&self.0[0..count])
  }

  /// Allows for use of `double`, `take`, `tag` and other common parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::{InputTake, error::Error, bytes::complete::take};
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
}

/// Allows for the `take_till1` parser
/// We can't really do anything special for searching,
/// so just use the default, linear search
impl UnspecializedInput for SpanTokens<'_> {}

impl InputLength for SpanTokens<'_> {
  /// Allows for use of `many0`, `many1`, `separated_list0` and other parsers
  ///
  /// ```
  /// # use diom_tokens::{SpanTokens, SpanToken, Token::*};
  /// # use nom::{InputLength};
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
}

/// Needed to allow most parsers to work
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
