use std::ops::{Deref, DerefMut, Range};

use nom::{error::ParseError, InputIter, InputTake};
use nom_locate::position;

use super::{span::Span, traits::Parsable};

pub struct ASTNode<I, N>(Span<I>, Span<I>, N);

impl<I, E, T: Parsable<Span<I>, E>> Parsable<Span<I>, E> for ASTNode<I, T>
where
  Span<I>: InputIter + InputTake,
  E: ParseError<Span<I>>,
{
  fn parse(input: Span<I>) -> nom::IResult<Span<I>, Self, E> {
    let (input, s) = position(input)?;
    let (input, node) = T::parse(input)?;
    let (input, e) = position(input)?;
    Ok((input, ASTNode(s, e, node)))
  }
}

impl<I, T> Deref for ASTNode<I, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.2
  }
}

impl<I, T> DerefMut for ASTNode<I, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.2
  }
}

impl<I, T> From<ASTNode<I, T>> for Range<usize> {
  fn from(ASTNode(s, e, _): ASTNode<I, T>) -> Self {
    s.location_offset()..e.location_offset()
  }
}
