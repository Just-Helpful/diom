use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

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

impl CustomDisplay<SpanWriter> for Ident<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("ident", &self.info)
  }
}
