use nom::error::ParseError;

pub struct ParserError<I> {
  input: I,
}
