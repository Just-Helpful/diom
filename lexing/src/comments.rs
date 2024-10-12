use super::{SResult, Span};
use diom_tokens::Token;
use nom::{
  branch::alt,
  bytes::complete::{is_not, tag, take_until},
  character::complete::{char, line_ending},
  sequence::delimited,
};

/// Parses a comment that spans until the end of the current line
/// ```_,ignore
/// # initialise loop index to 0
/// let i = 0;
/// ```
fn parse_line(input: Span) -> SResult<Token> {
  let (input, comment) = delimited(char('#'), is_not("\r\n"), line_ending)(input)?;
  Ok((input, Token::Comment(comment.into_fragment().into())))
}

/// Parses a comment that spans from an opening "bracket" to a closing "bracket"
/// potentially over multiple lines.
/// ```_,ignore
/// #(
/// The loop index starts at 0
/// and every loop iteration it will be incremented
/// the sum will be calculated up until the loop index  
/// )#
/// let i = 0;
/// ```
fn parse_block(input: Span) -> SResult<Token> {
  let (input, comment) = delimited(tag("#("), take_until(")#"), tag(")#"))(input)?;
  Ok((input, Token::Comment(comment.into_fragment().into())))
}

/// Parses either a `#` line comment, or a `#(` block comment `)#`
pub fn parse_comment(input: Span) -> SResult<Token> {
  alt((parse_block, parse_line))(input)
}

#[cfg(test)]
mod test {
  use super::{parse_comment, Token::*};
  use crate::tests::TResult;

  #[test]
  fn line() -> TResult<()> {
    let input = "\
    # initialise loop index to 0\n\
    let i = 0;\
    ";
    let (rest, comment) = parse_comment(input.into())?;
    assert_eq!(rest.into_fragment(), "let i = 0;");
    assert_eq!(comment, Comment(" initialise loop index to 0".into()));
    Ok(())
  }

  #[test]
  fn block() -> TResult<()> {
    let input = "\
    #(\n\
    The loop index starts at 0\n\
    and every loop iteration it will be incremented\n\
    the sum will be calculated up until the loop index\n\
    )#\n\
    let i = 0;\
    ";
    let (rest, comment) = parse_comment(input.into())?;
    assert_eq!(
      rest.into_fragment(),
      "\n\
      let i = 0;\
      "
    );
    assert_eq!(
      comment,
      Comment(
        "\n\
        The loop index starts at 0\n\
        and every loop iteration it will be incremented\n\
        the sum will be calculated up until the loop index\n\
        "
        .into()
      )
    );
    Ok(())
  }

  #[test]
  fn block_close() -> TResult<()> {
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
    let (rest, comment) = parse_comment(input.into())?;
    assert_eq!(
      rest.into_fragment(),
      "\n\
      log.info(\"end of comment\")\
      "
    );
    assert_eq!(
      comment,
      Comment(
        "\n\
        #(\n\
        )\n\
        \"\n\
        \'\n\
        "
        .into()
      )
    );
    Ok(())
  }
}
