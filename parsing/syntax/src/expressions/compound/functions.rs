use crate::{expressions::Expression, patterns::Pattern, scope::SyntaxScope, types::Type};

#[derive(Clone, Debug)]
pub struct Argument<S: SyntaxScope> {
  pub pattern: S::Single<Pattern<S>>,
  pub annotation: Option<S::Single<Type<S>>>,
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
  pub arguments: S::Multi<Argument<S>>,
  pub annotation: Option<S::Single<Type<S>>>,
  pub returned: S::Single<Expression<S>>,
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
  pub arms: S::Multi<FunctionArm<S>>,
}

// impl DisplayAs<Spans> for Function<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("function", &self.info)?;
//     self.arms.write(&mut w.child())
//   }
// }
