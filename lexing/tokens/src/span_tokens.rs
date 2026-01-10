use nom::FindSubstring;
use nom::Input;
use nom::Offset;
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
    let self_ptr = self.0.as_ptr() as usize;
    let snd_ptr = second.0.as_ptr() as usize;
    snd_ptr - self_ptr
  }
}
