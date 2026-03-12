use super::{Pattern, Rest};
use crate::{
  display::{Optn, Sep},
  ident::Ident,
  path::{Path, PathConfig},
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, option, prelude::Strategy, prop_oneof};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pattern: Pattern<I>,
  pub info: I,
}

impl<I> Display for StructField<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)?;
    f.write_char(':')?;
    self.pattern.fmt(f)
  }
}

impl DisplayAs<Spans> for StructField<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("field", &self.info)?;
    self.name.write(&mut w.child())?;
    self.pattern.write(&mut w.child())
  }
}

impl StructField<()> {
  /// Generates a generic strategy for generating `StructField` nodes
  pub fn any(item: impl Strategy<Value = Pattern<()>>) -> impl Strategy<Value = Self> {
    (Ident::any(), item).prop_map(|(name, pattern)| StructField {
      name,
      pattern,
      info: (),
    })
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

impl<I> Display for StructItem<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Field(v) => v.fmt(f),
      Self::Rest(r) => r.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for StructItem<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Field(f) => f.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

impl StructItem<()> {
  /// Generates a generic strategy for generating `StructItem` nodes
  pub fn any(item: impl Strategy<Value = Pattern<()>>) -> impl Strategy<Value = Self> {
    prop_oneof![
      Rest::any().prop_map(Self::Rest),
      StructField::any(item).prop_map(Self::Field),
    ]
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<StructItem<I>>,
  pub info: I,
}

impl<I> Display for Struct<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
    Sep(&self.fields, ',').fmt(f)
  }
}

impl DisplayAs<Spans> for Struct<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("struct", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct StructConfig(
  /// The config for the name of the array
  pub PathConfig,
  /// The maximum number of fields in a struct
  pub usize,
);
impl Default for StructConfig {
  fn default() -> Self {
    Self(Default::default(), 50)
  }
}
impl Struct<()> {
  /// Generates a generic strategy for generating `Struct` patterns
  pub fn any(
    item: impl Strategy<Value = Pattern<()>>,
    args: StructConfig,
  ) -> impl Strategy<Value = Self> {
    (
      option::of(Path::any(args.0)),
      vec(StructItem::any(item), 0..args.1),
    )
      .prop_map(|(name, fields)| Struct {
        name,
        fields,
        info: (),
      })
  }
}
