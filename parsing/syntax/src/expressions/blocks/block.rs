use super::Expression;
use crate::{
  display::Sep,
  types::{TypeConfig, TypeDef},
  Seq,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy, prop_oneof};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub enum Statement<I> {
  Expression(Expression<I>),
  TypeDef(TypeDef<I>),
}

impl<I> Display for Statement<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Expression(e) => e.fmt(f),
      Self::TypeDef(t) => t.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for Statement<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Expression(e) => e.write(w),
      Self::TypeDef(d) => d.write(w),
    }
  }
}

impl Statement<()> {
  /// Generates a generic strategy for generating `Statement` nodes
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: TypeConfig,
  ) -> impl Strategy<Value = Self> {
    prop_oneof![
      TypeDef::any(args).prop_map(Self::TypeDef),
      item.prop_map(Self::Expression),
    ]
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Block<I> {
  pub statements: Seq<Statement<I>>,
  pub info: I,
}

impl<I> Display for Block<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('(')?;
    Sep(&self.statements, ';').fmt(f)?;
    f.write_char(')')
  }
}

impl DisplayAs<Spans> for Block<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("block", &self.info)?;
    self.statements.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct BlockConfig(
  /// The config used to generate types within blocks
  pub TypeConfig,
  /// The maximum number of statements in a block
  pub usize,
);
impl Default for BlockConfig {
  fn default() -> Self {
    Self(Default::default(), 100)
  }
}
impl Block<()> {
  /// Generates a generic strategy for generating `Block` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: BlockConfig,
  ) -> impl Strategy<Value = Self> {
    vec(Statement::any(item, args.0), 0..args.1)
      .prop_map(Seq::from_iter)
      .prop_map(|statements| Block {
        statements,
        info: (),
      })
  }
}
