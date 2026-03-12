use crate::display::Sep;

use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy};
use std::{
  fmt::{Debug, Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Call<I> {
  pub value: Box<Expression<I>>,
  pub args: Vec<Expression<I>>,
  pub info: I,
}

impl<I> Display for Call<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char('(')?;
    Sep(&self.args, ',').fmt(f)?;
    f.write_char(')')
  }
}

impl DisplayAs<Spans> for Call<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("call", &self.info)?;
    self.value.write(&mut w.child())?;
    self.args.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct CallConfig(
  /// The maximum number of arguments in a function call
  pub usize,
);
impl Default for CallConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Call<()> {
  /// Generates a generic strategy for generating `Call` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>> + Clone,
    args: CallConfig,
  ) -> impl Strategy<Value = Self> {
    (item.clone(), vec(item, 0..args.0)).prop_map(|(value, args)| Call {
      value: Box::new(value),
      args,
      info: (),
    })
  }
}
