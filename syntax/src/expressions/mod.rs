mod chars;
pub use chars::Char;
mod floats;
use crate::InfoSource;
pub use floats::Float;

mod access;
pub use access::{Call, Field, Index};
mod blocks;
pub use blocks::{Assign, Block, Declare, MonadThen, Return, Statement};
mod compound;
pub use compound::{Argument, Array, Function, FunctionArm, Struct, Tuple};

#[derive(InfoSource, Clone)]
pub enum Expression<I> {
  /* fundamental values to the language */
  Char(Char<I>),
  Float(Float<I>),
  /* block expressions */
  Block(Block<I>),
  Assign(Assign<I>),
  Declare(Declare<I>),
  MonadThen(MonadThen<I>),
  Return(Return<I>),
  /* compound values in the language */
  Array(Array<I>),
  Function(Function<I>),
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  /* accessing values in the language */
  Call(Call<I>),
  Field(Field<I>),
  Index(Index<I>),
}
