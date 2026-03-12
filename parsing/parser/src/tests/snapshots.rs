use super::utils::{quick_lex, quick_parse};
use insta::assert_debug_snapshot;

#[test]
fn floats() {
  let tokens = quick_lex("5");
  let expr = quick_parse((&tokens).into());
  assert_debug_snapshot!(expr);

  let tokens = quick_lex("5.0");
  let expr = quick_parse((&tokens).into());
  assert_debug_snapshot!(expr);

  let tokens = quick_lex("5e0");
  let expr = quick_parse((&tokens).into());
  assert_debug_snapshot!(expr);

  let tokens = quick_lex("5e-1");
  let expr = quick_parse((&tokens).into());
  assert_debug_snapshot!(expr);

  let tokens = quick_lex("- 0.5e-2");
  let expr = quick_parse((&tokens).into());
  assert_debug_snapshot!(expr);
}

#[test]
fn functions() {
  let tokens = quick_lex("() => 5");
  let expr = quick_parse((&tokens).into());
  assert_debug_snapshot!(expr);
}
