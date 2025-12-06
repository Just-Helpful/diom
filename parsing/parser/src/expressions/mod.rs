//! Expression parsers
//!
//! All expression parsers assume their entire input is an expression.
//! This means that if there is any remaining input after each parse,
//! they will fail.
use crate::{
  ident::parse_ident,
  parsers::{group, token},
};
use diom_info_traits::InfoRef;
use diom_syntax::expressions::{Call, Expression, Infix};
use diom_tokens::{SpanTokens, Token};

mod compound;
mod control;
mod literals;
use compound::parse_compound_value;
use literals::parse_literal_value;
use nom::{branch::alt, combinator::eof, multi::separated_list0};

use crate::{common::Span, errors::PResult};

/// Values that have clear start + end delimiters
pub fn parse_value(input: SpanTokens) -> PResult<Expression<Span>> {
  alt((parse_literal_value, parse_compound_value))(input)
}

/// Constructs an infix parser from a higher precedence expression parser
fn infix_parser<'a>(
  mut parse_expr: impl FnMut(SpanTokens<'a>) -> PResult<'a, Expression<Span>>,
  op_tokens: impl AsRef<[Token]>,
) -> impl FnMut(SpanTokens<'a>) -> PResult<'a, Expression<Span>> {
  #[inline]
  move |input| {
    // parse an initial expression
    let (mut input, mut value) = parse_expr(input)?;

    // and then wrap it from left to right
    // this means we'll always bracket expressions as so:
    // `(((((x + x) + x) + x) + x) + x)`
    // @todo maybe support right to left binding?
    while input.get(0).is_some_and(|tok| {
      op_tokens
        .as_ref()
        .iter()
        .any(|pat| pat.matches(tok.as_ref()))
    }) {
      let (input_, ident) = parse_ident(input)?;
      let (input_, other) = parse_expr(input_)?;

      value = Expression::Infix(Infix {
        info: value.info().start..other.info().end,
        value: Box::new(value),
        name: ident,
        other: Box::new(other),
      });
      input = input_
    }

    Ok((input, value))
  }
}

/// Constructs an implicit call parser from a higher precedence expression parser\
/// Implicit calls don't require brackets and therefore should have a lower precedence
/// Than "method-like" operators (as otherwise it would consume the ident)
fn implicit_call_parser<'a>(
  mut parse_expr: impl FnMut(SpanTokens<'a>) -> PResult<'a, Expression<Span>>,
) -> impl FnMut(SpanTokens<'a>) -> PResult<Expression<Span>> {
  #[inline]
  move |input| {
    let (mut input, mut value) = parse_expr(input)?;

    while let Ok((input_, arg)) = parse_value(input) {
      value = Expression::Call(Call {
        info: value.info().start..arg.info().end,
        value: Box::new(value),
        args: vec![arg],
      });
      input = input_
    }

    Ok((input, value))
  }
}

fn explicit_call_parser<'a>(
  mut parse_expr: impl FnMut(SpanTokens<'a>) -> PResult<'a, Expression<Span>>,
) -> impl FnMut(SpanTokens<'a>) -> PResult<Expression<Span>> {
  move |input| {
    let (mut input, mut value) = parse_expr(input)?;

    while let Ok((input_, (inner, span))) = group(Token::LParen, Token::RParen)(input) {
      let (inner, args) = separated_list0(token(Token::Comma), parse_expression)(inner)?;
      eof(inner)?;

      value = Expression::Call(Call {
        info: value.info().start..span.end,
        value: Box::new(value),
        args,
      });
      input = input_
    }

    Ok((input, value))
  }
}

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
///
/// Parses infix operators i.e. `<value> <op> <other>`\
/// with the following precedences:
/// 1. *"method-like"* operators
/// 1. `*` and `/`
/// 1. `+` and `-`
/// 1. `&` and `|`
/// 1. `<`, `>`, `<=`, `>=`, `==` and `!=`
pub fn parse_expression(input: SpanTokens) -> PResult<Expression<Span>> {
  // parse explicit function calls first
  let parse_expr = explicit_call_parser(parse_value);

  let parse_expr = infix_parser(
    parse_expr,
    [Token::StringIdent("".into())], // **any** ident
  );

  let parse_expr = infix_parser(
    parse_expr,
    [Token::Times, Token::Divide], // `*`, `/`
  );

  let parse_expr = infix_parser(
    parse_expr,
    [Token::Plus, Token::Minus], // `+`, `-`
  );

  let parse_expr = infix_parser(
    parse_expr,
    [Token::And, Token::Or], // `&`, `|`
  );

  let parse_expr = infix_parser(
    parse_expr,
    [
      Token::Lt,
      Token::Gt,
      Token::LtEq,
      Token::GtEq,
      Token::Eq,
      Token::Ne,
    ], // `<`, `>`, `<=`, `>=`, `==`, `!=`
  );

  // parse implicit calls after "method-like"s
  let mut parse_expr = implicit_call_parser(parse_expr);

  parse_expr(input)
}
