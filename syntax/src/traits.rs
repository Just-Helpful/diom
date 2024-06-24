use std::ops::Range;

pub trait InfoSource {
  type Info;
  fn info(&self) -> &Self::Info;
}

pub trait Span {
  fn span(&self) -> Range<usize>;
}

impl Span for dyn InfoSource<Info = Range<usize>> {
  fn span(&self) -> Range<usize> {
    self.info().clone()
  }
}
