//! Expression parsers
//!
//! All expression parsers assume their entire input is an expression.
//! This means that if there is any remaining input after each parse,
//! they will fail.
use crate::{
  errors::{BuildError, SyntaxError},
  parsers::{matches, token},
  In,
};
use diom_syntax::expressions::Expression;
use diom_tokens::Token;
use nom::{branch::alt, error::context, Parser};
use nom_language::precedence::{binary_op, precedence, unary_op, Assoc, Operation};

mod compound;
use compound::parse_compound_value;
mod infix;
use infix::*;
mod postfix;
use postfix::*;
mod prefix;
use prefix::*;
mod literals;
use literals::parse_literal_value;
mod scopes;
use scopes::parse_scope_value;

/// When parsing expressions, we need to actually be somewhat careful
/// about the order that we parse different expression types in.
///
/// We need to parse in the rough order so that bracketing ends up "correct":
/// ```_
/// 3 > 4 - 2 + [() => 5][0]() * Some(3)? / 5 <= !False != True
/// ==
/// ((3 > ((4 - 2) + (((([() => 5])[0])()) * ((Some(3))?) / 5))) <= !False) != True
/// ```
/// <br>
///
/// First of all we need to parse expressions that are purely a prefix,<br>
/// i.e. expressions that are laid out as `<prefix> <expression>`:
/// 1. `(<parameters>) =>`
/// 1. `return`, `let <pattern> =`
///
/// As these expressions effectively "capture" the whole input.
///
/// ## Parsing order
///
/// 1. *"value-like"*s with unambiguous bounds
/// 1. `let` declerations
/// 1. `return` statements
/// 1. field accesses
/// 1. indexing
/// 1. explicit function calls
/// 1. *"method-like"* operators
/// 1. `*` and `/`
/// 1. `+` and `-`
/// 1. `&` and `|`
/// 1. `<`, `>`, `<=`, `>=`, `==` and `!=`
/// 1. `=` assignment
/// 1. implicit function calls
pub fn parse_expression<'a, E: SyntaxError<'a>>(
) -> impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E> {
  let parser = precedence(
    alt((
      unary_op(
        5,
        PartialPrefixOp::parse_with(token([Token::Plus, Token::Minus])).map(PartialPrefix::Op),
      ),
      unary_op(1, parse_let.map(PartialPrefix::Declare)),
      unary_op(1, parse_return.map(PartialPrefix::Return)),
    )),
    alt((
      unary_op(2, parse_field.map(PartialPostFix::Field)),
      unary_op(2, parse_index.map(PartialPostFix::Index)),
      unary_op(2, parse_explicit_call.map(PartialPostFix::Call)),
      // unary_op(9, parse_implicit_call.map(PartialPostFix::Call)),
    )),
    alt((
      binary_op(
        3,
        Assoc::Left,
        PartialInfix::parse_with(matches(Token::StringIdent("".into()))),
      ),
      binary_op(
        4,
        Assoc::Left,
        PartialInfix::parse_with(token([Token::Times, Token::Divide])),
      ),
      binary_op(
        5,
        Assoc::Left,
        PartialInfix::parse_with(token([Token::Plus, Token::Minus])),
      ),
      binary_op(
        6,
        Assoc::Left,
        PartialInfix::parse_with(token([Token::And, Token::Or])),
      ),
      binary_op(
        7,
        Assoc::Left,
        PartialInfix::parse_with(token([
          Token::Lt,
          Token::Gt,
          Token::LtEq,
          Token::GtEq,
          Token::Eq,
          Token::Ne,
        ])),
      ),
      binary_op(
        8,
        Assoc::Right,
        PartialInfix::parse_with(token(Token::Assign)),
      ),
    )),
    context(
      "value",
      alt((parse_scope_value, parse_literal_value, parse_compound_value)),
    ),
    apply_operation,
  );

  context("expression", parser)
}

fn apply_operation<'a>(
  op: Operation<
    PartialPrefix<In<'a>>,
    PartialPostFix<In<'a>>,
    PartialInfix<In<'a>>,
    Expression<In<'a>>,
  >,
) -> Result<Expression<In<'a>>, BuildError> {
  // SAFETY: `value` and `other` are produced from the same input slice
  Ok(match op {
    Operation::Prefix(pre, value) => unsafe { pre.apply(value) },
    Operation::Postfix(value, post) => unsafe { post.apply(value) },
    Operation::Binary(value, inf, other) => unsafe { Expression::Infix(inf.apply(value, other)) },
  })
}
