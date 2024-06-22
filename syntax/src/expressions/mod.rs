mod chars;
pub use chars::Char;
mod floats;
pub use floats::Float;

mod access;
pub use access::{Call, Field, Index};
mod blocks;
pub use blocks::{Assign, Block, Declare, MonadThen, Return};
mod compound;
pub use compound::{Array, Enum, Struct, Tuple};

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
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Enum(Enum<I>),
  /* accessing values in the language */
  Call(Call<I>),
  Field(Field<I>),
  Index(Index<I>),
}
