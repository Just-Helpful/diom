use super::{InfoMap, InfoSource};

impl<I: InfoSource> InfoSource for Option<I> {
  type Info = I::Info;
}

unsafe impl<I: InfoMap> InfoMap for Option<I> {
  /// ## Safety
  ///
  /// ```ignore
  /// Self::GenericSelf<Self::Info>
  ///   "`InfoMap` implementation"
  /// => Option<<I as InfoMap>::GenericSelf<Self::Info>>
  ///   "`InfoSource` implementation"
  /// => Option<<I as InfoMap>::GenericSelf<I::Info>>
  ///   "Induction"
  /// => Option<I>
  ///   "Implemetation"
  /// => Self
  /// ```
  type GenericSelf<T> = Option<<I as InfoMap>::GenericSelf<T>>;
  fn map<R>(self, f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
    self.map(|value| value.map(f))
  }
}
