//! Expression parsers
//!
//! All expression parsers assume their entire input is an expression.
//! This means that if there is any remaining input after each parse,
//! they will fail.
use diom_info::InfoRef;
use diom_syntax::{
  expressions::{Expression, Infix},
  ident::{Ident, Name},
};
use diom_tokens::SpanTokens;
use nom::error::{Error, ErrorKind};
use std::cmp::Ordering;

mod infix;
use infix::parse_infix;
mod postfix;
use postfix::{parse_postfix, UnaryOperator};
mod value;
use value::parse_value;

use crate::{common::Span, errors::PResult};

/// When parsing expressions, we need to actually be somewhat careful
/// about the order that we parse different expression types in.
///
/// We need to parse in the rough order so that bracketing ends up "correct":
/// ```ignore
/// 3 > 4 - 2 + [() => 5][0]() * Some(3)? / 5 <= !False != True
/// ==
/// ((3 > ((4 - 2) + (((([() => 5])[0])()) * ((Some(3))?) / 5))) <= !False) != True
/// ```
/// <br>
///
/// First of all we need to parse expressions that are purely a prefix,<br>
/// i.e. expressions that are laid out as `<prefix> <expression>`:
/// 1. `(<arguments>) =>`
/// 1. `return`, `let <pattern> =`
///
/// As these expressions effectively "capture" the whole input.
///
/// The reason that `function`s need to be parsed before `=`<br>
/// is that there is a slight ambiguity between functions and assignments<br>
/// which is resolved by parsing functions first.
/// <br>
///
/// We then parse the infix operators, with the following precedence:
/// 1. `=`
/// 1. `&`, `|`
/// 1. `<`, `>`, `<=`, `>=`, `==`, `!=`
/// 1. `+`
/// 1. `-`
/// 1. `*`
/// 1. `/`
/// 1. `!`
///
/// And then suffix operators:
/// `.x`, `(<arguments>)`, `[<indexes>]`, `?`
///
/// Finally, we parse the basic values and constructors:
/// 1. chars and floats
/// 1. structs, arrays, tuples
/// 1. groups, blocks
///
/// ---
///
/// ## _____
///
/// A modified shunting yard algorithm for parsing expressions.
///
/// We keep a stack of expressions in two effective halves:
/// `[...fields, current]` where
/// - `fields` are field expressions in increasing order of precedence
/// - `current` is the current expression being worked on
///
/// when we create a field expression from `current`, while the previous<br>
/// field expression has higher precedence than `current`, we combine it<br>
/// with `current`.
pub fn parse_expression(input: SpanTokens) -> PResult<Expression<Span>> {
  // all operators are field expressions with increasing precedence
  let (mut input, expr) = parse_value(input)?;
  let mut shunter = ShuntingYard::new(expr);

  loop {
    // attempt to parse infix expressions first
    // they require the `<expr> <op> <expr>` which is reused by postfix ops
    // this means that, if we parse postfix first, it'll take over
    if let Ok((input_, (name, value))) = parse_infix(input) {
      input = input_;
      if let Err(err) = shunter.push_infix((value, name)) {
        return Err(nom::Err::Error(Error::new(input, err.into())));
      };
      continue;
    }

    // then postfix expressions
    if let Ok((input_, postfix)) = parse_postfix(input) {
      input = input_;
      shunter.apply(postfix);
      continue;
    }

    // once both parsers fail, break
    break;
  }

  shunter
    .finish()
    .map(|expr| (input, expr))
    .map_err(|err| nom::Err::Error(Error::new(input, err.into())))
}

/// A structure implementing the shunting yard algorithm.
///
/// 2 main invariants are maintained by this structure:
///
/// 1. `values.len() >= 1`, i.e. `values` is non-empty
/// 2. `for i in 0..infix.len() - 1 { infix[i] < infix[i+1] }`<br>
///    i.e. infix is stored in strictly increasing order.
pub struct ShuntingYard {
  values: Vec<Expression<Span>>,
  infix: Vec<Ident<Span>>,
}

