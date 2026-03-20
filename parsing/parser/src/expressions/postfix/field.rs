use crate::{
  common::PResult, errors::SyntaxError, idents::parse_method, parsers::token, utils::merge_spans,
  In,
};
use diom_info_traits::InfoRef as _;
use diom_syntax::{
  expressions::{Expression, Field},
  idents::Method,
};
use diom_tokens::Token;
use nom::{combinator::consumed, error::context, sequence::preceded, Parser};

pub struct PostFixField<I> {
  pub(crate) name: Method<I>,
  pub(crate) info: I,
}

impl<'a> PostFixField<In<'a>> {
  /// Applies this postfix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Field<In<'a>> {
    let info = unsafe { merge_spans(*value.info(), self.info) };
    Field {
      value: Box::new(value),
      name: self.name,
      info,
    }
  }
}

pub fn parse_field<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, PostFixField<In<'a>>, E> {
  let parser = preceded(token(Token::Dot), parse_method);
  let parser = consumed(parser).map(|(info, name)| PostFixField { name, info });
  context("field access", parser).parse(input)
}
