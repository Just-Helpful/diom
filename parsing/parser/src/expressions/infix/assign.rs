use crate::{
  errors::{PResult, SyntaxError},
  parsers::matches,
  utils::merge_spans,
  In,
};
use diom_info_traits::InfoRef as _;
use diom_syntax::{
  expressions::{Assign, Expression},
  Ptr,
};
use diom_tokens::Token;
use nom::Parser;

pub struct PartialAssign;

impl PartialAssign {
  /// Applies this infix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply<'a>(
    self,
    value: Expression<In<'a>>,
    other: Expression<In<'a>>,
  ) -> Assign<In<'a>> {
    let v_info = *value.info();
    let o_info = *other.info();
    Assign {
      reference: Ptr::new(value),
      value: Ptr::new(other),
      info: unsafe { merge_spans(v_info, o_info) },
    }
  }
}

pub fn parse_assign<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, PartialAssign, E> {
  matches(Token::Assign).map(|_| PartialAssign).parse(input)
}
