use crate::ident::Ident;
use diom_info::{InfoMap, InfoRef, InfoSource};
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

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
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
