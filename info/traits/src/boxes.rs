use std::ops::Deref;

use crate::InfoRef;

use super::{InfoMap, InfoSource};

impl<I: InfoSource + ?Sized> InfoSource for Box<I> {
  type Info = I::Info;
}

impl<I: InfoRef + ?Sized> InfoRef for Box<I> {
  fn info(&self) -> &Self::Info {
    self.deref().info()
  }
}

unsafe impl<I: InfoMap> InfoMap for Box<I> {
  /// ## Safety
  ///
  /// ```ignore
  /// Self::GenericSelf<Self::Info>
  ///   "`InfoMap` implementation"
  /// => Box<<I as InfoMap>::GenericSelf<Self::Info>>
  ///   "`InfoSource` implementation"
  /// => Box<<I as InfoMap>::GenericSelf<I::Info>>
  ///   "Induction"
  /// => Box<I>
  ///   "Implemetation"
  /// => Self
  /// ```
  type GenericSelf<T> = Box<<I as InfoMap>::GenericSelf<T>>;

  fn map_dyn<R>(self, f: &mut dyn FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
    Box::new(InfoMap::map(*self, f))
  }
}
