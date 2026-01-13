use diom_syntax::{
  expressions::{Expression, Infix, Statement},
  ident::{Ident, Name},
  patterns::Pattern,
};
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Unit,
  Float(f64),
  Bool(bool),
  Char(char),
  Array(Array),
  Struct(Struct),
}

type Array = Vec<Value>;
type Struct = HashMap<Name, Value>;

type Scope = HashMap<Name, Value>;

#[derive(Debug)]
pub enum Error<I> {
  Unsupported(&'static str),
  Type(&'static str),
  MissingVar(Ident<I>),
  NotStruct(Value, Ident<I>),
  MissingField(Struct, Ident<I>),
  NotArray(Value, Vec<Expression<I>>),
  TooManyKeys(Vec<Value>, Vec<Expression<I>>),
  IndexMissing(Vec<Value>),
  IndexNotInt(Vec<Value>, Value),
  IndexOutsideBounds(Vec<Value>, usize, usize),
}

/// A type that can be evaluated to a given value when given a starting state
pub trait Eval<S: Default = ()> {
  /// The output value produced when the type is evaluated
  type Output;
  /// Errors produced when evaluating `Self`
  type Error;

  /// Evaluate `self` with the starting state `state`
  fn eval_with(&self, state: &mut S) -> Result<Self::Output, Self::Error>;

  /// Evaluate `self` with the default "empty" starting state
  fn eval(&self) -> Result<Self::Output, Self::Error> {
    self.eval_with(&mut S::default())
  }
}

impl<I: Clone> Eval<Scope> for Infix<I> {
  type Output = Value;
  type Error = Error<I>;

  fn eval_with(&self, state: &mut Scope) -> Result<Self::Output, Self::Error> {
    use Name::*;
    use Value::*;
    let Infix {
      value,
      name: Ident { name, .. },
      other,
      ..
    } = self;
    let value = value.eval_with(state)?;
    let other = other.eval_with(state)?;
    match name {
      Literal(_) => Err(Error::Unsupported("Methods")),
      Not => Err(Error::Unsupported("Not as infix")),
      And => {
        let (Bool(lhs), Bool(rhs)) = (value, other) else {
          return Err(Error::Type("And on non-`bool`s"));
        };
        Ok(Bool(lhs & rhs))
      }
      Or => {
        let (Bool(lhs), Bool(rhs)) = (value, other) else {
          return Err(Error::Type("Or on non-`bool`s"));
        };
        Ok(Bool(lhs | rhs))
      }
      Plus => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("Plus on non-`float`s"));
        };
        Ok(Float(lhs + rhs))
      }
      Minus => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("Minus on non-`float`s"));
        };
        Ok(Float(lhs - rhs))
      }
      Times => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("Times on non-`float`s"));
        };
        Ok(Float(lhs * rhs))
      }
      Divide => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("Divide on non-`float`s"));
        };
        Ok(Float(lhs / rhs))
      }
      Eq => Ok(Bool(value == other)),
      Ne => Ok(Bool(value != other)),
      Lt => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("Lt on non-`float`s"));
        };
        Ok(Bool(lhs < rhs))
      }
      Gt => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("Gt on non-`float`s"));
        };
        Ok(Bool(lhs > rhs))
      }
      LtEq => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("LtEq on non-`float`s"));
        };
        Ok(Bool(lhs <= rhs))
      }
      GtEq => {
        let (Float(lhs), Float(rhs)) = (value, other) else {
          return Err(Error::Type("GtEq on non-`float`s"));
        };
        Ok(Bool(lhs >= rhs))
      }
    }
  }
}

impl<I: Clone> Eval<Scope> for Statement<I> {
  type Output = Value;
  type Error = Error<I>;

  fn eval_with(&self, state: &mut Scope) -> Result<Self::Output, Self::Error> {
    use Statement::*;
    match self {
      TypeDef(_) => Err(Error::Unsupported("Types")),
      Expression(expr) => expr.eval_with(state),
    }
  }
}

