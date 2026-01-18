use crate::{expressions::Expression, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Return<S: SyntaxScope> {
  pub value: S::Single<Expression<S>>,
}

// impl DisplayAs<Spans> for Return<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("return", &self.info)?;
//     self.value.write(&mut w.child())
//   }
// }
