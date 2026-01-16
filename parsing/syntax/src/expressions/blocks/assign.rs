use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Assign<S: SyntaxScope> {
  pub reference: S::Expression,
  pub value: S::Expression,
}

// impl DisplayAs<Spans> for Assign<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("assign", &self.info)?;
//     self.reference.write(&mut w.child())?;
//     self.value.write(&mut w.child())
//   }
// }
