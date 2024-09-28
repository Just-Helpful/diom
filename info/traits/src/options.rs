use super::{InfoMap, InfoSource};

impl<I: InfoSource> InfoSource for Option<I> {
  type Info = I::Info;
}

impl<I: InfoMap> InfoMap for Option<I> {
  type GenericSelf<T> = Option<<I as InfoMap>::GenericSelf<T>>;
  fn map<R>(self, f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
    self.map(|value| value.map(f))
  }
}
