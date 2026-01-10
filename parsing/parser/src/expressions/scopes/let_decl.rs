use crate::{
  errors::{PResult, SyntaxError},
  expressions::parse_expression,
  parsers::matches,
  patterns::parse_pattern,
  types::parse_type,
  In,
};
use diom_syntax::expressions::Declare;
use diom_tokens::Token;
use nom::{
  combinator::{consumed, opt},
  sequence::{preceded, separated_pair},
  Parser,
};

/// Parses a `let` expression, i.e.
///
/// ```_
/// let x = 2;
/// let y: Bool = True;
/// let Some(z) = if(x > 3)(2);
/// ```
///
/// ## Type of `let`
///
/// The type of a `let` expression is `Option<{...scope}>`.<br>
/// This allows us to support both of the following syntaxes:
///
/// ```_
/// (
///   if (let Some(x) = xs.position(0 <)) (
///     return x + 1;
///   );
///   xs.length
/// )
///
/// (
///   (let Some(x) = xs.position(0 <)) else (
///     return xs.length;
///   );
///   x + 1
/// )
/// ```
pub fn parse_let<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Declare<In<'a>>, E> {
  let parse_annotation = preceded(matches(Token::Colon), parse_type);
  let parser = preceded(
    matches(Token::Let),
    separated_pair(
      parse_pattern.and(opt(parse_annotation)),
      matches(Token::Assign),
      parse_expression,
    ),
  );

  let (input, (info, ((pattern, annotation), value))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Declare {
      info,
      pattern,
      annotation,
      value: Box::new(value),
    },
  ))
}
