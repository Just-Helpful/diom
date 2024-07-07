use std::ops::{Deref, Range};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // Brackets
  LParen,
  RParen,
  LCurly,
  RCurly,
  LSquare,
  RSquare,
  // Punctuation
  Dot,
  Semi,
  Colon,
  Comma,
  Assign,
  Ellipses,
  // Reserved keywords
  Let,
  Return,
  // Operators
  Not,
  Plus,
  Minus,
  Times,
  Divide,
  Eq,
  Ne,
  Lt,
  Gt,
  LtEq,
  GtEq,
  // String-like
  Ident(Box<str>),
  Char(char),
  Comment(Box<str>),
  // Value-like
  Float(f64),
}

impl Token {
  /// We often only care about the Token type, not about the internal data.
  /// Therefore we implement a custom PartialEq type for equality.
  pub fn matches(&self, other: &Token) -> bool {
    use Token::*;
    matches!(
      (self, other),
      (LParen, LParen)
        | (RParen, RParen)
        | (LCurly, LCurly)
        | (RCurly, RCurly)
        | (LSquare, LSquare)
        | (RSquare, RSquare)
        | (Dot, Dot)
        | (Semi, Semi)
        | (Colon, Colon)
        | (Comma, Comma)
        | (Assign, Assign)
        | (Ellipses, Ellipses)
        | (Let, Let)
        | (Not, Not)
        | (Plus, Plus)
        | (Minus, Minus)
        | (Times, Times)
        | (Divide, Divide)
        | (Eq, Eq)
        | (Ne, Ne)
        | (Lt, Lt)
        | (Gt, Gt)
        | (LtEq, LtEq)
        | (GtEq, GtEq)
        | (Ident(_), Ident(_))
        | (Char(_), Char(_))
        | (Comment(_), Comment(_))
        | (Float(_), Float(_))
    )
  }
}

impl AsRef<Token> for Token {
  fn as_ref(&self) -> &Token {
    self
  }
}

#[derive(Debug, Clone)]
pub struct SpanToken {
  pub token: Token,
  pub span: Range<usize>,
}

impl SpanToken {
  pub fn with_token(&self, token: Token) -> Self {
    Self {
      token,
      span: self.span.clone(),
    }
  }
}

impl From<Token> for SpanToken {
  fn from(token: Token) -> Self {
    SpanToken { token, span: 0..0 }
  }
}

impl Deref for SpanToken {
  type Target = Token;
  fn deref(&self) -> &Self::Target {
    &self.token
  }
}
