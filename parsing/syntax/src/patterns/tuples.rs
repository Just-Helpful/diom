use super::Rest;
use crate::{path::Path, patterns::Pattern, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub enum TupleItem<S: SyntaxScope> {
  Field(Pattern<S>),
  Rest(Rest),
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
  pub name: Option<S::Single<Path<S>>>,
  pub fields: S::Multi<TupleItem<S>>,
}

// impl DisplayAs<Spans> for Tuple<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("tuple", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.fields.write(&mut w.child())
//   }
// }
