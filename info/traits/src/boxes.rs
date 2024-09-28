use super::{InfoMap, InfoSource};

impl<I: InfoSource> InfoSource for Box<I> {
  type Info = I::Info;
}

impl<I: InfoMap> InfoMap for Box<I> {
  type GenericSelf<T> = Box<<I as InfoMap>::GenericSelf<T>>;
  fn map<R>(self, f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
    Box::new(InfoMap::map(*self, f))
  }
}
