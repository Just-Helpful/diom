use std::ops::Range;

use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use crate::fmt::{bracket, OptionsDisplay};

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

impl OptionsDisplay for Ident<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("ident", self.info.len()));
    Ok(())
  }
}
