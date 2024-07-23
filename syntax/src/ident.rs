use crate::InfoSource;

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

#[derive(InfoSource, Clone)]
pub struct Ident<I> {
  pub name: Name,
  pub info: I,
}
