use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Field<S: SyntaxScope> {
  pub value: Box<S::Expression>,
  pub name: S::Ident,
}

// impl DisplayAs<Spans> for Field<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("field", &self.info)?;
//     self.value.write(&mut w.child())?;
//     self.name.write(&mut w.child())
//   }
// }
