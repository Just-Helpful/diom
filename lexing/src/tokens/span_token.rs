use super::Token;
use std::ops::{Deref, Range};

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
