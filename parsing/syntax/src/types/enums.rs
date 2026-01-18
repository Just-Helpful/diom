use crate::{ident::Name, scope::SyntaxScope, types::Type};

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
  pub name: Option<S::Single<Name>>,
  pub variants: S::Multi<(Name, Type<S>)>,
}

// impl DisplayAs<Spans> for Enum<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("enum", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.variants.write(&mut w.child())
//   }
// }
