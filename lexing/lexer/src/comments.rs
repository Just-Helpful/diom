use nom::{
  branch::alt,
  bytes::complete::{is_not, tag, take_until},
  character::complete::{char, line_ending},
  error::Error,
  sequence::delimited,
  Parser,
};

/// Parses a comment that spans until the end of the current line
/// ```_
/// # initialise loop index to 0
/// let i = 0;
/// ```
fn line_comment<'a>() -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
  delimited(char('#'), is_not("\r\n"), line_ending)
}

/// Parses a comment that spans from an opening "bracket" to a closing "bracket"
/// potentially over multiple lines.
/// ```_
/// #(
/// The loop index starts at 0
/// and every loop iteration it will be incremented
/// the sum will be calculated up until the loop index  
/// )#
/// let i = 0;
/// ```
fn parse_block<'a>() -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
  delimited(tag("#("), take_until(")#"), tag(")#"))
}

/// Parses either a `# line comment`, or a `#( block comment )#`
pub fn parse_comment<'a>() -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
  alt((parse_block(), line_comment()))
}

#[cfg(test)]
mod test {
  use super::parse_comment;
  use crate::tests::TestResult;
  use nom::Parser;

  #[test]
  fn line() -> TestResult<'static, ()> {
    let input = "\
    # initialise loop index to 0\n\
    let i = 0;\
    ";
    let (rest, comment) = parse_comment().parse(input)?;
    assert_eq!(rest, "let i = 0;");
    assert_eq!(comment, " initialise loop index to 0");
    Ok(())
  }

  #[test]
  fn block() -> TestResult<'static, ()> {
    let input = "\
    #(\n\
    The loop index starts at 0\n\
    and every loop iteration it will be incremented\n\
    the sum will be calculated up until the loop index\n\
    )#\n\
    let i = 0;\
    ";
    let (rest, comment) = parse_comment().parse(input)?;
    assert_eq!(
      rest,
      "\n\
      let i = 0;\
      "
    );
    assert_eq!(
      comment,
      "\n\
        The loop index starts at 0\n\
        and every loop iteration it will be incremented\n\
        the sum will be calculated up until the loop index\n\
        "
    );
    Ok(())
  }

  #[test]
  fn block_close() -> TestResult<'static, ()> {
    // attempt a few character combinations to break the block comment
    let input = "\
    #(\n\
    #(\n\
    )\n\
    \"\n\
    \'\n\
    )#\n\
    log.info(\"end of comment\")\
    ";
    let (rest, comment) = parse_comment().parse(input)?;
    assert_eq!(
      rest,
      "\n\
      log.info(\"end of comment\")\
      "
    );
    assert_eq!(
      comment,
      "\n\
        #(\n\
        )\n\
        \"\n\
        \'\n\
        "
    );
    Ok(())
  }
}
