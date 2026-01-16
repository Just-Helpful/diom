use crate::scope::SyntaxScope;

mod access;
pub use access::{Call, Field, Index, Infix, MonadResult, MonadThen};
mod blocks;
pub use blocks::{Assign, Block, Declare, Group, Return, Statement};
mod compound;
pub use compound::{Argument, Array, Function, FunctionArm, Struct};
mod chars;
pub use chars::Char;
mod floats;
pub use floats::Float;

#[derive(Clone, Debug)]
pub enum Expression<S: SyntaxScope> {
  /* base values in the language */
  Char(Char),
  Float(Float<S>),
  Var(S::Ident),
  /* block expressions */
  Group(Group<S>),
  Block(Block<S>),
  Assign(Assign<S>),
  Declare(Declare<S>),
  Return(Return<S>),
  /* compound values in the language */
  Array(Array<S>),
  Function(Function<S>),
  Struct(Struct<S>),
  /* accessing values in the language */
  Call(Call<S>),
  Field(Field<S>),
  Index(Index<S>),
  Infix(Infix<S>),
  Monad(MonadThen<S>),
  Result(MonadResult<S>),
}

// impl DisplayAs<Spans> for Expression<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     use Expression::*;
//     match self {
//       Char(c) => c.write(w),
//       Float(f) => f.write(w),
//       Var(v) => v.write(w),
//       //
//       Group(g) => g.write(w),
//       Block(b) => b.write(w),
//       Assign(a) => a.write(w),
//       Declare(d) => d.write(w),
//       Return(r) => r.write(w),
//       //
//       Array(a) => a.write(w),
//       Function(f) => f.write(w),
//       Struct(s) => s.write(w),
//       //
//       Call(c) => c.write(w),
//       Field(f) => f.write(w),
//       Index(i) => i.write(w),
//       Infix(i) => i.write(w),
//       Monad(m) => m.write(w),
//       Result(r) => r.write(w),
//     }
//   }
// }
