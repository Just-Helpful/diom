/// # AST nodes for Diom
///
/// The nodes here fall into 2 broad categories:
/// 1. nodes for Diom typing
/// 2. nodes for Diom values
pub mod expressions;
pub mod ident;
pub mod path;
pub mod patterns;
pub mod types;

mod display {
  use std::fmt::{Display, Write};

  /// Displays a list of items separated with a character
  pub struct Sep<I: IntoIterator<Item: Display>>(pub I, pub char);

  impl<'a, I: IntoIterator<Item: Display> + 'a + Clone> Display for Sep<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut iter = self.0.clone().into_iter();
      let Some(item) = iter.next() else {
        return Ok(());
      };

      item.fmt(f)?;
      for item in iter {
        f.write_char(self.1)?;
        item.fmt(f)?
      }
      Ok(())
    }
  }

  /// Displays an optional item
  pub struct Optn<'a, T>(pub &'a Option<T>);

  impl<T: Display> Display for Optn<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      self.0.as_ref().map_or(Ok(()), |v| v.fmt(f))
    }
  }

  /// Displays a tuple of items without a separator
  pub struct Tuple<T>(pub T);

  impl<'a, T0: Display, T1: Display> Display for Tuple<&'a (T0, T1)> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      self.0 .0.fmt(f)?;
      self.0 .1.fmt(f)
    }
  }
}
