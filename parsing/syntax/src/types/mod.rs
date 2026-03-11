use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

mod arrays;
pub use arrays::Array;
mod chars;
pub use chars::Char;
mod enums;
pub use enums::Enum;
mod floats;
pub use floats::Float;
mod functions;
pub use functions::{Function, Parameter, Parameters};
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

impl<I> Display for Type<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Type::*;
    match self {
      Var(v) => v.fmt(f),
      //
      Array(a) => a.fmt(f),
      Struct(s) => s.fmt(f),
      Tuple(t) => t.fmt(f),
      Enum(e) => e.fmt(f),
      //
      Function(v) => v.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for Type<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
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
