use std::ops::Range;

use derivative::Derivative;

#[derive(Debug, Clone)]
pub enum Token {
  // Brackets
  LParen,
  RParen,
  LCurly,
  RCurly,
  LSquare,
  RSquare,
  // Punctuation
  Eof,
  Dot,
  Semi,
  Colon,
  Comma,
  Assign,
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
  String(Box<str>),
  Comment(Box<str>),
  // Value-like
  Float(Box<str>, f64),
}

impl PartialEq<Token> for Token {
  /// We only care about the type of the Token, not about the internal data.
  /// Therefore we implement a custom PartialEq type for equality.
  fn eq(&self, other: &Token) -> bool {
    use Token::*;
    matches!(
      (self, other),
      (LParen, LParen)
        | (RParen, RParen)
        | (LCurly, LCurly)
        | (RCurly, RCurly)
        | (LSquare, LSquare)
        | (RSquare, RSquare)
        | (Eof, Eof)
        | (Dot, Dot)
        | (Semi, Semi)
        | (Colon, Colon)
        | (Comma, Comma)
        | (Assign, Assign)
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
        | (String(_), String(_))
        | (Comment(_), Comment(_))
        | (Float(_, _), Float(_, _))
    )
  }
}
impl Eq for Token {}

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct SpanToken {
  pub token: Token,
  #[derivative(PartialEq = "ignore")]
  pub span: Range<usize>,
}

impl From<Token> for SpanToken {
  fn from(token: Token) -> Self {
    SpanToken { token, span: 0..0 }
  }
}
