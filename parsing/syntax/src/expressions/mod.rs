use std::ops::Range;

use crate::ident::Ident;
use diom_fmt::{CustomDisplay, SpanWriter};
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
    }
  }
}
