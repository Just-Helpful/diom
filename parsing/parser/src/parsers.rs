use std::num::NonZero;

use crate::common::{PResult, SpanToken, SpanTokens, Token};
use crate::Span;
use diom_info_traits::InfoRef;
use nom::InputTake;
use nom::{
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
    let mut scope = 1usize;

    // bracket counting and scope detection
    let idx = input.into_iter().position(|tok| {
      if tok.matches(lbrac.as_ref()) {
        scope += 1;
        return false;
      }
      if tok.matches(rbrac.as_ref()) {
        scope -= 1;
      }
      scope == 0
    });

    // if we are in scope `scope` by the end of the input
    // then we need at least `scope` more closing brackets...
    let Some(i) = idx else {
      return Err(nom::Err::Incomplete(nom::Needed::Size(
        NonZero::new(scope).unwrap(),
      )));
    };

    let (input, inner) = input.take_split(i);
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
