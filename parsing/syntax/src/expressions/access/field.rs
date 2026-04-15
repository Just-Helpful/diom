use super::Expression;
use crate::{idents::Method, Ptr};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Field<I> {
  pub value: Ptr<Expression<I>>,
  pub name: Method<I>,
  pub info: I,
}

impl<I> Display for Field<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char('.')?;
    self.name.fmt(f)
  }
}

impl DisplayAs<Spans> for Field<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("field", &self.info)?;
    self.value.write(&mut w.child())?;
    self.name.write(&mut w.child())
  }
}

impl Field<()> {
  /// Generates a generic strategy for generating `Field` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>>) -> impl Strategy<Value = Self> {
    (item, Method::any()).prop_map(|(value, name)| Field {
      value: Ptr::new(value),
      name,
      info: (),
    })
  }
}
