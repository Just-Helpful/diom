//! Character parsers, mostly written from the [nom string example](https://github.com/rust-bakery/nom/blob/main/examples/string.rs)
use super::{token::span_wrap, SResult, Span};
use crate::token::{SpanToken, Token};
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

fn parse_escaped_ws(input: Span) -> SResult<Span> {
  preceded(char('\\'), multispace1)(input)
}

/// Parses a single character, using the syntax
/// ```ignore
/// 'a';
/// '\
///    a';
/// '\t';
/// '\'';
/// '\n';
/// '\u{fe0e}';
/// ```
pub fn parse_char(input: Span) -> SResult<Token> {
  let parse_single = preceded(
    parse_escaped_ws,
    alt((
      parse_escaped,
      // escaped single quotes, i.e. '\''
      value('\'', tag("\\'")),
      // anything that's not a single quote
      verify(anychar, |&c| c != '\''),
    ))
    .map(Token::Char),
  );
  delimited(char('\''), parse_single, char('\''))(input)
}

/// Parses a string as an array of characters:
/// ```ignore
/// assert "hello world" == ['h','e','l','l','o',' ','w','o','r','l','d',];
/// assert "foo\
///   bar\
///
/// " == ['f','o','o','b','a','r',];
/// assert "foo\tbar" == ['f','o','o','\t','b','a','r',];
/// assert "\"hi\" \"hey\"" == ['"','h','i','"',' ','"','h','e','y','"',];
/// assert "snowman:\u{fe0e}" == ['s','n','o','w','m','a','n',':','\u{fe0e}',];
/// ```
pub fn parse_string(input: Span) -> SResult<Vec<SpanToken>> {
  let parse_single = preceded(
    parse_escaped_ws,
    alt((
      parse_escaped,
      // escaped double quotes, i.e. "\""
      value('"', tag("\\\"")),
      // anything that's not a double quote
      verify(anychar, |&c| c != '"'),
    ))
    .map(Token::Char),
  );

  let (input, lbrac) = span_wrap(char('"').map(|_| Token::LSquare))(input)?;
  let (input, chars) = many0(span_wrap(parse_single))(input)?;
  let (input, rbrac) = span_wrap(char('"').map(|_| Token::RSquare))(input)?;
  let char_tokens = chars.into_iter().flat_map(|c| {
    let comma = c.with_token(Token::Comma);
    [c, comma].into_iter()
  });

  Ok((
    input,
    once(lbrac).chain(char_tokens).chain(once(rbrac)).collect(),
  ))
}
