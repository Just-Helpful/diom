use crate::scope::SyntaxScope;

pub mod arrays;
use arrays::Array;
pub mod rest;
use rest::Rest;
pub mod structs;
use structs::Struct;
pub mod tuples;
use tuples::Tuple;

#[derive(Clone, Debug)]
pub enum Pattern<S: SyntaxScope> {
  Array(Array<S>),
  Struct(Struct<S>),
  Tuple(Tuple<S>),
  Var(S::Ident),
  Ignored,
}

// impl DisplayAs<Spans> for Pattern<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     use Pattern::*;
//     match self {
//       Array(a) => a.write(w),
//       Struct(s) => s.write(w),
//       Tuple(t) => t.write(w),
//       Ignored(i) => i.write(w),
//       Var(v) => v.write(w),
//     }
//   }
// }
