use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use diom_tokens::Token;
use proptest::{
  prelude::{any, Arbitrary, BoxedStrategy, Just, Strategy},
  prop_oneof,
};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

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
impl Display for Name {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Token::from(self.clone()).fmt(f)
  }
}

impl Name {
  /// Generates a generic strategy for generating `Name`s
  pub fn any() -> impl Strategy<Value = Self> {
    let lit = any::<String>()
      .prop_map(|s| s.into_boxed_str())
      .prop_map(Name::Literal);
    prop_oneof![
      Just(Name::Not),
      Just(Name::And),
      Just(Name::Or),
      Just(Name::Plus),
      Just(Name::Minus),
      Just(Name::Times),
      Just(Name::Divide),
      Just(Name::Eq),
      Just(Name::Ne),
      Just(Name::Lt),
      Just(Name::Gt),
      Just(Name::LtEq),
      Just(Name::GtEq),
      lit,
    ]
  }
}
impl Arbitrary for Name {
  type Parameters = ();
  type Strategy = BoxedStrategy<Name>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, PartialEq, Eq, Hash, Debug)]
pub struct Ident<I> {
  #[map_ignore]
  pub name: Name,
  pub info: I,
}

impl<I> Display for Ident<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)
  }
}

impl DisplayAs<Spans> for Ident<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("ident", &self.info)
  }
}

impl Ident<()> {
  /// Generates a generic strategy for generating `Ident`s
  pub fn any() -> impl Strategy<Value = Self> {
    Name::any().prop_map(|name| Ident { name, info: () })
  }
}
impl Arbitrary for Ident<()> {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}
