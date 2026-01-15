use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Write, ops::Range};

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
