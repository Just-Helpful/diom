use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Formatter, Write},
  ops::Range,
};

mod chars;
pub use chars::Char;
mod floats;
pub use floats::Float;

mod access;
pub use access::{Call, Field, Index, Infix, MonadResult, MonadThen};
mod blocks;
pub use blocks::{Assign, Block, Declare, Group, Return, Statement};
mod compound;
pub use compound::{Array, Function, FunctionArm, Parameter, Parameters, Struct};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum Expression<I> {
  /* base values in the language */
  Char(Char<I>),
  Float(Float<I>),
  Var(Ident<I>),
  /* block expressions */
  Group(Group<I>),
  Block(Block<I>),
  Assign(Assign<I>),
  Declare(Declare<I>),
  Return(Return<I>),
  /* compound values in the language */
  Array(Array<I>),
  Function(Function<I>),
  Struct(Struct<I>),
  /* accessing values in the language */
  Call(Call<I>),
  Field(Field<I>),
  Index(Index<I>),
  Infix(Infix<I>),
  Monad(MonadThen<I>),
  Result(MonadResult<I>),
}

impl<I> Display for Expression<I> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    use Expression::*;
    match self {
      Char(c) => c.fmt(f),
      Float(v) => v.fmt(f),
      Var(v) => v.fmt(f),
      //
      Group(g) => g.fmt(f),
      Block(b) => b.fmt(f),
      Assign(a) => a.fmt(f),
      Declare(d) => d.fmt(f),
      Return(r) => r.fmt(f),
      //
      Array(a) => a.fmt(f),
      Function(v) => v.fmt(f),
      Struct(s) => s.fmt(f),
      //
      Call(c) => c.fmt(f),
      Field(v) => v.fmt(f),
      Index(i) => i.fmt(f),
      Infix(i) => i.fmt(f),
      Monad(m) => m.fmt(f),
      Result(r) => r.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for Expression<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    use Expression::*;
    match self {
      Char(c) => c.write(w),
      Float(f) => f.write(w),
      Var(v) => v.write(w),
      //
      Group(g) => g.write(w),
      Block(b) => b.write(w),
      Assign(a) => a.write(w),
      Declare(d) => d.write(w),
      Return(r) => r.write(w),
      //
      Array(a) => a.write(w),
      Function(f) => f.write(w),
      Struct(s) => s.write(w),
      //
      Call(c) => c.write(w),
      Field(f) => f.write(w),
      Index(i) => i.write(w),
      Infix(i) => i.write(w),
      Monad(m) => m.write(w),
      Result(r) => r.write(w),
    }
  }
}
