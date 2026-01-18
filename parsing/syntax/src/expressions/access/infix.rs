//! Infix operators
//!
//! ## Warning
//!
//! These are only used during parsing!<br>
//! They will be translated into field calls.
use crate::{expressions::Expression, ident::Name, scope::SyntaxScope};

#[derive(Clone, Debug)]
pub struct Infix<S: SyntaxScope> {
  pub value: S::Single<Expression<S>>,
  pub name: S::Single<Name>,
  pub other: S::Single<Expression<S>>,
}

// impl DisplayAs<Spans> for Infix<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("infix", &self.info)?;
//     self.value.write(&mut w.child())?;
//     self.name.write(&mut w.child())?;
//     self.other.write(&mut w.child())
//   }
// }
