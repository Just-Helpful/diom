//! Character parsers, mostly written from the [nom string example](https://github.com/rust-bakery/nom/blob/main/examples/string.rs)
use diom_tokens::{SpanToken, Token};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while_m_n},
  character::complete::{char, multispace1, none_of},
  combinator::{consumed, value},
  error::Error,
  multi::many0,
  sequence::{delimited, preceded, terminated},
  IResult, Parser,
};

/// Parses a unicode encoded character, of the form `u{XXXX}`
fn unicode_char<'a>() -> impl Parser<&'a str, Output = char, Error = Error<&'a str>> {
  let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());
  let parse_full = delimited(tag("u{"), parse_hex, char::<&str, Error<&str>>('}'));

  parse_full
    .map_res(|hex| u32::from_str_radix(hex, 16))
    .map_opt(char::from_u32)
}

/// Parses an escaped character, of the form `\t`, `\n` or `\u{fe0e}`
fn escaped_char<'a>() -> impl Parser<&'a str, Output = char, Error = Error<&'a str>> {
  let parse_datum = alt((
    unicode_char(),
    value('\n', char('n')),
    value('\r', char('r')),
    value('\t', char('t')),
    value('\u{08}', char('b')),
    value('\u{0C}', char('f')),
    value('\\', char('\\')),
  ));

  preceded(char('\\'), parse_datum)
}

/// Parses whitespace escaped with a `\`
fn escaped_eol<'a>() -> impl Parser<&'a str, Output = (), Error = Error<&'a str>> {
  value((), preceded(char('\\'), multispace1))
}

/// Parses a single character, using the syntax:
/// ```_
/// 'a';
/// '\
/// a';
/// '\t';
/// '\'';
/// '\n';
/// '\u{fe0e}';
/// ```
pub fn enclosed_char<'a>() -> impl Parser<&'a str, Output = char, Error = Error<&'a str>> {
  let parse_contents = delimited(
    many0(escaped_eol()),
    alt((
      escaped_char(),
      // escaped single quotes, i.e. '\''
      value('\'', tag("\\'")),
      // anything that's not a single quote
      none_of("'"),
    )),
    many0(escaped_eol()),
  );

  delimited(char('\''), parse_contents, char('\''))
}

/// Parses a string as an array of characters:
/// ```_,ignore
/// assert "hello world" == ['h','e','l','l','o',' ','w','o','r','l','d',];
/// assert "foo\
///  bar\
/// " == ['f','o','o',' ',' ','b','a','r',];
/// assert "\tfoo\tbar" == ['f','o','o','\t','b','a','r',];
/// assert "\"hi\" \"hey\"" == ['"','h','i','"',' ','"','h','e','y','"',];
/// assert "snowman:\u{fe0e}" == ['s','n','o','w','m','a','n',':','\u{fe0e}',];
/// ```
pub fn parse_string<'a>(input: &str) -> IResult<&str, Vec<char>> {
  let parse_single = alt((
    escaped_char(),
    // escaped double quotes, i.e. "\""
    value('"', tag("\\\"")),
    // anything that's not a double quote
    none_of("\""),
  ));

  let parse_content = terminated(
    many0(preceded(many0(escaped_eol()), parse_single)),
    many0(escaped_eol()),
  );

  delimited(char('"'), parse_content, char('"')).parse(input)
}

pub fn parse_span_string(input: &str) -> IResult<&str, Vec<SpanToken<'_>>> {
  let parse_single = consumed(alt((
    escaped_char(),
    value('"', tag("\\\"")),
    none_of("\""),
  )))
  .map(|(origin, ch)| SpanToken {
    token: Token::Char(ch),
    origin,
  });

  let parse_content = terminated(
    many0(preceded(many0(escaped_eol()), parse_single)),
    many0(escaped_eol()),
  );

  delimited(char('"'), parse_content, char('"')).parse(input)
}

#[cfg(test)]
mod test {
  use super::{enclosed_char, parse_string};
  use crate::tests::TestResult;

  mod char {
    use super::*;
    use nom::{combinator::all_consuming, Parser};

    #[test]
    fn simple() -> TestResult<'static, ()> {
      assert_eq!(all_consuming(enclosed_char()).parse("'a'")?, ("", 'a'));
      Ok(())
    }

    #[test]
    fn escaped_eol() -> TestResult<'static, ()> {
      let input = r"'\
      a'";
      assert_eq!(all_consuming(enclosed_char()).parse(input)?, ("", 'a'));
      Ok(())
    }

    #[test]
    fn escaped_multi() -> TestResult<'static, ()> {
      let input = r"'\
      \
      a\
      '";
      assert_eq!(all_consuming(enclosed_char()).parse(input)?, ("", 'a'));
      Ok(())
    }

    #[test]
    fn tab() -> TestResult<'static, ()> {
      assert_eq!(all_consuming(enclosed_char()).parse(r"'\t'")?, ("", '\t'));
      Ok(())
    }

    #[test]
    fn escaped_quote() -> TestResult<'static, ()> {
      assert_eq!(all_consuming(enclosed_char()).parse(r"'\''")?, ("", '\''));
      Ok(())
    }

    #[test]
    fn newline() -> TestResult<'static, ()> {
      assert_eq!(all_consuming(enclosed_char()).parse(r"'\n'")?, ("", '\n'));
      Ok(())
    }

    #[test]
    fn unicode() -> TestResult<'static, ()> {
      assert_eq!(
        all_consuming(enclosed_char()).parse(r"'\u{fe0e}'")?,
        ("", '\u{fe0e}')
      );
      Ok(())
    }
  }

  mod string {
    use super::*;
    use nom::{combinator::all_consuming, Parser};

    #[test]
    fn empty() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""""#)?;
      assert_eq!(res, vec![]);
      Ok(())
    }

    #[test]
    fn only_escaped_eols() -> TestResult<'static, ()> {
      let input = r#""\
      \
      ""#;
      let (_, res) = all_consuming(parse_string).parse(input)?;
      assert_eq!(res, vec![]);
      Ok(())
    }

    #[test]
    fn single() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""a""#)?;
      assert_eq!(res, vec!['a']);
      Ok(())
    }

    #[test]
    fn multi() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""hello world""#)?;
      assert_eq!(
        res,
        vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd',]
      );
      Ok(())
    }

    #[test]
    fn eols() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""foo\n bar\n""#)?;
      assert_eq!((res), vec!['f', 'o', 'o', '\n', ' ', 'b', 'a', 'r', '\n',]);
      Ok(())
    }

    #[test]
    fn escaped_eols() -> TestResult<'static, ()> {
      let input = r#""\
      foo\
      bar\
      ""#;
      let (_, res) = all_consuming(parse_string).parse(input)?;
      assert_eq!(res, vec!['f', 'o', 'o', 'b', 'a', 'r',]);
      Ok(())
    }

    #[test]
    fn tabs() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""\tfoo\tbar""#)?;
      assert_eq!(res, vec!['\t', 'f', 'o', 'o', '\t', 'b', 'a', 'r',]);
      Ok(())
    }

    #[test]
    fn quotes() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""\"hi\" \"hey\"""#)?;
      assert_eq!(res, vec!['"', 'h', 'i', '"', ' ', '"', 'h', 'e', 'y', '"',]);
      Ok(())
    }

    #[test]
    fn unicode() -> TestResult<'static, ()> {
      let (_, res) = all_consuming(parse_string).parse(r#""snowman:\u{fe0e}""#)?;
      assert_eq!(
        res,
        vec!['s', 'n', 'o', 'w', 'm', 'a', 'n', ':', '\u{fe0e}',]
      );
      Ok(())
    }
  }
}
