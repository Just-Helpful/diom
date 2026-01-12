use std::ops::Range;

use crate::{fmt::OptionsDisplay, ident::Ident};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
mod chars;
pub use chars::Char;
mod floats;
pub use floats::Float;

mod access;
pub use access::{Call, Field, Index, Infix, MonadThen};
mod blocks;
pub use blocks::{Assign, Block, Declare, Group, Return, Statement};
mod compound;
pub use compound::{Argument, Array, Function, FunctionArm, Struct};

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
}

impl OptionsDisplay for Expression<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      Self::Char(c) => c.optn_fmt(w, depth),
      Self::Float(f) => f.optn_fmt(w, depth),
      Self::Var(v) => v.optn_fmt(w, depth),
      //
      Self::Group(g) => g.optn_fmt(w, depth),
      Self::Block(b) => b.optn_fmt(w, depth),
      Self::Assign(a) => a.optn_fmt(w, depth),
      Self::Declare(d) => d.optn_fmt(w, depth),
      Self::Return(r) => r.optn_fmt(w, depth),
      //
      Self::Array(a) => a.optn_fmt(w, depth),
      Self::Function(f) => f.optn_fmt(w, depth),
      Self::Struct(s) => s.optn_fmt(w, depth),
      //
      Self::Call(c) => c.optn_fmt(w, depth),
      Self::Field(f) => f.optn_fmt(w, depth),
      Self::Index(i) => i.optn_fmt(w, depth),
      Self::Infix(i) => i.optn_fmt(w, depth),
      Self::Monad(m) => m.optn_fmt(w, depth),
    }
  }
}
