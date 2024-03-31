use nom::{
  error::{Error, ParseError},
  IResult, Parser,
};

pub trait Parsable<I, E: ParseError<I> = Error<I>>: Sized {
  fn parse(input: I) -> IResult<I, Self, E>;
}

impl<I, E: ParseError<I>, T: Parsable<I, E>> Parsable<I, E> for Box<T> {
  fn parse(input: I) -> IResult<I, Self, E> {
    T::parse.map(Box::new).parse(input)
  }
}
