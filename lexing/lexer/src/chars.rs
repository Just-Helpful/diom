//! Character parsers, mostly written from the [nom string example](https://github.com/rust-bakery/nom/blob/main/examples/string.rs)
use super::{helpers::span_wrap, SResult, Span};
use diom_tokens::{SpanToken, Token};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while_m_n},
  character::complete::{anychar, char, multispace1},
  combinator::{value, verify},
  error::{Error, ErrorKind},
  multi::many0,
  sequence::{delimited, preceded},
  Parser,
};
use std::iter::once;

/// Parses a unicode encoded character, of the form `u{XXXX}`
fn parse_unicode(input: Span) -> SResult<char> {
  let (input, _) = tag("u{")(input)?;
  let (input, hex) = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit())(input)?;
  let (input, _) = char('}')(input)?;

  let Ok(code) = u32::from_str_radix(hex.fragment(), 16) else {
    return Err(nom::Err::Error(Error::new(input, ErrorKind::MapRes)));
  };
  let Some(result) = char::from_u32(code) else {
    return Err(nom::Err::Error(Error::new(input, ErrorKind::MapOpt)));
  };
  Ok((input, result))
}

/// Parses an escaped character, of the form `\t`, `\n` or `\u{fe0e}`
fn parse_escaped(input: Span) -> SResult<char> {
  preceded(
    char('\\'),
    alt((
      parse_unicode,
      value('\n', char('n')),
      value('\r', char('r')),
      value('\t', char('t')),
      value('\u{08}', char('b')),
      value('\u{0C}', char('f')),
      value('\\', char('\\')),
    )),
  )(input)
}

/// Parses a newline escaped with a `\`
fn parse_escaped_eol(input: Span) -> SResult<()> {
  value((), preceded(char('\\'), multispace1))(input)
}

