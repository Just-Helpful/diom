use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use diom_tokens::Token;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Name {
  Literal(Box<str>),
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
}

impl TryFrom<Token> for Name {
  type Error = ();
  fn try_from(value: Token) -> Result<Self, Self::Error> {
    use Name::*;
    match value {
      Token::StringIdent(name) => Ok(Literal(name)),
      Token::Not => Ok(Not),
      Token::And => Ok(And),
      Token::Or => Ok(Or),
      Token::Plus => Ok(Plus),
      Token::Minus => Ok(Minus),
      Token::Times => Ok(Times),
      Token::Divide => Ok(Divide),
      Token::Eq => Ok(Eq),
      Token::Ne => Ok(Ne),
      Token::Lt => Ok(Lt),
      Token::Gt => Ok(Gt),
      Token::LtEq => Ok(LtEq),
      Token::GtEq => Ok(GtEq),
      _ => Err(()),
    }
  }
}
impl From<Name> for Token {
  fn from(value: Name) -> Self {
    use Name::*;
    match value {
      Literal(name) => Token::StringIdent(name),
      Not => Token::Not,
      And => Token::And,
      Or => Token::Or,
      Plus => Token::Plus,
      Minus => Token::Minus,
      Times => Token::Times,
      Divide => Token::Divide,
      Eq => Token::Eq,
      Ne => Token::Ne,
      Lt => Token::Lt,
      Gt => Token::Gt,
      LtEq => Token::LtEq,
      GtEq => Token::GtEq,
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, PartialEq, Eq, Hash, Debug)]
pub struct Ident<I> {
  #[map_ignore]
  pub name: Name,
  pub info: I,
}

impl DisplayAs<Spans> for Ident<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("ident", &self.info)
  }
}
