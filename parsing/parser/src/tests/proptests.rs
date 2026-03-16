use super::utils::{quick_lex, quick_parse};
use diom_info_traits::InfoMap;
use diom_syntax::expressions::Expression;
use diom_tokens::SpanTokens;
use proptest::prelude::*;

proptest! {
  /// Tests that we can format an Expression to code and then parse it back to an expression
  #[test]
  fn parser_roundtrip(expr: Expression<()>) {
    let code = format!("{expr}");
    let tokens = quick_lex(&code);

    let expr_ = quick_parse(&code, SpanTokens::new(&tokens, &code));
    let _expr = expr_.map(|_| ());
    // @todo implement `PartialEq` and test expression equality
  }
}
