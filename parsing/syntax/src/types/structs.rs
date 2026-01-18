use crate::{ident::Name, scope::SyntaxScope, types::Type};

/// The type for a combination of named fields.
///
/// ```ignore
/// let Vec2 {
///   x: Float,
///   y: Float,
/// };
/// let Email: {
///   name: [Char],
///   domain: [Char],
/// };
///
/// let vec2: Vec2 = Vec2 { x: 1.2, y: 3.0 };
/// let bobs_email: Email = { name: "bob.jones", domain: "hotmail.com" };
/// ```
#[derive(Clone, Debug)]
pub struct Struct<S: SyntaxScope> {
  pub name: Option<S::Single<Name>>,
  pub fields: S::Multi<(Name, Type<S>)>,
}

// impl DisplayAs<Spans> for Struct<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("struct", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.fields.write(&mut w.child())?;
//     Ok(())
//   }
// }
