use crate::{
  idents::Ident,
  patterns::{arrays::ArrayConfig, structs::StructConfig, tuples::TupleConfig},
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{
  prelude::{Arbitrary, BoxedStrategy, Strategy},
  prop_oneof,
};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

pub mod arrays;
use arrays::Array;
pub mod ignored;
use ignored::Ignored;
pub mod rest;
use rest::Rest;
pub mod structs;
use structs::Struct;
pub mod tags;
pub use tags::Tagged;
pub mod tuples;
use tuples::Tuple;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub enum Pattern<I> {
  Array(Array<I>),
  Struct(Struct<I>),
  Tagged(Tagged<I>),
  Tuple(Tuple<I>),
  Ignored(Ignored<I>),
  Var(Ident<I>),
}

impl<I> Display for Pattern<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Pattern::*;
    match self {
      Array(a) => a.fmt(f),
      Struct(s) => s.fmt(f),
      Tagged(t) => t.fmt(f),
      Tuple(t) => t.fmt(f),
      Ignored(i) => i.fmt(f),
      Var(v) => v.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for Pattern<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    use Pattern::*;
    match self {
      Array(a) => a.write(w),
      Struct(s) => s.write(w),
      Tagged(t) => t.write(w),
      Tuple(t) => t.write(w),
      Ignored(i) => i.write(w),
      Var(v) => v.write(w),
    }
  }
}

#[derive(Clone, Copy)]
pub struct PatternConfig {
  /// The maximum depth for type definitions
  pub depth: u32,
  /// The maximum number of nodes in type definitions
  pub size: u32,

  /// The maximum number of items in an array
  pub array_items: usize,
  /// The maximum number of fields in a struct
  pub struct_fields: usize,
  /// The maximum number of items in a tuple
  pub tuple_items: usize,
}
impl Default for PatternConfig {
  fn default() -> Self {
    Self {
      depth: 8,
      size: 256,
      array_items: ArrayConfig::default().0,
      struct_fields: StructConfig::default().0,
      tuple_items: TupleConfig::default().0,
    }
  }
}
impl From<PatternConfig> for ArrayConfig {
  fn from(value: PatternConfig) -> Self {
    Self(value.array_items)
  }
}
impl From<PatternConfig> for StructConfig {
  fn from(value: PatternConfig) -> Self {
    Self(value.struct_fields)
  }
}
impl From<PatternConfig> for TupleConfig {
  fn from(value: PatternConfig) -> Self {
    Self(value.tuple_items)
  }
}
impl Pattern<()> {
  /// Generates a generic strategy for generating `Pattern` nodes
  pub fn any(args: PatternConfig) -> impl Strategy<Value = Self> {
    let leaf = prop_oneof![
      Ignored::any().prop_map(Self::Ignored),
      Ident::any().prop_map(Self::Var),
    ];
    let branch_width = args
      .array_items
      .max(args.struct_fields)
      .max(args.tuple_items) as u32;

    leaf.prop_recursive(args.depth, args.size, branch_width, move |inner| {
      prop_oneof![
        Tagged::any(inner.clone()).prop_map(Self::Tagged),
        Tuple::any(inner.clone(), args.into()).prop_map(Self::Tuple),
        Array::any(inner.clone(), args.into()).prop_map(Self::Array),
        Struct::any(inner.clone(), args.into()).prop_map(Self::Struct),
      ]
    })
  }
}
impl Arbitrary for Pattern<()> {
  type Parameters = PatternConfig;
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
    Self::any(args).boxed()
  }
}
