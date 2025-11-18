use crate::{
  errors::PResult, expressions::parse_expression, parsers::token, patterns::parse_pattern,
  types::parse_type, Span,
};
use diom_syntax::{expressions::Declare, InfoSource};
use diom_tokens::{SpanTokens, Token};
use nom::{combinator::opt, sequence::preceded};

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
/// The type of a `let` expression is `Option<Never>`.<br>
/// This allows us to support both of the following syntaxes:
///
/// ```_
/// {
///   if (let Some(x) = xs.position(0 <)) {
///     return x + 1;
///   }
///   xs.length
/// }
///
/// {
///   (let Some(x) = xs.position(0 <)) else {
///     return xs.length;
///   }
///   x + 1
/// }
/// ```
pub fn parse_let(input: SpanTokens) -> PResult<Declare<Span>> {
  let (input, tok) = token(Token::Let)(input)?;
  let (input, pattern) = parse_pattern(input)?;
  let (input, annotation) = opt(preceded(token(Token::Colon), parse_type))(input)?;
  let (input, _) = token(Token::Assign)(input)?;
  let (input, value) = parse_expression(input)?;
  Ok((
    input,
    Declare {
      info: tok.span.start..value.info().end,
      pattern,
      annotation,
      value: Box::new(value),
    },
  ))
}
