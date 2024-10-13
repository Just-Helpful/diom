use super::{InfoMap, InfoSource};

impl<I: InfoSource> InfoSource for Vec<I> {
  type Info = I::Info;
}

unsafe impl<I: InfoMap> InfoMap for Vec<I> {
  /// ## Safety
  ///
  /// ```ignore
  /// Self::GenericSelf<Self::Info>
  ///   "`InfoMap` implementation"
  /// => Vec<<I as InfoMap>::GenericSelf<Self::Info>>
  ///   "`InfoSource` implementation"
  /// => Vec<<I as InfoMap>::GenericSelf<I::Info>>
  ///   "Induction"
  /// => Vec<I>
  ///   "Implemetation"
  /// => Self
  /// ```
  type GenericSelf<T> = Vec<<I as InfoMap>::GenericSelf<T>>;
  fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
    self.into_iter().map(|x| InfoMap::map(x, &mut f)).collect()
  }
}
