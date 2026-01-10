use nom::error::Error;

/// The result type used in testing
pub type TestResult<'a, T> = Result<T, nom::Err<Error<&'a str>>>;
