use crate::{
  expressions::{
    access::{call::CallConfig, index::IndexConfig},
    blocks::{block::BlockConfig, declare::DeclareConfig},
    compound::{arrays::ArrayConfig, functions::FunctionConfig, structs::StructConfig},
  },
  ident::Ident,
  patterns::PatternConfig,
  types::TypeConfig,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{
  prelude::{Arbitrary, BoxedStrategy, Strategy},
  prop_oneof,
};
use std::{
  fmt::{Display, Formatter, Write},
  ops::Range,
};

mod chars;
pub use chars::Char;
mod floats;
pub use floats::Float;

mod access;
pub use access::{Call, Field, Index, Infix, MonadResult, MonadThen};
mod blocks;
pub use blocks::{Assign, Block, Declare, Group, Return, Statement};
mod compound;
pub use compound::{Array, Function, FunctionArm, Parameter, Parameters, Struct};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum Expression<I> {
  /* base values in the language */
  Char(Char<I>),
  Float(Float<I>),
  Var(Ident<I>),
  /* block expressions */
  Group(Group<I>),
  Block(Block<I>),
  Assign(Assign<I>),
  Declare(Declare<I>),
  Return(Return<I>),
  /* compound values in the language */
  Array(Array<I>),
  Function(Function<I>),
  Struct(Struct<I>),
  /* accessing values in the language */
  Call(Call<I>),
  Field(Field<I>),
  Index(Index<I>),
  Infix(Infix<I>),
  Monad(MonadThen<I>),
  Result(MonadResult<I>),
}

impl<I> Display for Expression<I> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    use Expression::*;
    match self {
      Char(c) => c.fmt(f),
      Float(v) => v.fmt(f),
      Var(v) => v.fmt(f),
      //
      Group(g) => g.fmt(f),
      Block(b) => b.fmt(f),
      Assign(a) => a.fmt(f),
      Declare(d) => d.fmt(f),
      Return(r) => r.fmt(f),
      //
      Array(a) => a.fmt(f),
      Function(v) => v.fmt(f),
      Struct(s) => s.fmt(f),
      //
      Call(c) => c.fmt(f),
      Field(v) => v.fmt(f),
      Index(i) => i.fmt(f),
      Infix(i) => i.fmt(f),
      Monad(m) => m.fmt(f),
      Result(r) => r.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for Expression<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    use Expression::*;
    match self {
      Char(c) => c.write(w),
      Float(f) => f.write(w),
      Var(v) => v.write(w),
      //
      Group(g) => g.write(w),
      Block(b) => b.write(w),
      Assign(a) => a.write(w),
      Declare(d) => d.write(w),
      Return(r) => r.write(w),
      //
      Array(a) => a.write(w),
      Function(f) => f.write(w),
      Struct(s) => s.write(w),
      //
      Call(c) => c.write(w),
      Field(f) => f.write(w),
      Index(i) => i.write(w),
      Infix(i) => i.write(w),
      Monad(m) => m.write(w),
      Result(r) => r.write(w),
    }
  }
}

#[derive(Clone, Copy)]
pub struct ExpressionConfig {
  /// The config used to generate patterns
  pub patterns: PatternConfig,
  /// The config used to generate types
  pub types: TypeConfig,

  /// The maximum depth of an expression
  pub depth: u32,
  /// The maximum size of an expression
  pub size: u32,

  /// The maximum number of statements in a block
  pub block_stmts: usize,
  /// The maximum number of items in an array
  pub array_items: usize,
  /// The maximum number of parameters in a function arm
  pub func_params: usize,
  /// The maximum number of arms in a function
  pub func_arms: usize,
  /// The maximum number of fields in a struct
  pub struct_fields: usize,
  /// The maximum number of arguments to a function call
  pub call_args: usize,
  /// The maximum number of keys for a value index
  pub index_keys: usize,
}
impl Default for ExpressionConfig {
  fn default() -> Self {
    let func_config = FunctionConfig::default();
    Self {
      patterns: PatternConfig {
        // lower pattern depth
        depth: 3,
        size: 64,
        ..Default::default()
      },
      types: TypeConfig {
        // lower type depth
        depth: 3,
        size: 64,
        ..Default::default()
      },

      depth: 10,
      size: 256,

      block_stmts: BlockConfig::default().1,
      array_items: ArrayConfig::default().0,
      func_params: func_config.2,
      func_arms: func_config.3,
      struct_fields: StructConfig::default().0,
      call_args: CallConfig::default().0,
      index_keys: IndexConfig::default().0,
    }
  }
}
impl From<ExpressionConfig> for BlockConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(value.types, value.block_stmts)
  }
}
impl From<ExpressionConfig> for DeclareConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(value.patterns, value.types)
  }
}
impl From<ExpressionConfig> for ArrayConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(value.array_items)
  }
}
impl From<ExpressionConfig> for FunctionConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(
      value.patterns,
      value.types,
      value.func_params,
      value.func_arms,
    )
  }
}
impl From<ExpressionConfig> for StructConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(value.struct_fields)
  }
}
impl From<ExpressionConfig> for CallConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(value.call_args)
  }
}
impl From<ExpressionConfig> for IndexConfig {
  fn from(value: ExpressionConfig) -> Self {
    Self(value.index_keys)
  }
}
impl Expression<()> {
  /// Generates a generic strategy for generating `Expression` nodes
  pub fn any(args: ExpressionConfig) -> impl Strategy<Value = Self> {
    let leaf = prop_oneof![
      Char::any().prop_map(Self::Char),
      Float::any().prop_map(Self::Float),
      Ident::any().prop_map(Self::Var)
    ];
    let branch_width = args
      .block_stmts
      .max(args.array_items)
      .max(args.func_arms)
      .max(args.struct_fields)
      .max(args.call_args + 1)
      .max(args.index_keys + 1) as u32;

    leaf.prop_recursive(args.depth, args.size, branch_width, move |item| {
      prop_oneof![
        Group::any(item.clone()).prop_map(Self::Group),
        Block::any(item.clone(), args.into()).prop_map(Self::Block),
        Assign::any(item.clone()).prop_map(Self::Assign),
        Declare::any(item.clone(), args.into()).prop_map(Self::Declare),
        Return::any(item.clone()).prop_map(Self::Return),
        //
        Array::any(item.clone(), args.into()).prop_map(Self::Array),
        Function::any(item.clone(), args.into()).prop_map(Self::Function),
        Struct::any(item.clone(), args.into()).prop_map(Self::Struct),
        //
        Call::any(item.clone(), args.into()).prop_map(Self::Call),
        Field::any(item.clone()).prop_map(Self::Field),
        Index::any(item.clone(), args.into()).prop_map(Self::Index),
        Infix::any(item.clone()).prop_map(Self::Infix),
        MonadThen::any(item.clone()).prop_map(Self::Monad),
        MonadResult::any(item.clone()).prop_map(Self::Result),
      ]
    })
  }
}
impl Arbitrary for Expression<()> {
  type Parameters = ExpressionConfig;
  type Strategy = BoxedStrategy<Self>;

  fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
    Self::any(args).boxed()
  }
}
