//! Deriving Info traits for tuples with up to 6 items.
//!
//! Whilst there is a lot of code duplication here, it makes the pattern a<br>
//! lot more understandable. I am **not** going to use a macro here as it<br>
//! could obscure the implementation details.
//!
//! **note**: Code folding's pretty invaluable here.
use super::{InfoMap, InfoSource};

pub mod tuple1 {
  use crate::InfoRef;

  pub use super::*;

  impl<I0> InfoSource for (I0,)
  where
    I0: InfoSource,
  {
    type Info = I0::Info;
  }

  // We can only really implement InfoRef for items with one source of info.
  // You won't find this implementation on tuples with > 1 item.
  impl<I0> InfoRef for (I0,)
  where
    I0: InfoRef,
  {
    fn info(&self) -> &Self::Info {
      self.0.info()
    }
  }

  impl<I0> InfoMap for (I0,)
  where
    I0: InfoMap,
  {
    type GenericSelf<T> = (<I0 as InfoMap>::GenericSelf<T>,);
    fn map<R>(self, f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      (self.0.map(f),)
    }
  }
}
pub mod tuple2 {
  pub use super::*;

  impl<I0, I1> InfoSource for (I0, I1)
  where
    I0: InfoSource,
    I1: InfoSource<Info = I0::Info>,
  {
    type Info = I0::Info;
  }

  impl<I0, I1> InfoMap for (I0, I1)
  where
    I0: InfoMap,
    I1: InfoMap<Info = I0::Info>,
  {
    type GenericSelf<T> = (
      <I0 as InfoMap>::GenericSelf<T>,
      <I1 as InfoMap>::GenericSelf<T>,
    );
    fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      (self.0.map(&mut f), self.1.map(f))
    }
  }
}
pub mod tuple3 {
  pub use super::*;

  impl<I0, I1, I2> InfoSource for (I0, I1, I2)
  where
    I0: InfoSource,
    I1: InfoSource<Info = I0::Info>,
    I2: InfoSource<Info = I0::Info>,
  {
    type Info = I0::Info;
  }

  impl<I0, I1, I2> InfoMap for (I0, I1, I2)
  where
    I0: InfoMap,
    I1: InfoMap<Info = I0::Info>,
    I2: InfoMap<Info = I0::Info>,
  {
    type GenericSelf<T> = (
      <I0 as InfoMap>::GenericSelf<T>,
      <I1 as InfoMap>::GenericSelf<T>,
      <I2 as InfoMap>::GenericSelf<T>,
    );
    fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      (self.0.map(&mut f), self.1.map(&mut f), self.2.map(f))
    }
  }
}
pub mod tuple4 {
  pub use super::*;

  impl<I0, I1, I2, I3> InfoSource for (I0, I1, I2, I3)
  where
    I0: InfoSource,
    I1: InfoSource<Info = I0::Info>,
    I2: InfoSource<Info = I0::Info>,
    I3: InfoSource<Info = I0::Info>,
  {
    type Info = I0::Info;
  }

  impl<I0, I1, I2, I3> InfoMap for (I0, I1, I2, I3)
  where
    I0: InfoMap,
    I1: InfoMap<Info = I0::Info>,
    I2: InfoMap<Info = I0::Info>,
    I3: InfoMap<Info = I0::Info>,
  {
    type GenericSelf<T> = (
      <I0 as InfoMap>::GenericSelf<T>,
      <I1 as InfoMap>::GenericSelf<T>,
      <I2 as InfoMap>::GenericSelf<T>,
      <I3 as InfoMap>::GenericSelf<T>,
    );
    fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      (
        self.0.map(&mut f),
        self.1.map(&mut f),
        self.2.map(&mut f),
        self.3.map(f),
      )
    }
  }
}
pub mod tuple5 {
  pub use super::*;

  impl<I0, I1, I2, I3, I4> InfoSource for (I0, I1, I2, I3, I4)
  where
    I0: InfoSource,
    I1: InfoSource<Info = I0::Info>,
    I2: InfoSource<Info = I0::Info>,
    I3: InfoSource<Info = I0::Info>,
    I4: InfoSource<Info = I0::Info>,
  {
    type Info = I0::Info;
  }

  impl<I0, I1, I2, I3, I4> InfoMap for (I0, I1, I2, I3, I4)
  where
    I0: InfoMap,
    I1: InfoMap<Info = I0::Info>,
    I2: InfoMap<Info = I0::Info>,
    I3: InfoMap<Info = I0::Info>,
    I4: InfoMap<Info = I0::Info>,
  {
    type GenericSelf<T> = (
      <I0 as InfoMap>::GenericSelf<T>,
      <I1 as InfoMap>::GenericSelf<T>,
      <I2 as InfoMap>::GenericSelf<T>,
      <I3 as InfoMap>::GenericSelf<T>,
      <I4 as InfoMap>::GenericSelf<T>,
    );
    fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      (
        self.0.map(&mut f),
        self.1.map(&mut f),
        self.2.map(&mut f),
        self.3.map(&mut f),
        self.4.map(f),
      )
    }
  }
}
pub mod tuple6 {
  pub use super::*;

  impl<I0, I1, I2, I3, I4, I5> InfoSource for (I0, I1, I2, I3, I4, I5)
  where
    I0: InfoSource,
    I1: InfoSource<Info = I0::Info>,
    I2: InfoSource<Info = I0::Info>,
    I3: InfoSource<Info = I0::Info>,
    I4: InfoSource<Info = I0::Info>,
    I5: InfoSource<Info = I0::Info>,
  {
    type Info = I0::Info;
  }

  impl<I0, I1, I2, I3, I4, I5> InfoMap for (I0, I1, I2, I3, I4, I5)
  where
    I0: InfoMap,
    I1: InfoMap<Info = I0::Info>,
    I2: InfoMap<Info = I0::Info>,
    I3: InfoMap<Info = I0::Info>,
    I4: InfoMap<Info = I0::Info>,
    I5: InfoMap<Info = I0::Info>,
  {
    type GenericSelf<T> = (
      <I0 as InfoMap>::GenericSelf<T>,
      <I1 as InfoMap>::GenericSelf<T>,
      <I2 as InfoMap>::GenericSelf<T>,
      <I3 as InfoMap>::GenericSelf<T>,
      <I4 as InfoMap>::GenericSelf<T>,
      <I5 as InfoMap>::GenericSelf<T>,
    );
    fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      (
        self.0.map(&mut f),
        self.1.map(&mut f),
        self.2.map(&mut f),
        self.3.map(&mut f),
        self.4.map(&mut f),
        self.5.map(f),
      )
    }
  }
}
