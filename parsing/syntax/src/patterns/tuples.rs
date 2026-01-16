use super::Rest;
use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub enum TupleItem<S: SyntaxScope> {
  Field(S::Pattern),
  Rest(Rest<S>),
}

// impl DisplayAs<Spans> for TupleItem<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     match self {
//       Self::Field(f) => f.write(w),
//       Self::Rest(r) => r.write(w),
//     }
//   }
// }

#[derive(Clone, Debug)]
pub struct Tuple<S: SyntaxScope> {
  pub name: Option<S::Path>,
  pub fields: Vec<TupleItem<S>>,
}

// impl DisplayAs<Spans> for Tuple<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("tuple", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.fields.write(&mut w.child())
//   }
// }
