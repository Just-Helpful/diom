use super::utils::{quick_lex, quick_parse};
use diom_info_traits::InfoMap;
use diom_syntax::expressions::Expression;
use proptest::prelude::*;

proptest! {
  #[test]
  fn parser_roundtrip(expr: Expression<()>) {
    let code = format!("{expr}");
    let tokens = quick_lex(&code);

    let expr_ = quick_parse((&tokens).into());
    let expr_ = expr_.map(|_| ());
    // @todo implement `PartialEq` and test expression equality
  }
}
