use super::{Parsable, ParsableNum};
use crate::common::span::Span;
use nom::{
  bytes::complete::{is_a, tag, take_until1},
  character::{complete::char, is_digit, streaming::one_of},
  combinator::opt,
  error::Error,
  sequence::preceded,
  IResult,
};
use std::str::FromStr;

pub struct Int<T>(T);

fn parse_base(input: Span<&str>) -> IResult<Span<&str>, u8, Error<Span<&str>>> {
  let (input, base_str) = opt(preceded(char('0'), one_of("bBdDxX")))(input)?;
  let base = base_str.map(|c| match c {
    'b' | 'B' => 2,
    'd' | 'D' => 10,
    'x' | 'X' => 16,
  });
  Ok((input, base.unwrap_or(10)))
}

pub enum SignedError {
  I8(<i8 as FromStr>::Err),
  I16(<i16 as FromStr>::Err),
  I32(<i32 as FromStr>::Err),
  I64(<i64 as FromStr>::Err),
  I128(<i128 as FromStr>::Err),
}

impl<'a, T: ParsableNum> Parsable<Span<&'a str>> for Int<T> {
  fn parse(input: Span<&'a str>) -> IResult<Span<&'a str>, Self, Error<Span<&'a str>>> {
    let (input, base) = parse_base(input)?;
    let (input, _) = is_a("0")(input)?; // strip leading 0s

    let (input, digits) = opt(take_until1("i"))(input)?;
    let (input, _) = tag(T::IDENT)(input)?;
    let value = digits.unwrap_or("0".into()).parse()?;
    Ok((input, Int(value)))
  }
}
