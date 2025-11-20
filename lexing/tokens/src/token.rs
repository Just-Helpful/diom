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
  /* Brackets */
  /// A left parenthesis `(`
  LParen,
  /// A right parenthesis `)`
  RParen,
  /// A left curly bracket `{`
  LCurly,
  /// A right curly bracket `}`
  RCurly,
  /// A left square bracket `[`
  LBrace,
  /// A right square bracket `]`
  RBrace,

  /* Punctuation */
  /// A single dot `.` (for field / method access)
  Dot,
  /// A semicolon `;` (for line termination)
  Semi,
  /// A colon `:` (for struct fields)
  Colon,
  /// A comma `,` (for tuples / arrays)
  Comma,
  /// An equals sign `=` (for variable assignment)
  Assign,
  /// Ellipses `...` (for "rest" parameters)
  Ellipses,
  /// Function arrow `=>` (for anonymous functions)
  Function,

  /* Reserved keywords */
  /// A `let` keyword (for variable definition)
  Let,
  /// A `return` keyword (for early returns)
  Return,

  /* Operators */
  /// A not `!` operator
  Not,
  /// An and `&` operator
  And,
  /// An or `|` operator
  Or,
  /// A plus `+` operator
  Plus,
  /// A minus `-` operator
  Minus,
  /// A times `*` operator
  Times,
  /// A divide `/` operator
  Divide,
  /// An equality `==` operator
  Eq,
  /// An inverse equality `!=` operator
  Ne,
  /// A less than `<` operator
  Lt,
  /// A greater than `>` operator
  Gt,
  /// A less than / equal to `<=` operator
  LtEq,
  /// A greater than / equal to `>=` operator
  GtEq,
  /// A monadic application `?` operator
  Monad,
  // Literals
  /// A floating point value, i.e. `2.34`, `-0.02`
  Float(f64),
  /// A single character, i.e. `'x'`, `'\u+26c4'`
  Char(char),
  // String-like
  /// An identifier, i.e. `x`, `_`, `string_ident_0`
  StringIdent(Box<str>),
  /// A single / multiline comment
  Comment(Box<str>),
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
