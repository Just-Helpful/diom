use super::utils::{quick_lex, quick_parse};
use insta::assert_debug_snapshot;

#[test]
fn floats() {
  let input = "5";
  let tokens = quick_lex(input);
  let expr = quick_parse(input, (&tokens).into());
  assert_debug_snapshot!(expr);

  let input = "5.0";
  let tokens = quick_lex(input);
  let expr = quick_parse(input, (&tokens).into());
  assert_debug_snapshot!(expr);

  let input = "5e0";
  let tokens = quick_lex(input);
  let expr = quick_parse(input, (&tokens).into());
  assert_debug_snapshot!(expr);

  let input = "5e-1";
  let tokens = quick_lex(input);
  let expr = quick_parse(input, (&tokens).into());
  assert_debug_snapshot!(expr);

  let input = "- 0.5e-2";
  let tokens = quick_lex(input);
  let expr = quick_parse(input, (&tokens).into());
  assert_debug_snapshot!(expr);
}

#[test]
fn functions() {
  let input = "() => 5";
  let tokens = quick_lex(input);
  let expr = quick_parse(input, (&tokens).into());
  assert_debug_snapshot!(expr);
}
