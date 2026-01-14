/// A value that can provide partial updates to another
pub trait Updater<V> {
  /// Partially update the value `V`
  fn update(self, value: V) -> V;
}

impl Updater<[usize; 2]> for [usize; 0] {
  #[inline]
  fn update(self, value: [usize; 2]) -> [usize; 2] {
    value
  }
}
impl Updater<[usize; 2]> for usize {
  #[inline]
  fn update(self, [_, y]: [usize; 2]) -> [usize; 2] {
    [self, y]
  }
}
impl Updater<[usize; 2]> for [usize; 1] {
  #[inline]
  fn update(self, [_, y]: [usize; 2]) -> [usize; 2] {
    [self[0], y]
  }
}
impl Updater<[usize; 2]> for [usize; 2] {
  #[inline]
  fn update(self, _: [usize; 2]) -> [usize; 2] {
    self
  }
}
