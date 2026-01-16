use super::Rest;
use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub enum ArrayItem<S: SyntaxScope> {
  Item(S::Pattern),
  Rest(Rest<S>),
}

// impl DisplayAs<Spans> for ArrayItem<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     match self {
//       Self::Item(i) => i.write(w),
//       Self::Rest(r) => r.write(w),
//     }
//   }
// }

#[derive(Clone, Debug)]
pub struct Array<S: SyntaxScope> {
  pub name: Option<S::Path>,
  pub items: Vec<ArrayItem<S>>,
}

// impl DisplayAs<Spans> for Array<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("array", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.items.write(&mut w.child())
//   }
// }
