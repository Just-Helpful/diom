use crate::{SpanToken, SpanTokens, Token};
use std::fmt::{Display, Write};

#[cfg(feature = "pretty")]
mod pretty {
  use super::Token;
  use colorz::{ansi, Colorize, Style};
  use std::fmt::Display;

  pub const BRACKET_STYLE: Style = Style::new().const_into_runtime_style();
  pub const PUNCTUATION_STYLE: Style = Style::new().const_into_runtime_style();
  pub const KEYWORD_STYLE: Style = Style::new().fg(ansi::Magenta).const_into_runtime_style();
  pub const OPERATOR_STYLE: Style = Style::new().const_into_runtime_style();
  pub const LITERAL_STYLE: Style = Style::new().const_into_runtime_style();
  pub const IDENTIFIER_STYLE: Style = Style::new().const_into_runtime_style();
  pub const COMMENT_STYLE: Style = Style::new().fg(ansi::White).const_into_runtime_style();

  pub fn display(tok: &Token, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match tok {
      /* Brackets */
      Token::LParen => '('.style_with(BRACKET_STYLE).fmt(f),
      Token::RParen => ')'.style_with(BRACKET_STYLE).fmt(f),
      Token::LCurly => '{'.style_with(BRACKET_STYLE).fmt(f),
      Token::RCurly => '}'.style_with(BRACKET_STYLE).fmt(f),
      Token::LBrace => '['.style_with(BRACKET_STYLE).fmt(f),
      Token::RBrace => ']'.style_with(BRACKET_STYLE).fmt(f),

      /* Punctuation */
      Token::Dot => '.'.style_with(PUNCTUATION_STYLE).fmt(f),
      Token::Semi => ';'.style_with(PUNCTUATION_STYLE).fmt(f),
      Token::Colon => ':'.style_with(PUNCTUATION_STYLE).fmt(f),
      Token::Comma => ','.style_with(PUNCTUATION_STYLE).fmt(f),
      Token::Assign => '='.style_with(PUNCTUATION_STYLE).fmt(f),
      Token::Ellipses => "...".style_with(PUNCTUATION_STYLE).fmt(f),
      Token::Function => "=>".style_with(PUNCTUATION_STYLE).fmt(f),

      /* Reserved keywords */
      Token::Let => "let".style_with(KEYWORD_STYLE).fmt(f),
      Token::Type => "type".style_with(KEYWORD_STYLE).fmt(f),
      Token::Return => "return".style_with(KEYWORD_STYLE).fmt(f),

      /* Operators */
      Token::Not => '!'.style_with(OPERATOR_STYLE).fmt(f),
      Token::And => '&'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Or => '|'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Plus => '+'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Minus => '-'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Times => '*'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Divide => '/'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Eq => "==".style_with(OPERATOR_STYLE).fmt(f),
      Token::Ne => "!=".style_with(OPERATOR_STYLE).fmt(f),
      Token::Lt => '<'.style_with(OPERATOR_STYLE).fmt(f),
      Token::Gt => '>'.style_with(OPERATOR_STYLE).fmt(f),
      Token::LtEq => "<=".style_with(OPERATOR_STYLE).fmt(f),
      Token::GtEq => ">=".style_with(OPERATOR_STYLE).fmt(f),
      Token::Monad => '?'.style_with(OPERATOR_STYLE).fmt(f),

      /* Literals */
      Token::Float(value) => value.style_with(LITERAL_STYLE).fmt(f),
      Token::Char(chr) => chr.style_with(LITERAL_STYLE).fmt(f),

      /* String-like */
      Token::StringIdent(ident) => ident.style_with(IDENTIFIER_STYLE).fmt(f),
      Token::Comment(com) => com.style_with(COMMENT_STYLE).fmt(f),
    }
  }
}

fn display(tok: &Token, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  match tok {
    /* Brackets */
    Token::LParen => f.write_char('('),
    Token::RParen => f.write_char(')'),
    Token::LCurly => f.write_char('{'),
    Token::RCurly => f.write_char('}'),
    Token::LBrace => f.write_char('['),
    Token::RBrace => f.write_char(']'),

    /* Punctuation */
    Token::Dot => f.write_char('.'),
    Token::Semi => f.write_char(';'),
    Token::Colon => f.write_char(':'),
    Token::Comma => f.write_char(','),
    Token::Assign => f.write_char('='),
    Token::Ellipses => f.write_str("..."),
    Token::Function => f.write_str("=>"),

    /* Reserved keywords */
    Token::Let => f.write_str("let"),
    Token::Type => f.write_str("type"),
    Token::Return => f.write_str("return"),

    /* Operators */
    Token::Not => f.write_char('!'),
    Token::And => f.write_char('&'),
    Token::Or => f.write_char('|'),
    Token::Plus => f.write_char('+'),
    Token::Minus => f.write_char('-'),
    Token::Times => f.write_char('*'),
    Token::Divide => f.write_char('/'),
    Token::Eq => f.write_str("=="),
    Token::Ne => f.write_str("!="),
    Token::Lt => f.write_char('<'),
    Token::Gt => f.write_char('>'),
    Token::LtEq => f.write_str("<="),
    Token::GtEq => f.write_str(">="),
    Token::Monad => f.write_char('?'),

    /* Literals */
    Token::Float(value) => value.fmt(f),
    Token::Char(chr) => chr.fmt(f),

    /* String-like */
    Token::StringIdent(ident) => ident.fmt(f),
    Token::Comment(com) => com.fmt(f),
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    #[cfg(feature = "pretty")]
    if f.alternate() {
      return pretty::display(self, f);
    }
    display(self, f)
  }
}

impl<'a> Display for SpanToken<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.token.fmt(f)
  }
}

impl<'a> Display for SpanTokens<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut iter = self.iter();
    let Some(tok) = iter.next() else {
      return Ok(());
    };

    tok.fmt(f)?;
    for tok in iter {
      f.write_char(' ')?;
      tok.fmt(f)?;
    }
    Ok(())
  }
}
