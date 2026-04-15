use diom_info_traits::InfoRef as _;
use diom_syntax::expressions::{Expression, Infix};
use diom_syntax::idents::Method;
use diom_syntax::Ptr;
use nom::combinator::recognize;
use nom::Parser;

use crate::errors::SyntaxError;
use crate::idents::parse_method;
use crate::utils::merge_spans;
use crate::{In, Item};

pub struct PartialMethod<I> {
  pub name: Method<I>,
}

impl<I> From<Method<I>> for PartialMethod<I> {
  fn from(value: Method<I>) -> Self {
    Self { name: value }
  }
}

impl<'a> PartialMethod<In<'a>> {
  pub fn parse_with<E: SyntaxError<'a>>(
    token_parser: impl Parser<In<'a>, Output = Item<'a>, Error = E>,
  ) -> impl Parser<In<'a>, Output = PartialMethod<In<'a>>, Error = E> {
    recognize(token_parser.map(|v| dbg!(v)))
      .and_then(parse_method)
      .map(PartialMethod::from)
  }
}

impl<'a> PartialMethod<In<'a>> {
  /// Applies this infix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>, other: Expression<In<'a>>) -> Infix<In<'a>> {
    let v_info = *value.info();
    let o_info = *other.info();
    Infix {
      value: Ptr::new(value),
      name: self.name,
      other: Ptr::new(other),
      info: unsafe { merge_spans(v_info, o_info) },
    }
  }
}
