use crate::{expressions::Expression, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Group<S: SyntaxScope> {
  pub value: S::Single<Expression<S>>,
}

// impl DisplayAs<Spans> for Group<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("group", &self.info)?;
//     self.value.write(&mut w.child())
//   }
// }
