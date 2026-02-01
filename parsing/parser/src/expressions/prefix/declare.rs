use crate::{
  errors::SyntaxError, parsers::matches, patterns::parse_pattern, types::parse_type,
  utils::merge_spans, In,
};
use diom_info_traits::InfoRef;
use diom_syntax::{
  expressions::{Declare, Expression},
  patterns::Pattern,
  types::Type,
};
use diom_tokens::Token;
use nom::{
  combinator::{consumed, cut, opt},
  sequence::{preceded, terminated},
  IResult, Parser,
};

pub struct PartialDeclare<I> {
  pattern: Pattern<I>,
  annotation: Option<Type<I>>,
  info: I,
}

impl<'a> PartialDeclare<In<'a>> {
  /// Applies this prefix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Declare<In<'a>> {
    let info = *value.info();
    Declare {
      pattern: self.pattern,
      annotation: self.annotation,
      value: Box::new(value),
      info: unsafe { merge_spans(self.info, info) },
    }
  }
}

pub fn parse_let<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> IResult<In<'a>, PartialDeclare<In<'a>>, E> {
  let parse_annot = preceded(matches(Token::Colon), parse_type);
  let parse_rest = terminated(parse_pattern.and(opt(parse_annot)), matches(Token::Assign));
  let parser = consumed(preceded(matches(Token::Let), cut(parse_rest)));

  parser
    .map(|(info, (pattern, annotation))| PartialDeclare {
      pattern,
      annotation,
      info,
    })
    .parse(input)
}