impl ShuntingYard {
  pub fn new(value: Expression<Span>) -> Self {
    Self {
      values: vec![value],
      infix: vec![],
    }
  }

  /// Pops a single infix operator off the stack and applies it.
  fn shunt(&mut self, iop: Ident<Span>) -> Result<(), ShuntingErrorKind> {
    let expr1 = self.values.pop().unwrap(/* invariant 1 */);
    let expr0 = self.values.pop().ok_or(ShuntingErrorKind::MissingValue)?;
    self.values.push(Expression::Infix(Infix {
      info: expr0.info().start..expr1.info().end,
      value: Box::new(expr0),
      name: iop,
      other: Box::new(expr1),
    }));
    Ok(())
  }

  /// Applies all infix operators after position `i`
  fn shunt_from(&mut self, i: usize) -> Result<(), ShuntingErrorKind> {
    let infix = self.infix.split_off(i);
    for iop in infix.into_iter().rev() {
      self.shunt(iop)?;
    }
    Ok(())
  }

  pub fn apply(&mut self, postfix: impl UnaryOperator<Info = Span>) {
    let expr = self.values.pop().unwrap(/* invariant 1 */);
    self.values.push(postfix.apply(expr));
  }

  pub fn push_infix(
    &mut self,
    (value, iop): (Expression<Span>, Ident<Span>),
  ) -> Result<(), ShuntingErrorKind> {
    self.values.push(value);

    // invariant 2 => we can binary search `infix`
    let res = self.infix.binary_search_by(|op| infix_cmp(&iop, op));
    let i = res.unwrap_or_else(|e| e);
    // invariant 2 => all operators after `i` have greater precedence
    self.shunt_from(i)?;

    self.infix.push(iop);
    Ok(())
  }

  pub fn finish(mut self) -> Result<Expression<Span>, ShuntingErrorKind> {
    // apply all remaining infix operators
    self.shunt_from(0)?;

    match self.values.len() {
      0 => Err(ShuntingErrorKind::MissingValue),
      1 => Ok(self.values.pop().unwrap(/* we've checked len == 1 */)),
      _ => Err(ShuntingErrorKind::ExtraValue),
    }
  }
}

/// Implement operator precedence via PartialEq and PartialOrd
///
/// Precedence (high to low):
/// 1. `*`, `/`
/// 2. `+`, `-`
/// 3. `==`, `!=`, `<`, `>`, `<=`, `>=`
/// 4. `&`, `|`
pub fn infix_cmp<I>(op0: &Ident<I>, op1: &Ident<I>) -> Ordering {
  use Name::*;
  match (&op0.name, &op1.name) {
    (Times | Divide, Times | Divide) => Ordering::Equal,
    (Times | Divide, _) => Ordering::Greater,
    (_, Times | Divide) => Ordering::Less,

    (Plus | Minus, Plus | Minus) => Ordering::Equal,
    (Plus | Minus, _) => Ordering::Greater,
    (_, Plus | Minus) => Ordering::Less,

    (Eq | Ne | Lt | Gt | LtEq | GtEq, Eq | Ne | Lt | Gt | LtEq | GtEq) => Ordering::Equal,
    (Eq | Ne | Lt | Gt | LtEq | GtEq, _) => Ordering::Greater,
    (_, Eq | Ne | Lt | Gt | LtEq | GtEq) => Ordering::Less,

    (And | Or | Not, And | Or | Not) => Ordering::Equal,
    (And | Or | Not, _) => Ordering::Greater,
    (_, And | Or | Not) => Ordering::Less,

    (Literal(_), Literal(_)) => Ordering::Equal,
  }
}

/// Possible errors in the shunting yard algorithm
pub enum ShuntingErrorKind {
  MissingValue,
  ExtraValue,
}

impl From<ShuntingErrorKind> for ErrorKind {
  fn from(value: ShuntingErrorKind) -> Self {
    use ShuntingErrorKind::*;
    match value {
      MissingValue => ErrorKind::Eof,
      ExtraValue => ErrorKind::NonEmpty,
    }
  }
}
