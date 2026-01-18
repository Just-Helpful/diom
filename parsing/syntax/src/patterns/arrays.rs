use super::Rest;
use crate::{path::Path, patterns::Pattern, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub enum ArrayItem<S: SyntaxScope> {
  Item(Pattern<S>),
  Rest(Rest),
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
  pub name: Option<S::Single<Path<S>>>,
  pub items: S::Multi<ArrayItem<S>>,
}

// impl DisplayAs<Spans> for Array<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("array", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.items.write(&mut w.child())
//   }
// }