/// Parses a single character, using the syntax:
/// ```_,ignore
/// 'a';
/// '\
/// a';
/// '\t';
/// '\'';
/// '\n';
/// '\u{fe0e}';
/// ```
pub fn parse_char(input: Span) -> SResult<Token> {
  let parse_single = delimited(
    many0(parse_escaped_eol),
    alt((
      parse_escaped,
      // escaped single quotes, i.e. '\''
      value('\'', tag("\\'")),
      // anything that's not a single quote
      verify(anychar, |&c| c != '\''),
    ))
    .map(Token::Char),
    many0(parse_escaped_eol),
  );
  delimited(char('\''), parse_single, char('\''))(input)
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
pub fn parse_string(input: Span) -> SResult<Vec<SpanToken>> {
  let parse_single = alt((
    parse_escaped,
    // escaped double quotes, i.e. "\""
    value('"', tag("\\\"")),
    // anything that's not a double quote
    verify(anychar, |&c| c != '"'),
  ))
  .map(Token::Char);
  let mut parse_content = delimited(
    many0(parse_escaped_eol),
    many0(preceded(many0(parse_escaped_eol), span_wrap(parse_single))),
    many0(parse_escaped_eol),
  );

  let (input, lbrac) = span_wrap(char('"').map(|_| Token::LBrace))(input)?;
  let (input, chars) = parse_content(input)?;
  let (input, rbrac) = span_wrap(char('"').map(|_| Token::RBrace))(input)?;
  let char_tokens = chars.into_iter().flat_map(|c| {
    let comma = c.with_token(Token::Comma);
    [c, comma].into_iter()
  });

  Ok((
    input,
    once(lbrac).chain(char_tokens).chain(once(rbrac)).collect(),
  ))
}

#[cfg(test)]
mod test {
  use super::{parse_char, parse_string};
  use crate::{
    tests::{complete_parse, TResult},
    Token::{self, *},
  };

  mod char {
    use super::*;

    #[test]
    fn simple() -> TResult<()> {
      assert_eq!(complete_parse(parse_char, "'a'")?, Char('a'));
      Ok(())
    }

    #[test]
    fn escaped_eol() -> TResult<()> {
      let input = r"'\
      a'";
      assert_eq!(complete_parse(parse_char, input)?, Char('a'));
      Ok(())
    }

    #[test]
    fn escaped_multi() -> TResult<()> {
      let input = r"'\
      \
      a\
      '";
      assert_eq!(complete_parse(parse_char, input)?, Char('a'));
      Ok(())
    }

    #[test]
    fn tab() -> TResult<()> {
      assert_eq!(complete_parse(parse_char, r"'\t'")?, Char('\t'));
      Ok(())
    }

    #[test]
    fn escaped_quote() -> TResult<()> {
      assert_eq!(complete_parse(parse_char, r"'\''")?, Char('\''));
      Ok(())
    }

    #[test]
    fn newline() -> TResult<()> {
      assert_eq!(complete_parse(parse_char, r"'\n'")?, Char('\n'));
      Ok(())
    }

    #[test]
    fn unicode() -> TResult<()> {
      assert_eq!(complete_parse(parse_char, r"'\u{fe0e}'")?, Char('\u{fe0e}'));
      Ok(())
    }
  }

  mod string {
    use super::*;

    #[test]
    fn empty() -> TResult<()> {
      let res = complete_parse(parse_string, r#""""#)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![LBrace, RBrace]
      );
      Ok(())
    }

    #[test]
    fn only_escaped_eols() -> TResult<()> {
      let input = r#""\
      \
      ""#;
      let res = complete_parse(parse_string, input)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![LBrace, RBrace]
      );
      Ok(())
    }

    #[test]
    fn single() -> TResult<()> {
      let res = complete_parse(parse_string, r#""a""#)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![LBrace, Char('a'), Comma, RBrace]
      );
      Ok(())
    }

    #[test]
    fn multi() -> TResult<()> {
      let res = complete_parse(parse_string, r#""hello world""#)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![
          LBrace,
          Char('h'),
          Comma,
          Char('e'),
          Comma,
          Char('l'),
          Comma,
          Char('l'),
          Comma,
          Char('o'),
          Comma,
          Char(' '),
          Comma,
          Char('w'),
          Comma,
          Char('o'),
          Comma,
          Char('r'),
          Comma,
          Char('l'),
          Comma,
          Char('d'),
          Comma,
          RBrace
        ]
      );
      Ok(())
    }

    #[test]
    fn eols() -> TResult<()> {
      let res = complete_parse(parse_string, "\"foo\n bar\n\"")?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![
          LBrace,
          Char('f'),
          Comma,
          Char('o'),
          Comma,
          Char('o'),
          Comma,
          Char('\n'),
          Comma,
          Char(' '),
          Comma,
          Char('b'),
          Comma,
          Char('a'),
          Comma,
          Char('r'),
          Comma,
          Char('\n'),
          Comma,
          RBrace
        ]
      );
      Ok(())
    }

    #[test]
    fn escaped_eols() -> TResult<()> {
      let input = r#""\
      foo\
      bar\
      ""#;
      let res = complete_parse(parse_string, input)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![
          LBrace,
          Char('f'),
          Comma,
          Char('o'),
          Comma,
          Char('o'),
          Comma,
          Char('b'),
          Comma,
          Char('a'),
          Comma,
          Char('r'),
          Comma,
          RBrace,
        ]
      );
      Ok(())
    }

    #[test]
    fn tabs() -> TResult<()> {
      let res = complete_parse(parse_string, r#""\tfoo\tbar""#)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![
          LBrace,
          Char('\t'),
          Comma,
          Char('f'),
          Comma,
          Char('o'),
          Comma,
          Char('o'),
          Comma,
          Char('\t'),
          Comma,
          Char('b'),
          Comma,
          Char('a'),
          Comma,
          Char('r'),
          Comma,
          RBrace
        ]
      );
      Ok(())
    }

    #[test]
    fn quotes() -> TResult<()> {
      let res = complete_parse(parse_string, r#""\"hi\" \"hey\"""#)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![
          LBrace,
          Char('"'),
          Comma,
          Char('h'),
          Comma,
          Char('i'),
          Comma,
          Char('"'),
          Comma,
          Char(' '),
          Comma,
          Char('"'),
          Comma,
          Char('h'),
          Comma,
          Char('e'),
          Comma,
          Char('y'),
          Comma,
          Char('"'),
          Comma,
          RBrace,
        ]
      );
      Ok(())
    }

    #[test]
    fn unicode() -> TResult<()> {
      let res = complete_parse(parse_string, r#""snowman:\u{fe0e}""#)?;
      assert_eq!(
        Vec::from_iter(res.into_iter().map(Token::from)),
        vec![
          LBrace,
          Char('s'),
          Comma,
          Char('n'),
          Comma,
          Char('o'),
          Comma,
          Char('w'),
          Comma,
          Char('m'),
          Comma,
          Char('a'),
          Comma,
          Char('n'),
          Comma,
          Char(':'),
          Comma,
          Char('\u{fe0e}'),
          Comma,
          RBrace,
        ]
      );
      Ok(())
    }
  }
}
