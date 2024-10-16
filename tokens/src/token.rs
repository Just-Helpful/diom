/// All possible tokens in the Diom language.
///
/// ## Naming
///
/// The naming here is meant to reflect how the tokens are **used**<br>
/// not the current string that parses to the token in `/lexing`.
///
/// If you can think of better names for these tokens based on **usage**<br>
/// then feel absolutely free to rename a variant, they are by no means<br>
/// final. If you do, make sure that tests **still pass**.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // Brackets
  LParen,
  RParen,
  LCurly,
  RCurly,
  LBrace,
  RBrace,
  // Punctuation
  Dot,
  Semi,
  Colon,
  Comma,
  Assign,
  Ellipses,
  Function,
  // Reserved keywords
  Let,
  Return,
  // Operators
  Not,
  And,
  Or,
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
  Monad,
  // String-like
  StringIdent(Box<str>),
  Char(char),
  Comment(Box<str>),
  // Value-like
  Float(f64),
}

impl Token {
  /// We often only care about the Token type, not about the internal data.<br>
  /// Therefore we implement a custom `matches` method for light equality.
  ///
  /// ```
  /// # use diom_tokens::Token::*;
  /// assert!(Dot.matches(&Dot));
  /// assert!(Char('a').matches(&Char('b')));
  /// assert!(Float(2.5).matches(&Float(3e-4)));
  /// assert!(StringIdent("foo".into()).matches(&StringIdent("bar".into())));
  /// ```
  pub fn matches(&self, other: &Token) -> bool {
    use Token::*;
    matches! {
      (self, other),
      (LParen, LParen)
        | (RParen, RParen)
        | (LCurly, LCurly)
        | (RCurly, RCurly)
        | (LBrace, LBrace)
        | (RBrace, RBrace)
        | (Dot, Dot)
        | (Semi, Semi)
        | (Colon, Colon)
        | (Comma, Comma)
        | (Assign, Assign)
        | (Ellipses, Ellipses)
        | (Let, Let)
        | (Not, Not)
        | (And, And)
        | (Or, Or)
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
        | (Monad, Monad)
        | (StringIdent(_), StringIdent(_))
        | (Char(_), Char(_))
        | (Comment(_), Comment(_))
        | (Float(_), Float(_))
    }
  }
}

impl AsRef<Token> for Token {
  fn as_ref(&self) -> &Token {
    self
  }
}
