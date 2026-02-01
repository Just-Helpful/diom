use crate::{errors::SyntaxError, expressions::merge_spans, parsers::matches, In};
use diom_info_traits::InfoRef;
use diom_syntax::expressions::{Expression, Return};
use diom_tokens::Token;
use nom::{combinator::recognize, IResult, Parser};

pub struct PartialReturn<I> {
  info: I,
}

impl<'a> PartialReturn<In<'a>> {
  /// Applies this prefix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Return<In<'a>> {
    let info = *value.info();
    Return {
      value: Box::new(value),
      info: unsafe { merge_spans(self.info, info) },
    }
  }
}

pub fn parse_return<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> IResult<In<'a>, PartialReturn<In<'a>>, E> {
  recognize(matches(Token::Return))
    .map(|info| PartialReturn { info })
    .parse(input)
}
