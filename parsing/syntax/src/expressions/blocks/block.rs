use super::Expression;
use crate::fmt::{bracket, MultiDisplay};
use crate::types::TypeDef;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum Statement<I> {
  Expression(Expression<I>),
  TypeDef(TypeDef<I>),
}

impl MultiDisplay for Statement<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      Statement::Expression(e) => e.multi_fmt(w, depth),
      Statement::TypeDef(t) => t.multi_fmt(w, depth),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Block<I> {
  pub statements: Vec<Statement<I>>,
  pub info: I,
}

impl MultiDisplay for Block<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("block", self.info.len()));
    for stmt in &self.statements {
      stmt.multi_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
