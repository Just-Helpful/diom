use crate::{writers::SpanWriter, DisplayAs, Spans};
use std::fmt::Write;

impl<T: DisplayAs<Spans>> DisplayAs<Spans> for Option<T> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    self.as_ref().map_or(Ok(()), |value| value.write(w))
  }
}

impl<T: DisplayAs<Spans>> DisplayAs<Spans> for Vec<T> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    self.iter().try_for_each(move |value| value.write(w))
  }
}

impl<T0: DisplayAs<Spans>, T1: DisplayAs<Spans>> DisplayAs<Spans> for (T0, T1) {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    self.0.write(w)?;
    self.1.write(w)
  }
}
