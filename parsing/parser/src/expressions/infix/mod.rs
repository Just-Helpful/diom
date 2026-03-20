use diom_info_traits::InfoRef;
use diom_syntax::{
  expressions::{Expression, Infix},
  idents::Method,
};
use nom::{combinator::recognize, Parser};

use crate::{errors::SyntaxError, idents::parse_method, utils::merge_spans, In, Item};

pub struct PartialInfix<I> {
  pub name: Method<I>,
}

impl<I> From<Method<I>> for PartialInfix<I> {
  fn from(value: Method<I>) -> Self {
    Self { name: value }
  }
}

impl<'a> PartialInfix<In<'a>> {
  pub fn parse_with<E: SyntaxError<'a>>(
    token_parser: impl Parser<In<'a>, Output = Item<'a>, Error = E>,
  ) -> impl Parser<In<'a>, Output = PartialInfix<In<'a>>, Error = E> {
    recognize(token_parser.map(|v| dbg!(v)))
      .and_then(parse_method)
      .map(PartialInfix::from)
  }
}

impl<'a> PartialInfix<In<'a>> {
  /// Applies this infix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>, other: Expression<In<'a>>) -> Infix<In<'a>> {
    let v_info = *value.info();
    let o_info = *other.info();
    Infix {
      value: Box::new(value),
      name: self.name,
      other: Box::new(other),
      info: unsafe { merge_spans(v_info, o_info) },
    }
  }
}
