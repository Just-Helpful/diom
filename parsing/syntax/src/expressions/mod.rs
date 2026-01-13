use std::ops::Range;

use crate::{
  fmt::{CustomDisplay, SpanWriter},
  ident::Ident,
};
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

impl CustomDisplay<SpanWriter> for Expression<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    match self {
      Self::Char(c) => c.write(w),
      Self::Float(f) => f.write(w),
      Self::Var(v) => v.write(w),
      //
      Self::Group(g) => g.write(w),
      Self::Block(b) => b.write(w),
      Self::Assign(a) => a.write(w),
      Self::Declare(d) => d.write(w),
      Self::Return(r) => r.write(w),
      //
      Self::Array(a) => a.write(w),
      Self::Function(f) => f.write(w),
      Self::Struct(s) => s.write(w),
      //
      Self::Call(c) => c.write(w),
      Self::Field(f) => f.write(w),
      Self::Index(i) => i.write(w),
      Self::Infix(i) => i.write(w),
      Self::Monad(m) => m.write(w),
    }
  }
}
