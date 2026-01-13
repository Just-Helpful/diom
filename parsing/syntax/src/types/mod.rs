use std::ops::Range;

use crate::{
  fmt::{CustomDisplay, SpanWriter},
  ident::Ident,
};
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

impl CustomDisplay<SpanWriter> for Type<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    match self {
      Self::Var(v) => v.write(w),
      //
      Self::Array(a) => a.write(w),
      Self::Struct(s) => s.write(w),
      Self::Tuple(t) => t.write(w),
      Self::Enum(e) => e.write(w),
      //
      Self::Function(f) => f.write(w),
    }
  }
}
