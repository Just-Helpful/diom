mod arrays;
pub use arrays::Array;
mod chars;
pub use chars::Char;
mod enums;
pub use enums::Enum;
mod floats;
pub use floats::Float;
mod functions;
pub use functions::Function;
mod structs;
pub use structs::Struct;
mod tuples;
pub use tuples::Tuple;

use crate::ident::Ident;

pub enum Type<I> {
  /* base types */
  Float(Float<I>),
  Char(Char<I>),
  /* type variables */
  Var(Ident<I>),
  /* structural types for composition */
  Array(Array<I>),
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Enum(Enum<I>),
  /* function types */
  Function(Function<I>),
}
