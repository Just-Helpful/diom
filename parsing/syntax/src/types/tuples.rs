use crate::scope::SyntaxScope;

/// The type for a combination of indexed fields
///
/// ```ignore
/// let Vec2 [Float, Float];
/// let Email: [String, String];
///
/// let vec2: Vec2 = Vec2 [1.2, 3.0];
/// let bobs_email: Email = ["bob.jones", "hotmail.com"];
/// ```
#[derive(Clone, Debug)]
pub struct Tuple<S: SyntaxScope> {
  pub name: Option<S::Ident>,
  pub fields: Vec<S::Type>,
}

// impl DisplayAs<Spans> for Tuple<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("tuple", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.fields.write(&mut w.child())
//   }
// }
