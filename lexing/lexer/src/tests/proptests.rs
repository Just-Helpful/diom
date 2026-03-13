use super::LexError;
use crate::parse_tokens;
use diom_syntax::expressions::Expression;
use nom::combinator::eof;
use proptest::prelude::*;

proptest! {
  /// Tests that we can lex code produced from a syntax tree
  #[test]
  fn lex_code(expr: Expression<()>) {
    let code = format!("{expr}");
    let (code, _) = parse_tokens(&code).expect("we can parse the tokens for the expression");
    eof::<_, LexError>(code).expect("There shouldn't be any input left to lex");
  }
}
