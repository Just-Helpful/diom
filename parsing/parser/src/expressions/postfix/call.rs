use crate::{
  common::PResult,
  errors::SyntaxError,
  expressions::parse_expression,
  parsers::{token, token_separated_list},
  utils::merge_spans,
  In,
};
use diom_info_traits::InfoRef as _;
use diom_syntax::expressions::{Call, Expression};
use diom_tokens::Token;
use nom::{combinator::consumed, error::context, sequence::delimited, Parser};

pub struct PostFixCall<I> {
  pub(crate) args: Vec<Expression<I>>,
  pub(crate) info: I,
}

impl<'a> PostFixCall<In<'a>> {
  /// Applies this postfix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Call<In<'a>> {
    let info = unsafe { merge_spans(*value.info(), self.info) };
    Call {
      value: Box::new(value),
      args: self.args,
      info,
    }
  }
}

pub fn parse_implicit_call<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, PostFixCall<In<'a>>, E> {
  let parser = consumed(parse_expression()).map(|(info, arg)| PostFixCall {
    args: vec![arg],
    info,
  });
  context("implicit call", parser).parse(input)
}

pub fn parse_explicit_call<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, PostFixCall<In<'a>>, E> {
  let parser = token_separated_list(Token::Comma, parse_expression());
  let parser = delimited(token(Token::LParen), parser, token(Token::RParen));
  let parser = consumed(parser).map(|(info, args)| PostFixCall { args, info });
  context("explicit call", parser).parse(input)
}
