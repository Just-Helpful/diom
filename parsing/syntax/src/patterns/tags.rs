use crate::{display::Seq, idents::Ident, patterns::Pattern};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// A type with a unique tag attached.\
/// This effectively converts duck typing into static typing.
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Tagged<I> {
  pub name: Ident<I>,
  pub value: Box<Pattern<I>>,
  pub info: I,
}

impl<I> Display for Tagged<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Seq(&(&self.name, &self.value)).fmt(f)
  }
}

impl DisplayAs<Spans> for Tagged<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("tag", &self.info)?;
    self.name.write(&mut w.child())?;
    self.value.write(&mut w.child())
  }
}

impl Tagged<()> {
  pub fn any(item: impl Strategy<Value = Pattern<()>>) -> impl Strategy<Value = Self> {
    (Ident::any(), item).prop_map(|(name, value)| Tagged {
      name,
      value: Box::new(value),
      info: (),
    })
  }
}
