use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Struct<S: SyntaxScope> {
  pub fields: Vec<(S::Ident, S::Expression)>,
}

// impl DisplayAs<Spans> for Struct<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("struct", &self.info)?;
//     self.fields.write(&mut w.child())
//   }
// }