impl<I: Clone> Eval<Scope> for Expression<I> {
  type Output = Value;
  type Error = Error<I>;

  fn eval_with(&self, state: &mut Scope) -> Result<Self::Output, Self::Error> {
    use Expression::*;
    match self {
      Char(c) => Ok(Value::Char(c.value)),
      Float(f) => Ok(Value::Float(f.value)),
      Var(v) => {
        let Some(value) = state.get(&v.name) else {
          return Err(Error::MissingVar(v.clone()));
        };
        Ok(value.clone())
      }
      Group(group) => group.value.eval_with(state),
      Block(block) => {
        let mut inner = state.clone();
        block
          .statements
          .iter()
          .try_fold(Value::Unit, |_, stmt| stmt.eval_with(&mut inner))
      }
      Assign(a) => {
        let Expression::Var(v) = a.reference.deref() else {
          return Err(Error::Unsupported("Assignments to non-variables"));
        };
        let value = a.value.eval_with(state)?;
        let Some(entry) = state.get_mut(&v.name) else {
          return Err(Error::Unsupported("Assignment to non-existant variables"));
        };
        // @todo this breaks reference updates, i.e. this fails:
        // `(x = {value: 5}).value = 3; assert(x.value == 3)`
        // You'll probably want to an `Rc<RefCell<...>>` or similar
        *entry = value.clone();
        Ok(value)
      }
      Declare(d) => {
        let Pattern::Var(v) = &d.pattern else {
          return Err(Error::Unsupported("`let` for non-variables"));
        };
        let value = d.value.eval_with(state)?;
        state.insert(v.name.clone(), value);
        Ok(Value::Bool(true)) // pattern matching succeeded

        // @todo I probably need to consider what `let` statements / pattern matching
        // should "return" as a value. Currently a `bool` doesn't really work too well
        // with the type system when we try to use:
        // ```
        // let Some(x) = value else { return None }
        // // or
        // if let Some(x) = value { x + 1 }
        // ```
        // because we don't have type guarantees that `x` exists with a `bool`
      }
      Return(_) => Err(Error::Unsupported("Returns")),
      Array(arr) => arr
        .contents
        .iter()
        .map(|e| e.eval_with(state))
        .collect::<Result<Vec<Value>, _>>()
        .map(Value::Array),
      Function(_) => Err(Error::Unsupported("Functions")),
      Struct(data) => data
        .fields
        .iter()
        .map(|(ident, item)| item.eval_with(state).map(|val| (ident.name.clone(), val)))
        .collect::<Result<HashMap<Name, Value>, _>>()
        .map(Value::Struct),
      Call(_) => Err(Error::Unsupported("Functions")),
      Field(field) => {
        let value = field.value.eval_with(state)?;
        let Value::Struct(value) = value else {
          return Err(Error::NotStruct(value, field.name.clone()));
        };
        let Some(value) = value.get(&field.name.name) else {
          return Err(Error::MissingField(value, field.name.clone()));
        };
        Ok(value.clone())
      }
      Index(index) => {
        let mut index = index.clone();
        let value = index.value.eval_with(state)?;
        let Value::Array(value) = value else {
          return Err(Error::NotArray(value, index.key));
        };
        if index.key.len() > 1 {
          return Err(Error::TooManyKeys(value, index.key));
        }
        let Some(key) = index.key.pop() else {
          return Err(Error::IndexMissing(value));
        };
        let idx = key.eval_with(state)?;
        let Value::Float(idx) = idx else {
          return Err(Error::IndexNotInt(value, idx));
        };
        if idx.fract() != 0.0 {
          return Err(Error::IndexNotInt(value, Value::Float(idx)));
        };
        let idx = idx.floor() as usize;
        let len = value.len();
        if !(0..len).contains(&idx) {
          return Err(Error::IndexOutsideBounds(value, len, idx));
        }
        return Ok(value[idx].clone());
      }
      Infix(infix) => infix.eval_with(state),
      Monad(_) => Err(Error::Unsupported("Monads")),
    }
  }
}
