use nom::{FindSubstring, InputIter, InputLength, InputTake, Slice};
use std::{
  iter::Enumerate,
  ops::{Deref, RangeFrom},
  slice,
};

use super::{SpanToken, Token};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpanTokens<'a>(&'a [SpanToken]);

/// Allows conversion from containers of `SpanToken`s
///
/// ```
/// # use diom_lexing::{SpanTokens, Token::*};
/// SpanTokens::from(&[LSquare.into(), Float(3.0).into(), RSquare.into()]);
/// SpanTokens::from(&vec![LSquare.into(), Float(3.0).into(), RSquare.into()]);
/// ```
impl<'a, T: AsRef<[SpanToken]> + ?Sized> From<&'a T> for SpanTokens<'a> {
  fn from(value: &'a T) -> Self {
    SpanTokens(value.as_ref())
  }
}

/// Simplifies access to the underlying `SpanToken`s
///
/// ```
/// # use diom_lexing::{SpanTokens, Token::*};
/// let float = [Float(3.0).into()];
/// let tokens = SpanTokens::from(&float);
/// // this is possible due to `Deref`
/// assert_eq!(tokens[0], Float(3.0).into());
/// // as is this
/// assert_eq!(tokens.len(), 1);
/// ```
impl<'a> Deref for SpanTokens<'a> {
  type Target = [SpanToken];
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

/// Simplifies skipping of a single `SpanToken`
///
/// ```
/// # use diom_lexing::{SpanTokens, Token::*, SpanToken};
/// # use nom::Slice;
/// let array = [LSquare, Float(1.0), Comma, Float(2.0), RSquare];
/// let array = array.map(SpanToken::from);
/// let tokens = SpanTokens::from(&array);
///
/// assert_eq!(tokens.slice(1..), SpanTokens::from(&array[1..]));
/// assert_eq!(tokens.slice(3..), SpanTokens::from(&array[3..]));
/// assert_eq!(tokens.slice(5..), SpanTokens::from(&[]));
/// ```
impl<'a> Slice<RangeFrom<usize>> for SpanTokens<'a> {
  fn slice(&self, range: RangeFrom<usize>) -> Self {
    Self(self.0.slice(range))
  }
}

/// Allows for use of `double`, `take`, `tag` and other common parsers
impl<'a> InputTake for SpanTokens<'a> {
  fn take(&self, count: usize) -> Self {
    Self(&self.0[0..count])
  }

  fn take_split(&self, count: usize) -> (Self, Self) {
    let (init, tail) = self.0.split_at(count);
    (Self(init), Self(tail))
  }
}

/// Allows for use of `many0`, `many1`, `separated_list0` and other parsers
impl<'a> InputLength for SpanTokens<'a> {
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

impl<'a> FindSubstring<SpanToken> for SpanTokens<'a> {
  fn find_substring(&self, substr: SpanToken) -> Option<usize> {
    for (i, tok) in self.0.iter().enumerate() {
      if tok.matches(&substr) {
        return Some(i);
      }
    }
    None
  }
}

impl<'a> FindSubstring<&Token> for SpanTokens<'a> {
  fn find_substring(&self, substr: &Token) -> Option<usize> {
    for (i, tok) in self.0.iter().enumerate() {
      if tok.as_ref().matches(substr) {
        return Some(i);
      }
    }
    None
  }
}
