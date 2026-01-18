use crate::{expressions::Expression, ident::Name, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Field<S: SyntaxScope> {
  pub value: S::Single<Expression<S>>,
  pub name: S::Single<Name>,
}

// impl DisplayAs<Spans> for Field<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("field", &self.info)?;
//     self.value.write(&mut w.child())?;
//     self.name.write(&mut w.child())
//   }
// }
