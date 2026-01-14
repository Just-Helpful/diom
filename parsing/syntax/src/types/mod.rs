use crate::ident::Ident;
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

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
    use Type::*;
    match self {
      Var(v) => v.write(w),
      //
      Array(a) => a.write(w),
      Struct(s) => s.write(w),
      Tuple(t) => t.write(w),
      Enum(e) => e.write(w),
      //
      Function(f) => f.write(w),
    }
  }
}
