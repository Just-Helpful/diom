use super::Rest;
use crate::{ident::Name, path::Path, patterns::Pattern, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct StructField<S: SyntaxScope> {
  pub name: S::Single<Name>,
  pub pattern: S::Single<Pattern<S>>,
}

// impl DisplayAs<Spans> for StructField<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("field", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.pattern.write(&mut w.child())
//   }
// }

#[derive(Clone, Debug)]
pub enum StructItem<S: SyntaxScope> {
  Field(StructField<S>),
  Rest(Rest),
}

// impl DisplayAs<Spans> for StructItem<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     match self {
//       Self::Field(f) => f.write(w),
//       Self::Rest(r) => r.write(w),
//     }
//   }
// }

#[derive(Clone, Debug)]
pub struct Struct<S: SyntaxScope> {
  pub name: Option<S::Single<Path<S>>>,
  pub fields: S::Multi<StructItem<S>>,
}

// impl DisplayAs<Spans> for Struct<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("struct", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.fields.write(&mut w.child())
//   }
// }
