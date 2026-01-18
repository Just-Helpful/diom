use crate::{ident::Name, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Path<S: SyntaxScope> {
  pub segments: S::Multi<Name>,
}

// impl DisplayAs<Spans> for Path<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("path", &self.info)?;
//     self.segments.write(&mut w.child())
//   }
// }
