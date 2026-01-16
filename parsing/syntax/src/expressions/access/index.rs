use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Index<S: SyntaxScope> {
  pub value: S::Expression,
  pub key: Vec<S::Expression>,
}

// impl DisplayAs<Spans> for Index<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("index", &self.info)?;
//     self.value.write(&mut w.child())?;
//     self.key.write(&mut w.child())
//   }
// }
