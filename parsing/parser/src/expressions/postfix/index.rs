use crate::{
  common::PResult,
  errors::SyntaxError,
  expressions::parse_expression,
  parsers::{token, token_separated_list},
  utils::merge_spans,
  In,
};
use diom_info_traits::InfoRef as _;
use diom_syntax::expressions::{Expression, Index};
use diom_tokens::Token;
use nom::{combinator::consumed, error::context, sequence::delimited, Parser};

pub struct PostFixIndex<I> {
  pub(crate) keys: Vec<Expression<I>>,
  pub(crate) info: I,
}

impl<'a> PostFixIndex<In<'a>> {
  /// Applies this postfix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Index<In<'a>> {
    let info = unsafe { merge_spans(*value.info(), self.info) };
    Index {
      value: Box::new(value),
      keys: self.keys,
      info,
    }
  }
}

pub fn parse_index<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, PostFixIndex<In<'a>>, E> {
  let parser = token_separated_list(Token::Comma, parse_expression());
  let parser = delimited(token(Token::LBrace), parser, token(Token::RBrace));
  let parser = consumed(parser).map(|(info, keys)| PostFixIndex { keys, info });
  context("index", parser).parse(input)
}
