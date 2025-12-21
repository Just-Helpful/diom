use crate::fmt::MultiDisplay;
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

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

impl MultiDisplay for Pattern<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    match self {
      Pattern::Array(a) => a.multi_fmt(w, depth),
      Pattern::Struct(s) => s.multi_fmt(w, depth),
      Pattern::Tuple(t) => t.multi_fmt(w, depth),
      Pattern::Ignored(i) => i.multi_fmt(w, depth),
      Pattern::Var(v) => v.multi_fmt(w, depth),
    }
  }
}
