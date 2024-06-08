mod arrays;
pub use arrays::Array;
mod chars;
pub use chars::Char;
mod floats;
pub use floats::Float;
mod structs;
pub use structs::Struct;
mod tuples;
pub use tuples::Tuple;
mod enums;
pub use enums::Enum;

pub enum Value<I> {
  /* fundamental values to the language */
  Char(Char<I>),
  Float(Float<I>),
  /* compound values in the language */
  Array(Array<I>),
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Enum(Enum<I>),
}
