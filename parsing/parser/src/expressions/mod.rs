//! Expression parsers
//!
//! All expression parsers assume their entire input is an expression.
//! This means that if there is any remaining input after each parse,
//! they will fail.
use crate::errors::PResult;
use crate::errors::SyntaxError;
use crate::In;
use crate::{
  ident::parse_ident,
  parsers::{group, matches},
};
use diom_info_traits::InfoRef;
use diom_syntax::expressions::{Call, Expression, Field, Index, Infix};
use diom_tokens::{SpanTokens, Token};
use nom::combinator::consumed;
use nom::sequence::{preceded, terminated};
use nom::{branch::alt, combinator::eof, error::context, multi::separated_list0, Parser};

mod compound;
use compound::parse_compound_value;
mod literals;
use literals::parse_literal_value;
mod scopes;
use scopes::parse_scope_value;

/// Values that have clear start + end delimiters
pub fn parse_value<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Expression<In<'a>>, E> {
  context(
    "value",
    alt((parse_scope_value, parse_literal_value, parse_compound_value)),
  )
  .parse(input)
}

/// Merges two slices into one
///
/// # Safety
///
/// Boths `a` and `b` must come from the same original slice
const unsafe fn merge_slices<'a, T>(a: &'a [T], b: &'a [T]) -> &'a [T] {
  let start = a.as_ptr();
  let end = b.as_ptr().add(b.len());
  let len = end.offset_from(start) as usize;
  std::slice::from_raw_parts(start, len)
}

/// Merges two spans into one
///
/// # Safety
///
/// Boths `a` and `b` must come from the same original input span
unsafe fn merge_spans<'a>(a: SpanTokens<'a>, b: SpanTokens<'a>) -> SpanTokens<'a> {
  let merged = merge_slices(a.0, b.0);
  SpanTokens::from(merged)
}

/// Constructs an infix parser from a higher precedence expression parser
fn infix_parser<'a, E: SyntaxError<'a>>(
  mut parse_expr: impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E>,
  op_tokens: impl AsRef<[Token]>,
) -> impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E> {
  context("infix", move |input: In<'a>| {
    // parse an initial expression
    let (mut input, mut value) = parse_expr.parse(input.clone())?;

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
      let (input_, other) = parse_expr.parse(input_)?;

      // Safety: `value` and `other`'s info is constructed from `input`
      let info = unsafe { merge_spans(*value.info(), *other.info()) };

      value = Expression::Infix(Infix {
        value: Box::new(value.clone()),
        name: ident,
        other: Box::new(other.clone()),
        info,
      });
      input = input_
    }

    Ok((input, value))
  })
}

/// Constructs an implicit call parser from a higher precedence expression parser\
/// Implicit calls don't require brackets and therefore should have a lower precedence
/// Than "method-like" operators (as otherwise it would consume the ident)
fn implicit_call_parser<'a, E: SyntaxError<'a>>(
  mut parse_expr: impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E>,
) -> impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E> {
  context("call", move |input| {
    let (mut input, mut value) = parse_expr.parse(input)?;

    while let Ok((input_, arg)) = parse_value::<E>(input) {
      // Safety: `value` and `arg`'s info is constructed from `input`
      let info = unsafe { merge_spans(*value.info(), *arg.info()) };

      value = Expression::Call(Call {
        value: Box::new(value),
        args: vec![arg],
        info,
      });
      input = input_
    }

    Ok((input, value))
  })
}

fn explicit_call_parser<'a, E: SyntaxError<'a>>(
  mut parse_expr: impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E>,
) -> impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E> {
  let parse_inner = terminated(
    separated_list0(matches(Token::Comma), parse_expression),
    eof,
  );
  let parser = group::<E>(Token::LParen, Token::RParen).and_then(parse_inner);
  let mut parser = consumed(parser);

  context("call", move |input| {
    let (mut input, mut value) = parse_expr.parse(input)?;
    while let Ok((input_, (info, args))) = parser.parse(input) {
      // Safety: `value` and `args`'s info is constructed from `input`
      let info = unsafe { merge_spans(*value.info(), info) };

      value = Expression::Call(Call {
        value: Box::new(value),
        args,
        info,
      });
      input = input_
    }

    Ok((input, value))
  })
}

fn index_parser<'a, E: SyntaxError<'a>>(
  mut parse_expr: impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E>,
) -> impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E> {
  let parse_inner = terminated(
    separated_list0(matches(Token::Comma), parse_expression::<E>),
    eof,
  );
  let parser = group::<E>(Token::LBrace, Token::RBrace).and_then(parse_inner);
  let mut parser = consumed(parser);

  context("index", move |input| {
    let (mut input, mut value) = parse_expr.parse(input)?;
    while let Ok((input_, (info, key))) = parser.parse(input) {
      // Safety: `value` and `key`'s info is constructed from `input`
      let info = unsafe { merge_spans(*value.info(), info) };

      value = Expression::Index(Index {
        value: Box::new(value),
        key,
        info,
      });
      input = input_
    }

    Ok((input, value))
  })
}

fn field_parser<'a, E: SyntaxError<'a>>(
  mut parse_expr: impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E>,
) -> impl Parser<In<'a>, Output = Expression<In<'a>>, Error = E> {
  let parser = preceded(matches::<E>(Token::Dot), parse_ident);
  let mut consumed = consumed(parser);

  context("field", move |input| {
    let (mut input, mut value) = parse_expr.parse(input)?;
    while let Ok((input_, (info, name))) = consumed.parse(input) {
      // Safety: `value` and `name`'s info is constructed from `input`
      let info = unsafe { merge_spans(*value.info(), info) };

      value = Expression::Field(Field {
        value: Box::new(value),
        name,
        info,
      });
      input = input_
    }

    Ok((input, value))
  })
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
pub fn parse_expression<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, Expression<In<'a>>, E> {
  // parse explicit function calls first
  let parse_expr = explicit_call_parser(parse_value);

  let parse_expr = field_parser(parse_expr);
  let parse_expr = index_parser(parse_expr);

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
  let parse_expr = implicit_call_parser(parse_expr);

  context("expression", parse_expr).parse(input)
}
