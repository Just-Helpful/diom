use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Call<S: SyntaxScope> {
  pub value: S::Expression,
  pub args: Vec<S::Expression>,
}

// impl DisplayAs<Spans> for Call<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("call", &self.info)?;
//     self.value.write(&mut w.child())?;
//     self.args.write(&mut w.child())
//   }
// }
