use std::{ops::Deref, rc::Rc};

use crate::InfoRef;

use super::{InfoMap, InfoSource};

impl<I: InfoSource + ?Sized> InfoSource for Rc<I> {
  type Info = I::Info;
}

impl<I: InfoRef + ?Sized> InfoRef for Rc<I> {
  fn info(&self) -> &Self::Info {
    self.deref().info()
  }
}

unsafe impl<I: InfoMap> InfoMap for Rc<I> {
  /// ## Safety
  ///
  /// ```_
  /// Self::GenericSelf<Self::Info>
  ///   "`InfoMap` implementation"
  /// => Rc<<I as InfoMap>::GenericSelf<Self::Info>>
  ///   "`InfoSource` implementation"
  /// => Rc<<I as InfoMap>::GenericSelf<I::Info>>
  ///   "Induction"
  /// => Rc<I>
  ///   "Implemetation"
  /// => Self
  /// ```
  type GenericSelf<T> = Rc<<I as InfoMap>::GenericSelf<T>>;

  fn map_dyn<R>(self, f: &mut dyn FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
    // @todo this is not the best, ideally we'd have a stronger guarantee on uniqueness
    let inner = Rc::into_inner(self).expect("InfoMap called alongside mutable borrow");
    Rc::new(InfoMap::map(inner, f))
  }
}
