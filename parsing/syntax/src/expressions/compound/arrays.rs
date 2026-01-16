use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Array<S: SyntaxScope> {
  pub contents: Vec<S::Expression>,
}

// impl DisplayAs<Spans> for Array<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("array", &self.info)?;
//     self.contents.write(&mut w.child())
//   }
// }
