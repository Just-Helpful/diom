use super::Expression;
use crate::{
  fmt::{bracket, MultiDisplay},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Field<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}

impl MultiDisplay for Field<Range<usize>> {
  type Options = usize;
  fn multi_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at([self.info.start, depth], bracket("field", self.info.len()));
    self.value.multi_fmt(w, depth + 1)?;
    self.name.multi_fmt(w, depth + 1)?;
    Ok(())
  }
}
