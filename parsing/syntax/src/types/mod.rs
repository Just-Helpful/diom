use std::ops::Range;

use crate::{fmt::OptionsDisplay, ident::Ident};
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

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
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

impl OptionsDisplay for Type<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      Self::Var(v) => v.optn_fmt(w, depth),
      //
      Self::Array(a) => a.optn_fmt(w, depth),
      Self::Struct(s) => s.optn_fmt(w, depth),
      Self::Tuple(t) => t.optn_fmt(w, depth),
      Self::Enum(e) => e.optn_fmt(w, depth),
      //
      Self::Function(f) => f.optn_fmt(w, depth),
    }
  }
}
