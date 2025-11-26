use diom_syntax::{
  expressions::{Expression, Infix},
  ident::{Ident, Name},
};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub enum Value<I: Hash + Eq> {
  Float(f64),
  Bool(bool),
  Char(char),
  Array(Array<I>),
  Struct(Struct<I>),
}

type Array<I> = Vec<Value<I>>;
type Struct<I> = HashMap<Ident<I>, Value<I>>;

#[derive(Debug)]
pub enum Error<I: Hash + Eq> {
  Unsupported(&'static str),
  NotStruct(Value<I>, Ident<I>),
  MissingField(Struct<I>, Ident<I>),
  NotArray(Value<I>, Vec<Expression<I>>),
  TooManyKeys(Vec<Value<I>>, Vec<Expression<I>>),
  IndexMissing(Vec<Value<I>>),
  IndexNotInt(Vec<Value<I>>, Value<I>),
  IndexOutsideBounds(Vec<Value<I>>, usize, usize),
}

pub fn interpret_infix<I: Hash + Eq + Clone>(
  Infix {
    value, name, other, ..
  }: Infix<I>,
) -> Result<Value<I>, Error<I>> {
  use Name::*;
  use Value::*;
  let Ident { name, .. } = name;
  let value = interpret_expr(*value)?;
  let other = interpret_expr(*other)?;
  match name {
    Literal(_) => Err(Error::Unsupported("Methods")),
    Not => Err(Error::Unsupported("Not as infix")),
    And => {
      let (Bool(lhs), Bool(rhs)) = (value, other) else {
        return Err(Error::Unsupported("And on non-`bool`s"));
      };
      Ok(Bool(lhs & rhs))
    }
    Or => {
      let (Bool(lhs), Bool(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Or on non-`bool`s"));
      };
      Ok(Bool(lhs | rhs))
    }
    Plus => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Plus on non-`float`s"));
      };
      Ok(Float(lhs + rhs))
    }
    Minus => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Minus on non-`float`s"));
      };
      Ok(Float(lhs - rhs))
    }
    Times => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Times on non-`float`s"));
      };
      Ok(Float(lhs * rhs))
    }
    Divide => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Divide on non-`float`s"));
      };
      Ok(Float(lhs / rhs))
    }
    Eq => Ok(Bool(value == other)),
    Ne => Ok(Bool(value != other)),
    Lt => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Lt on non-`float`s"));
      };
      Ok(Bool(lhs < rhs))
    }
    Gt => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("Gt on non-`float`s"));
      };
      Ok(Bool(lhs > rhs))
    }
    LtEq => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("LtEq on non-`float`s"));
      };
      Ok(Bool(lhs <= rhs))
    }
    GtEq => {
      let (Float(lhs), Float(rhs)) = (value, other) else {
        return Err(Error::Unsupported("GtEq on non-`float`s"));
      };
      Ok(Bool(lhs >= rhs))
    }
  }
}

pub fn interpret_expr<I: Hash + Eq + Clone>(expr: Expression<I>) -> Result<Value<I>, Error<I>> {
  use Expression::*;
  match expr {
    Char(c) => Ok(Value::Char(c.value)),
    Float(f) => Ok(Value::Float(f.value)),
    Var(_) => Err(Error::Unsupported("Variables")),
    Group(group) => interpret_expr(*group.value),
    Block(_) => Err(Error::Unsupported("Blocks")),
    Assign(_) => Err(Error::Unsupported("Assignments")),
    Declare(_) => Err(Error::Unsupported("Declarations")),
    Return(_) => Err(Error::Unsupported("Returns")),
    Array(arr) => arr
      .contents
      .into_iter()
      .map(interpret_expr)
      .collect::<Result<Vec<Value<I>>, _>>()
      .map(Value::Array),
    Function(_) => Err(Error::Unsupported("Functions")),
    Struct(data) => data
      .fields
      .into_iter()
      .map(|(ident, item)| interpret_expr(item).map(|val| (ident, val)))
      .collect::<Result<HashMap<Ident<I>, Value<I>>, _>>()
      .map(Value::Struct),
    Call(_) => Err(Error::Unsupported("Functions")),
    Field(field) => {
      let value = interpret_expr(*field.value)?;
      let Value::Struct(value) = value else {
        return Err(Error::NotStruct(value, field.name));
      };
      let Some(value) = value.get(&field.name) else {
        return Err(Error::MissingField(value, field.name));
      };
      Ok(value.clone())
    }
    Index(mut index) => {
      let value = interpret_expr(*index.value)?;
      let Value::Array(value) = value else {
        return Err(Error::NotArray(value, index.key));
      };
      if index.key.len() > 1 {
        return Err(Error::TooManyKeys(value, index.key));
      }
      let Some(key) = index.key.pop() else {
        return Err(Error::IndexMissing(value));
      };
      let idx = interpret_expr(key)?;
      let Value::Float(idx) = idx else {
        return Err(Error::IndexNotInt(value, idx));
      };
      if idx.fract() != 0.0 {
        return Err(Error::IndexNotInt(value, Value::Float(idx)));
      };
      let idx = idx.floor() as usize;
      let len = value.len();
      if (0..len).contains(&idx) {
        return Err(Error::IndexOutsideBounds(value, len, idx));
      }
      return Ok(value[idx].clone());
    }
    Infix(infix) => interpret_infix(infix),
    Monad(_) => Err(Error::Unsupported("Monads")),
  }
}
