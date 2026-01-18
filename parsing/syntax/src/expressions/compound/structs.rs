use crate::{expressions::Expression, ident::Name, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Struct<S: SyntaxScope> {
  pub fields: S::Multi<(Name, Expression<S>)>,
}

// impl DisplayAs<Spans> for Struct<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("struct", &self.info)?;
//     self.fields.write(&mut w.child())
//   }
// }
