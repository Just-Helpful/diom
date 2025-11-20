//! A vast majority of the operators that we apply to values
//! can be expressed in a postfix operators, i.e. `<expr> <operator>`.
//!
//! For example:
//! - monadic application `<expr>?` is clearly postfix
//! - function application `<expr>(x, 1)` is equal to `<expr> <apply (x, 1)>`
//!   which effectively "creates" the postfix operator from the `args`
//! - similarly, indexing `<expr>[0, 3]` is equal to `<expr> <index [0, 3]>`
//! - field access `<expr>.id` is equal to `<expr> <access id>`
//! - operators `<expr> + 4` is equal to `<expr> <access +> <apply 4>`
//! - assignment `<expr> = x + 2` is equal to `<expr> <assign (x + 2)>`
//!
//! Hence we can go about expression parsing by parsing an initial value
//! and then parsing postfix operators and combining them with the expression.
use super::{parse_expression, Expression};
use crate::{common::PResult, Span};
use diom_info_traits::{InfoRef, InfoSource};
use nom::{branch::alt, Parser};

mod assign;
use assign::parse_assign;
use assign::AssignOp;
mod call;
use call::parse_call;
use call::CallOp;
mod field;
use diom_tokens::SpanTokens;
use field::parse_field;
use field::FieldOp;
mod index;
use index::parse_index;
use index::IndexOp;
mod monads;
use monads::parse_monad;
use monads::MonadOp;

/// Any Unary operator that can be applied to an Expression
pub trait UnaryOperator: InfoRef {
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info>;
}

/// We need to parse in the rough order so that bracketing ends up "correct":
/// ```ignore
/// 3 > 4 - 2 + [() => 5][0]() * Some(3)? / 5 <= !False != True
/// ===
/// ((3 > ((4 - 2) + (((([() => 5])[0])()) * ((Some(3))?) / 5))) <= !False) != True
/// ```
///
/// This gives us the following precedence (highest first):
/// 1. Postfix Operators:
///     - Field Access
///     - Indexing
///     - Function Calls
///     - Monadic Unpacking
///     - Assignment
/// 2. Infix Operators
#[derive(InfoSource, InfoRef)]
pub enum PostfixOp<I> {
  Field(FieldOp<I>),
  Index(IndexOp<I>),
  Call(CallOp<I>),
  Monad(MonadOp<I>),
  Assign(AssignOp<I>),
}

impl<I> UnaryOperator for PostfixOp<I>
where
  AssignOp<I>: UnaryOperator<Info = I>,
  CallOp<I>: UnaryOperator<Info = I>,
  FieldOp<I>: UnaryOperator<Info = I>,
  IndexOp<I>: UnaryOperator<Info = I>,
  MonadOp<I>: UnaryOperator<Info = I>,
{
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info> {
    match self {
      Self::Assign(op) => op.apply(expr),
      Self::Call(op) => op.apply(expr),
      Self::Field(op) => op.apply(expr),
      Self::Index(op) => op.apply(expr),
      Self::Monad(op) => op.apply(expr),
    }
  }
}

pub fn parse_postfix(input: SpanTokens) -> PResult<PostfixOp<Span>> {
  alt((
    parse_assign.map(PostfixOp::Assign),
    parse_index.map(PostfixOp::Index),
    parse_call.map(PostfixOp::Call),
    parse_field.map(PostfixOp::Field),
    parse_monad.map(PostfixOp::Monad),
  ))(input)
}
