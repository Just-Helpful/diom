use crate::{errors::SyntaxError, idents::parse_op, utils::merge_spans, In, Item};
use diom_info_traits::InfoRef;
use diom_syntax::{
  expressions::{Expression, Prefix},
  idents::Op,
};
use nom::{combinator::recognize, Parser};

pub struct PartialPrefixOp<I> {
  pub name: Op<I>,
}

impl<I> From<Op<I>> for PartialPrefixOp<I> {
  fn from(value: Op<I>) -> Self {
    Self { name: value }
  }
}

impl<'a> PartialPrefixOp<In<'a>> {
  /// Applies this prefix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Prefix<In<'a>> {
    let info = *value.info();
    let info = /*unsafe*/ { merge_spans(self.name.info, info) };
    Prefix {
      name: self.name,
      value: Box::new(value),
      info,
    }
  }
}

impl<'a> PartialPrefixOp<In<'a>> {
  pub fn parse_with<E: SyntaxError<'a>>(
    token_parser: impl Parser<In<'a>, Output = Item<'a>, Error = E>,
  ) -> impl Parser<In<'a>, Output = PartialPrefixOp<In<'a>>, Error = E> {
    recognize(token_parser)
      .and_then(parse_op)
      .map(PartialPrefixOp::from)
  }
}
