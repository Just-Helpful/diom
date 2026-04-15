use crate::Ptr;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use diom_tokens::Token;
use proptest::{
  prelude::{Arbitrary, BoxedStrategy, Just, Strategy},
  prop_oneof,
};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// A literal alphanumeric name
pub type LitName = Ptr<str>;

/// An alphanumeric identifier for use in variable definitions and tags
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Ident<I> {
  #[map_ignore]
  pub name: LitName,
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
  pub fn any() -> impl Strategy<Value = Self> {
    "[_a-zA-Z][_a-zA-Z0-9]*".prop_map(|name| Ident {
      name: name.into(),
      info: (),
    })
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Symbol {
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

impl TryFrom<Token> for Symbol {
  type Error = ();
  fn try_from(value: Token) -> Result<Self, Self::Error> {
    match value {
      Token::Not => Ok(Self::Not),
      Token::And => Ok(Self::And),
      Token::Or => Ok(Self::Or),
      Token::Plus => Ok(Self::Plus),
      Token::Minus => Ok(Self::Minus),
      Token::Times => Ok(Self::Times),
      Token::Divide => Ok(Self::Divide),
      Token::Eq => Ok(Self::Eq),
      Token::Ne => Ok(Self::Ne),
      Token::Lt => Ok(Self::Lt),
      Token::Gt => Ok(Self::Gt),
      Token::LtEq => Ok(Self::LtEq),
      Token::GtEq => Ok(Self::GtEq),
      _ => Err(()),
    }
  }
}

impl From<Symbol> for Token {
  fn from(value: Symbol) -> Self {
    match value {
      Symbol::Not => Token::Not,
      Symbol::And => Token::And,
      Symbol::Or => Token::Or,
      Symbol::Plus => Token::Plus,
      Symbol::Minus => Token::Minus,
      Symbol::Times => Token::Times,
      Symbol::Divide => Token::Divide,
      Symbol::Eq => Token::Eq,
      Symbol::Ne => Token::Ne,
      Symbol::Lt => Token::Lt,
      Symbol::Gt => Token::Gt,
      Symbol::LtEq => Token::LtEq,
      Symbol::GtEq => Token::GtEq,
    }
  }
}

impl Symbol {
  pub fn any() -> impl Strategy<Value = Self> {
    prop_oneof![
      Just(Symbol::Not),
      Just(Symbol::And),
      Just(Symbol::Or),
      Just(Symbol::Plus),
      Just(Symbol::Minus),
      Just(Symbol::Times),
      Just(Symbol::Divide),
      Just(Symbol::Eq),
      Just(Symbol::Ne),
      Just(Symbol::Lt),
      Just(Symbol::Gt),
      Just(Symbol::LtEq),
      Just(Symbol::GtEq),
    ]
  }
}
impl Arbitrary for Symbol {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Op<I> {
  #[map_ignore]
  pub sym: Symbol,
  pub info: I,
}

impl<I> Display for Op<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Token::from(self.sym).fmt(f)
  }
}

impl DisplayAs<Spans> for Op<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("symbol", &self.info)
  }
}

impl Op<()> {
  pub fn any() -> impl Strategy<Value = Self> {
    Symbol::any().prop_map(|sym| Op { sym, info: () })
  }
}

/// A method name that can be either a literal name or a symbol
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Name {
  Literal(LitName),
  Symbol(Symbol),
}

impl TryFrom<Token> for Name {
  type Error = ();
  fn try_from(value: Token) -> Result<Self, Self::Error> {
    match value {
      Token::StringIdent(name) => Ok(Self::Literal(name.into())),
      tok => tok.try_into().map(Self::Symbol),
    }
  }
}
impl From<Name> for Token {
  fn from(value: Name) -> Self {
    match value {
      Name::Literal(name) => Token::StringIdent(name),
      Name::Symbol(sym) => sym.into(),
    }
  }
}
impl From<LitName> for Name {
  fn from(value: LitName) -> Self {
    Self::Literal(value)
  }
}
impl TryFrom<Name> for LitName {
  type Error = ();
  fn try_from(value: Name) -> Result<Self, Self::Error> {
    match value {
      Name::Literal(name) => Ok(name),
      Name::Symbol(_) => Err(()),
    }
  }
}
impl From<Symbol> for Name {
  fn from(value: Symbol) -> Self {
    Self::Symbol(value)
  }
}
impl TryFrom<Name> for Symbol {
  type Error = ();
  fn try_from(value: Name) -> Result<Self, Self::Error> {
    match value {
      Name::Symbol(sym) => Ok(sym),
      Name::Literal(_) => Err(()),
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
    let lit = r"[_a-zA-Z][_a-zA-Z0-9]*"
      .prop_map(|s| s.into_boxed_str())
      .prop_map(Name::Literal);
    prop_oneof![Symbol::any().prop_map(Self::Symbol), lit]
  }
}
impl Arbitrary for Name {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}

/// An identifier used for method names and operators
#[derive(Clone, InfoSource, InfoRef, InfoMap, PartialEq, Eq, Hash, Debug)]
pub struct Method<I> {
  #[map_ignore]
  pub name: Name,
  pub info: I,
}

impl<I> TryFrom<Method<I>> for Ident<I> {
  type Error = ();
  fn try_from(value: Method<I>) -> Result<Self, Self::Error> {
    Ok(Self {
      name: value.name.try_into()?,
      info: value.info,
    })
  }
}
impl<I> From<Ident<I>> for Method<I> {
  fn from(value: Ident<I>) -> Self {
    Self {
      name: value.name.into(),
      info: value.info,
    }
  }
}
impl<I> TryFrom<Method<I>> for Op<I> {
  type Error = ();
  fn try_from(value: Method<I>) -> Result<Self, Self::Error> {
    Ok(Self {
      sym: value.name.try_into()?,
      info: value.info,
    })
  }
}
impl<I> From<Op<I>> for Method<I> {
  fn from(value: Op<I>) -> Self {
    Self {
      name: value.sym.into(),
      info: value.info,
    }
  }
}

impl<I> Display for Method<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)
  }
}

impl DisplayAs<Spans> for Method<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("method", &self.info)
  }
}

impl Method<()> {
  /// Generates a generic strategy for generating `Ident`s
  pub fn any() -> impl Strategy<Value = Self> {
    Name::any().prop_map(|name| Method { name, info: () })
  }
}
impl Arbitrary for Method<()> {
  type Parameters = ();
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
    Self::any().boxed()
  }
}
