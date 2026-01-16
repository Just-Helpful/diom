use crate::scope::SyntaxScope;

/// A type for combinations of possible types
///
/// ```ignore
/// type CharOption {
///   Some(Char),
///   None,
/// };
///
/// type Boolean: {
///   True,
///   False,
/// };
///
/// let c_optn = CharOption.Some('v');
/// let c_optn = CharOption.None;
///
/// let bool = Boolean.True;
/// let bool = Boolean.False;
/// ```
#[derive(Clone, Debug)]
pub struct Enum<S: SyntaxScope> {
  pub name: Option<S::Ident>,
  pub variants: Vec<(S::Ident, S::Type)>,
}

// impl DisplayAs<Spans> for Enum<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("enum", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.variants.write(&mut w.child())
//   }
// }
