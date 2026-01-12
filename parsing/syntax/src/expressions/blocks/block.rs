use super::Expression;
use crate::fmt::{bracket, OptionsDisplay};
use crate::types::TypeDef;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum Statement<I> {
  Expression(Expression<I>),
  TypeDef(TypeDef<I>),
}

impl OptionsDisplay for Statement<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      Statement::Expression(e) => e.optn_fmt(w, depth),
      Statement::TypeDef(t) => t.optn_fmt(w, depth),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Block<I> {
  pub statements: Vec<Statement<I>>,
  pub info: I,
}

impl OptionsDisplay for Block<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("block", self.info.len()));
    for stmt in &self.statements {
      stmt.optn_fmt(w, depth + 1)?;
    }
    Ok(())
  }
}
