use crate::{
  ident::Ident,
  types::{
    enums::EnumConfig, functions::FunctionConfig, structs::StructConfig, tuples::TupleConfig,
  },
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
pub use typedef::{Alias, NewType, TypeDef};
mod tags;
pub use tags::Tagged;

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

#[derive(Clone, Copy)]
pub struct TypeConfig {
  /// The maximum depth for type definitions
  pub depth: u32,
  /// The maximum number of nodes in type definitions
  pub size: u32,

  /// The maximum number of variants in an enum
  pub enum_variants: usize,
  /// The maximum number of parameters per function
  pub fn_parameters: usize,
  /// The maximum number of fields in a struct
  pub struct_props: usize,
  /// The maximum number of items in a tuple
  pub tuple_items: usize,
}
impl Default for TypeConfig {
  fn default() -> Self {
    Self {
      depth: 8,
      size: 256,
      enum_variants: EnumConfig::default().0,
      fn_parameters: FunctionConfig::default().0,
      struct_props: StructConfig::default().0,
      tuple_items: TupleConfig::default().0,
    }
  }
}
impl From<TypeConfig> for StructConfig {
  fn from(value: TypeConfig) -> Self {
    Self(value.struct_props)
  }
}
impl From<TypeConfig> for TupleConfig {
  fn from(value: TypeConfig) -> Self {
    Self(value.tuple_items)
  }
}
impl From<TypeConfig> for EnumConfig {
  fn from(value: TypeConfig) -> Self {
    Self(value.enum_variants)
  }
}
impl From<TypeConfig> for FunctionConfig {
  fn from(value: TypeConfig) -> Self {
    Self(value.fn_parameters)
  }
}
impl Type<()> {
  /// Generates a generic strategy for generating `Type`s
  pub fn any(args: TypeConfig) -> impl Strategy<Value = Self> {
    let leaf = Ident::any().prop_map(Self::Var);
    // the maximum expected branch width
    let branch_width = args
      .enum_variants
      .max(args.fn_parameters)
      .max(args.struct_props)
      .max(args.tuple_items) as u32;

    leaf.prop_recursive(args.depth, args.size, branch_width, move |inner| {
      prop_oneof![
        Array::any(inner.clone()).prop_map(Self::Array),
        Struct::any(inner.clone(), args.into()).prop_map(Self::Struct),
        Tuple::any(inner.clone(), args.into()).prop_map(Self::Tuple),
        Enum::any(inner.clone(), args.into()).prop_map(Self::Enum),
        Function::any(inner.clone(), args.into()).prop_map(Self::Function),
      ]
    })
  }
}
impl Arbitrary for Type<()> {
  type Parameters = TypeConfig;
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
    Self::any(args).boxed()
  }
}
