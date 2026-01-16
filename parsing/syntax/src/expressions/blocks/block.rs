use crate::{scope::SyntaxScope, types::TypeDef};

#[derive(Clone, Debug)]
pub enum Statement<S: SyntaxScope> {
  Expression(S::Expression),
  TypeDef(TypeDef<S>),
}

// impl DisplayAs<Spans> for Statement<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     match self {
//       Self::Expression(e) => e.write(w),
//       Self::TypeDef(d) => d.write(w),
//     }
//   }
// }

#[derive(Clone, Debug)]
pub struct Block<S: SyntaxScope> {
  pub statements: Vec<Statement<S>>,
}

// impl DisplayAs<Spans> for Block<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("block", &self.info)?;
//     self.statements.write(&mut w.child())
//   }
// }
