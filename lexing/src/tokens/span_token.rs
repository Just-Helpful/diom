use super::Token;
use std::ops::{Deref, Range};

#[derive(Debug, Clone, PartialEq)]
pub struct SpanToken {
  pub token: Token,
  pub span: Range<usize>,
}

impl SpanToken {
  /// Returns a spanned token with the same span, but a different token.
  ///
  /// ```
  /// # use diom_lexing::{SpanToken, Token::*};
  /// let lbrac = SpanToken { token: LParen, span: 4..5 };
  /// assert_eq!(
  ///   lbrac.with_token(LCurly),
  ///   SpanToken { token: LCurly, span: 4..5 }
  /// );
  /// assert_eq!(
  ///   lbrac.with_token(Comma),
  ///   SpanToken { token: Comma, span: 4..5 }
  /// );
  /// ```
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

/// Simple unwrapping of span tokens
/// ```
/// # use diom_lexing::{Token, Token::*, SpanToken};
/// let token = Comma;
/// let s_token: SpanToken = token.clone().into();
/// let n_token: Token = s_token.into();
/// assert_eq!(n_token, token)
/// ```
impl From<SpanToken> for Token {
  fn from(val: SpanToken) -> Self {
    val.token
  }
}

/// Allows usage of `matches` on `SpanToken`s
/// ```
/// # use diom_lexing::{SpanToken, Token::*};
/// let token1 = SpanToken::from(Ident("foo".into()));
/// let token2 = SpanToken::from(Ident("bar".into()));
/// assert!(token1.matches(&token2));
/// ```
impl Deref for SpanToken {
  type Target = Token;
  fn deref(&self) -> &Self::Target {
    &self.token
  }
}
