use crate::{errors::SyntaxError, ident::parse_ident, utils::merge_spans, In, Item};
use diom_info_traits::InfoRef;
use diom_syntax::{
  expressions::{Call, Expression},
  ident::Ident,
};
use nom::{combinator::recognize, Parser};

pub struct PartialPrefixOp<I> {
  pub name: Ident<I>,
}

impl<I> From<Ident<I>> for PartialPrefixOp<I> {
  fn from(value: Ident<I>) -> Self {
    Self { name: value }
  }
}

impl<'a> PartialPrefixOp<In<'a>> {
  /// Applies this prefix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Call<In<'a>> {
    let info = *value.info();
    let info = /*unsafe*/ { merge_spans(self.name.info, info) };
    Call {
      value: Box::new(Expression::Var(self.name)),
      args: vec![value],
      info,
    }
  }
}

impl<'a> PartialPrefixOp<In<'a>> {
  pub fn parse_with<E: SyntaxError<'a>>(
    token_parser: impl Parser<In<'a>, Output = Item<'a>, Error = E>,
  ) -> impl Parser<In<'a>, Output = PartialPrefixOp<In<'a>>, Error = E> {
    recognize(token_parser)
      .and_then(parse_ident)
      .map(PartialPrefixOp::from)
  }
}
