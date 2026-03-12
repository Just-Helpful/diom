use super::Type;
use crate::{display::Sep, ident::Ident};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{
  collection::vec,
  prelude::{any, Strategy},
};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Parameter<I> {
  pub name: Ident<I>,
  pub annotation: Type<I>,
  pub info: I,
}

impl<I> Display for Parameter<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)?;
    f.write_char(':')?;
    self.annotation.fmt(f)
  }
}

impl DisplayAs<Spans> for Parameter<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("param", &self.info)?;
    self.name.write(&mut w.child())?;
    self.annotation.write(&mut w.child())
  }
}

impl Parameter<()> {
  /// Generates a generic strategy for generating `Parameter`s
  pub fn any(item: impl Strategy<Value = Type<()>>) -> impl Strategy<Value = Self> {
    (any::<Ident<()>>(), item).prop_map(|(name, annotation)| Parameter {
      name,
      annotation,
      info: (),
    })
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Parameters<I> {
  pub parameters: Vec<Parameter<I>>,
  pub info: I,
}

impl<I> Display for Parameters<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Sep(&self.parameters, ',').fmt(f)
  }
}

impl DisplayAs<Spans> for Parameters<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("params", &self.info)?;
    self.parameters.write(&mut w.child())
  }
}

impl Parameters<()> {
  /// Generates a generic strategy for generating `Parameters` structures
  pub fn any(
    item: impl Strategy<Value = Type<()>>,
    args: FunctionConfig,
  ) -> impl Strategy<Value = Self> {
    vec(Parameter::any(item), 0..args.0).prop_map(|parameters| Parameters {
      parameters,
      info: (),
    })
  }
}

/// The type for a callable function
///
/// ```_
/// # function types can be simplified a bit
/// type Binary = (x: Float): (y: Float): Float;
/// type Binary = (x: Float)(y: Float): Float;
///
/// let add: Binary = (x)(y) => x + y;
/// let add: Binary = (x) => {(y) => {x + y}};
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Function<I> {
  pub parameters: Parameters<I>,
  pub returned: Box<Type<I>>,
  pub info: I,
}

impl<I> Display for Function<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('(')?;
    self.parameters.fmt(f)?;
    f.write_str("):")?;
    self.returned.fmt(f)
  }
}

impl DisplayAs<Spans> for Function<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("function", &self.info)?;
    self.parameters.write(&mut w.child())?;
    self.returned.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct FunctionConfig(
  /// The maximum number of parameters per function
  pub usize,
);
impl Default for FunctionConfig {
  fn default() -> Self {
    Self(10)
  }
}
impl Function<()> {
  /// Generates a generic strategy for generating `Function` types
  pub fn any(
    item: impl Strategy<Value = Type<()>> + Clone,
    args: FunctionConfig,
  ) -> impl Strategy<Value = Self> {
    (Parameters::any(item.clone(), args), item).prop_map(|(parameters, returned)| Function {
      parameters,
      returned: Box::new(returned),
      info: (),
    })
  }
}
