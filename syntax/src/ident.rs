use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone)]
pub enum Name {
  Literal(Box<str>),
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
}

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Ident<I> {
  #[map_ignore]
  pub name: Name,
  pub info: I,
}
