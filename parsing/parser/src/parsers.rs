use crate::common::{PResult, SpanToken, SpanTokens, Token};
use crate::Span;
use diom_info_traits::InfoRef;
use nom::{
  bytes::streaming::take_until,
  combinator::opt,
  error::{Error, ErrorKind},
  Slice,
};

pub fn token<'a>(tok: impl AsRef<Token> + 'a) -> impl Fn(SpanTokens) -> PResult<SpanToken> + 'a {
  move |input| match input.first() {
    Some(t) if t.matches(tok.as_ref()) => Ok((input.slice(1usize..), t.clone())),
    Some(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
    None => Err(nom::Err::Error(Error::new(input, ErrorKind::Eof))),
  }
}

pub fn group(
  lbrac: impl AsRef<Token>,
  rbrac: impl AsRef<Token>,
) -> impl Fn(SpanTokens) -> PResult<(SpanTokens, Span)> {
  move |input| {
    let (input, ltok) = token(&lbrac)(input)?;
    let (input, inner) = take_until(rbrac.as_ref())(input)?;
    let (input, rtok) = token(&rbrac)(input)?;
    Ok((input, (inner, ltok.span.start..rtok.span.end)))
  }
}

pub fn opt_tag_group<Tag: InfoRef<Info = Span>>(
  parse_tag: impl Fn(SpanTokens) -> PResult<Tag>,
  lbrac: impl AsRef<Token>,
  rbrac: impl AsRef<Token>,
) -> impl Fn(SpanTokens) -> PResult<(Option<Tag>, SpanTokens, Span)> {
  move |input| {
    let (input, tag) = opt(&parse_tag)(input)?;
    let (input, (inner, mut span)) = group(&lbrac, &rbrac)(input)?;
    if let Some(tag) = &tag {
      span.start = tag.info().start;
    };
    Ok((input, (tag, inner, span)))
  }
}
