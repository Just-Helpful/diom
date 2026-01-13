use crate::fmt::{CustomDisplay, SpanWriter};

impl<T: CustomDisplay<SpanWriter>> CustomDisplay<SpanWriter> for Option<T> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    self.as_ref().map_or(Ok(()), |value| value.write(w))
  }
}

impl<'a, T: CustomDisplay<SpanWriter>> CustomDisplay<SpanWriter> for Vec<T> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    self.iter().try_for_each(move |value| value.write(w))
  }
}

impl<'a, T0: CustomDisplay<SpanWriter>, T1: CustomDisplay<SpanWriter>> CustomDisplay<SpanWriter>
  for (T0, T1)
{
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    self.0.write(w)?;
    self.1.write(w)
  }
}
