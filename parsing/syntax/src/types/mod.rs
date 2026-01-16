use crate::scope::SyntaxScope;

mod arrays;
pub use arrays::Array;
mod enums;
pub use enums::Enum;
mod functions;
pub use functions::{Argument, Function};
mod structs;
pub use structs::Struct;
mod tuples;
pub use tuples::Tuple;
mod typedef;
pub use typedef::TypeDef;

#[derive(Clone, Debug)]
pub enum Type<S: SyntaxScope> {
  /* type variables */
  Var(S::Ident),
  Char,
  Float,
  /* structural types for composition */
  Array(Array<S>),
  Struct(Struct<S>),
  Tuple(Tuple<S>),
  Enum(Enum<S>),
  /* function types */
  Function(Function<S>),
}

// impl DisplayAs<Spans> for Type<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     use Type::*;
//     match self {
//       Var(v) => v.write(w),
//       //
//       Array(a) => a.write(w),
//       Struct(s) => s.write(w),
//       Tuple(t) => t.write(w),
//       Enum(e) => e.write(w),
//       //
//       Function(f) => f.write(w),
//     }
//   }
// }
