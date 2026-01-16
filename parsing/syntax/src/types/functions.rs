use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Argument<S: SyntaxScope> {
  pub name: S::Ident,
  pub annotation: S::Type,
}

// impl DisplayAs<Spans> for Argument<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("argument", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.annotation.write(&mut w.child())
//   }
// }

/// The type for a callable function
///
/// ```ignore
/// # function types can be simplified a bit
/// type Binary = (x: Float): (y: Float): Float;
/// type Binary = (x: Float)(y: Float): Float;
///
/// let add: Binary = (x)(y) => x + y;
/// let add: Binary = (x) => {(y) => {x + y}};
/// ```
#[derive(Clone, Debug)]
pub struct Function<S: SyntaxScope> {
  pub arguments: Vec<Argument<S>>,
  pub returned: S::Expression,
}

// impl DisplayAs<Spans> for Function<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("function", &self.info)?;
//     self.arguments.write(&mut w.child())?;
//     self.returned.write(&mut w.child())
//   }
// }
