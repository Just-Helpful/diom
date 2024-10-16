use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Ident<I> {
  #[map_ignore]
  pub name: Name,
  pub info: I,
}
