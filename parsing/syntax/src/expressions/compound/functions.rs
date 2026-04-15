use super::Expression;
use crate::{
  display::Sep,
  patterns::{Pattern, PatternConfig},
  types::{Type, TypeConfig},
  Seq,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, option, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Parameter<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub info: I,
}

impl<I> Display for Parameter<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.pattern.fmt(f)?;
    if let Some(annotation) = &self.annotation {
      f.write_char(':')?;
      annotation.fmt(f)?
    }
    Ok(())
  }
}

impl DisplayAs<Spans> for Parameter<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("param", &self.info)?;
    self.pattern.write(&mut w.child())?;
    self.annotation.write(&mut w.child())
  }
}

#[derive(Default, Clone, Copy)]
pub struct ParameterConfig(
  /// The config used to generate patterns
  pub PatternConfig,
  /// The config used to generate types
  pub TypeConfig,
);
impl Parameter<()> {
  /// Generates a generic strategy for generating `Parameter` nodes
  pub fn any(args: ParameterConfig) -> impl Strategy<Value = Self> {
    (Pattern::any(args.0), option::of(Type::any(args.1))).prop_map(|(pattern, annotation)| {
      Parameter {
        pattern,
        annotation,
        info: (),
      }
    })
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Parameters<I> {
  pub parameters: Seq<Parameter<I>>,
  pub info: I,
}

impl<I> Display for Parameters<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('(')?;
    Sep(&self.parameters, ',').fmt(f)?;
    f.write_char(')')
  }
}

impl DisplayAs<Spans> for Parameters<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("params", &self.info)?;
    self.parameters.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct ParametersConfig(
  /// The config used to generate patterns
  pub PatternConfig,
  /// The config used to generate types
  pub TypeConfig,
  /// The maximum number of parameters for a function
  pub usize,
);
impl Default for ParametersConfig {
  fn default() -> Self {
    Self(Default::default(), Default::default(), 50)
  }
}
impl From<ParametersConfig> for ParameterConfig {
  fn from(value: ParametersConfig) -> Self {
    Self(value.0, value.1)
  }
}
impl Parameters<()> {
  /// Generates a generic strategy for generating `Parameter` nodes
  pub fn any(args: ParametersConfig) -> impl Strategy<Value = Self> {
    vec(Parameter::any(args.into()), 0..args.2)
      .prop_map(Seq::from_iter)
      .prop_map(|parameters| Parameters {
        parameters,
        info: (),
      })
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct FunctionArm<I> {
  pub parameters: Parameters<I>,
  pub annotation: Option<Type<I>>,
  pub returned: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for FunctionArm<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.parameters.fmt(f)?;
    if let Some(annotation) = &self.annotation {
      f.write_char(':')?;
      annotation.fmt(f)?
    }
    f.write_str("=>")?;
    self.returned.fmt(f)
  }
}

impl DisplayAs<Spans> for FunctionArm<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("arm", &self.info)?;
    self.parameters.write(&mut w.child())?;
    self.annotation.write(&mut w.child())?;
    self.returned.write(&mut w.child())
  }
}

impl FunctionArm<()> {
  /// Generates a generic strategy for generating `Parameter` nodes
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: ParametersConfig,
  ) -> impl Strategy<Value = Self> {
    (Parameters::any(args), option::of(Type::any(args.1)), item).prop_map(
      |(parameters, annotation, returned)| FunctionArm {
        parameters,
        annotation,
        returned: Box::new(returned),
        info: (),
      },
    )
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Function<I> {
  pub arms: Vec<FunctionArm<I>>,
  pub info: I,
}

impl<I> Display for Function<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('{')?;
    Sep(&self.arms, ',').fmt(f)?;
    f.write_char('}')
  }
}

impl DisplayAs<Spans> for Function<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("function", &self.info)?;
    self.arms.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct FunctionConfig(
  /// The config used to generate patterns
  pub PatternConfig,
  /// The config used to generate types
  pub TypeConfig,
  /// The maximum number of parameters for a function
  pub usize,
  /// The maximum number of function arms for a function
  pub usize,
);
impl Default for FunctionConfig {
  fn default() -> Self {
    let config = ParametersConfig::default();
    Self(config.0, config.1, config.2, 10)
  }
}
impl From<FunctionConfig> for ParametersConfig {
  fn from(value: FunctionConfig) -> Self {
    Self(value.0, value.1, value.2)
  }
}
impl Function<()> {
  /// Generates a generic strategy for generating `Function` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: FunctionConfig,
  ) -> impl Strategy<Value = Self> {
    vec(FunctionArm::any(item, args.into()), 0..args.3).prop_map(|arms| Function { arms, info: () })
  }
}
