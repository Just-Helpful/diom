use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Group<S: SyntaxScope> {
  pub value: S::Expression,
}

// impl DisplayAs<Spans> for Group<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("group", &self.info)?;
//     self.value.write(&mut w.child())
//   }
// }
