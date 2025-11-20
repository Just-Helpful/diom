use crate::{errors::PResult, parsers::token, Span};
use diom_syntax::patterns::ignored::Ignored;
use diom_tokens::{SpanTokens, Token};

pub fn parse_ignored(input: SpanTokens) -> PResult<Ignored<Span>> {
  let (input, underscore) = token(Token::StringIdent("_".into()))(input)?;
  Ok((
    input,
    Ignored {
      info: underscore.span,
    },
  ))
}
