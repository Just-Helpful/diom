use crate::scope::SyntaxScope;

/// A type for arrays of items.
///
/// ```ignore
/// type String [Char; _];
/// type Nums = [Float];
///
/// let greeting: String = "Hello!";
/// let xs: Nums = [1, 2, 3];
/// ```
#[derive(Clone, Debug)]
pub struct Array<S: SyntaxScope> {
  pub name: Option<S::Ident>,
  pub item: S::Type,
}

// impl DisplayAs<Spans> for Array<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("array", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.item.write(&mut w.child())
//   }
// }
