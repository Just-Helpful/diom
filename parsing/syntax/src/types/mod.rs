use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

mod arrays;
pub use arrays::Array;
mod chars;
pub use chars::Char;
mod enums;
pub use enums::Enum;
mod floats;
pub use floats::Float;
mod functions;
pub use functions::{Argument, Function};
mod structs;
pub use structs::Struct;
mod tuples;
pub use tuples::Tuple;
mod typedef;
pub use typedef::TypeDef;

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub enum Type<I> {
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
