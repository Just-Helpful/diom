use crate::{expressions::Expression, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Call<S: SyntaxScope> {
  pub value: S::Single<Expression<S>>,
  pub args: S::Multi<Expression<S>>,
}

// impl DisplayAs<Spans> for Call<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("call", &self.info)?;
//     self.value.write(&mut w.child())?;
//     self.args.write(&mut w.child())
//   }
// }
