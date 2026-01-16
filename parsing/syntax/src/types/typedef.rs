use crate::scope::SyntaxScope;

/// The definition for a type alias
///
/// ```_,ignore
/// type Number = Float;
/// type Boolean = Bool;
///
/// type Vec2_0 = {x: Float, y: Float};
/// type Vec2_1 {x: Float, y: Float};
///
/// type Vec3_0 = [Float, Float, Float];
/// type Vec3_1 [Float, Float, Float];
///
/// type VecN_0 = [Float; _];
/// type VecN_1 [Float; _];
/// ```
#[derive(Clone, Debug)]
pub struct TypeDef<S: SyntaxScope> {
  pub name: S::Ident,
  pub value: Box<S::Type>,
}

// impl DisplayAs<Spans> for TypeDef<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("type def", &self.info)?;
//     self.name.write(&mut w.child())?;
//     self.value.write(&mut w.child())
//   }
// }
