use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Write, ops::Range};

pub mod arrays;
use arrays::Array;
pub mod ignored;
use ignored::Ignored;
pub mod rest;
use rest::Rest;
pub mod structs;
use structs::Struct;
pub mod tuples;
use tuples::Tuple;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum Pattern<I> {
  Array(Array<I>),
  Struct(Struct<I>),
  Tuple(Tuple<I>),
  Ignored(Ignored<I>),
  Var(Ident<I>),
}

impl DisplayAs<Spans> for Pattern<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    use Pattern::*;
    match self {
      Array(a) => a.write(w),
      Struct(s) => s.write(w),
      Tuple(t) => t.write(w),
      Ignored(i) => i.write(w),
      Var(v) => v.write(w),
    }
  }
}
