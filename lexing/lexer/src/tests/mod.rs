pub mod proptests;

pub type LexError<'a> = nom::error::Error<&'a str>;
/// The result type used in testing
pub type TestResult<'a, T> = Result<T, nom::Err<LexError<'a>>>;
