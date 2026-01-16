use crate::scope::SyntaxScope;

#[derive(Clone, Debug)]
pub struct Argument<S: SyntaxScope> {
  pub pattern: S::Pattern,
  pub annotation: Option<S::Type>,
}

// impl DisplayAs<Spans> for Argument<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("argument", &self.info)?;
//     self.pattern.write(&mut w.child())?;
//     self.annotation.write(&mut w.child())
//   }
// }

#[derive(Clone, Debug)]
pub struct FunctionArm<S: SyntaxScope> {
  pub arguments: Vec<Argument<S>>,
  pub annotation: Option<S::Type>,
  pub returned: Box<S::Expression>,
}

// impl DisplayAs<Spans> for FunctionArm<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("arm", &self.info)?;
//     self.arguments.write(&mut w.child())?;
//     self.annotation.write(&mut w.child())?;
//     self.returned.write(&mut w.child())
//   }
// }

#[derive(Clone, Debug)]
pub struct Function<S: SyntaxScope> {
  pub arms: Vec<FunctionArm<S>>,
}

// impl DisplayAs<Spans> for Function<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("function", &self.info)?;
//     self.arms.write(&mut w.child())
//   }
// }
