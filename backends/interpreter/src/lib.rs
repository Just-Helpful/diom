use diom_syntax::{
  expressions::{Expression, Infix},
  ident::{Ident, Name},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Float(f64),
  Bool(bool),
  Char(char),
  Array(Array),
  Struct(Struct),
}

type Array = Vec<Value>;
type Struct = HashMap<Ident<()>, Value>;

#[derive(Debug)]
pub enum Error {
  Unsupported(&'static str),
  NotStruct(Value, Ident<()>),
  MissingField(Struct, Ident<()>),
  NotArray(Value, Vec<Expr>),
  TooManyKeys(Vec<Value>, Vec<Expr>),
  IndexMissing(Vec<Value>),
  IndexNotInt(Vec<Value>, Value),
  IndexOutsideBounds(Vec<Value>, usize, usize),
}

type Expr = Expression<()>;

pub fn interpret_infix(
  Infix {
    value, name, other, ..
  }: Infix<()>,
) -> Result<Value, Error> {
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

pub fn interpret_expr(expr: Expr) -> Result<Value, Error> {
  use Expression::*;
  match expr {
    Char(c) => Ok(Value::Char(c.value)),
    Float(f) => Ok(Value::Float(f.value)),
    Var(_) => Err(Error::Unsupported("Variables")),
    Group(_) => unreachable!("We shouldn't have groups at this point"),
    Block(_) => Err(Error::Unsupported("Blocks")),
    Assign(_) => Err(Error::Unsupported("Assignments")),
    Declare(_) => Err(Error::Unsupported("Declarations")),
    Return(_) => Err(Error::Unsupported("Returns")),
    Array(arr) => arr
      .contents
      .into_iter()
      .map(interpret_expr)
      .collect::<Result<Vec<Value>, _>>()
      .map(Value::Array),
    Function(_) => Err(Error::Unsupported("Functions")),
    Struct(data) => data
      .fields
      .into_iter()
      .map(|(ident, item)| interpret_expr(item).map(|val| (ident, val)))
      .collect::<Result<HashMap<Ident<()>, Value>, _>>()
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
