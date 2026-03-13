use std::fmt::from_fn;

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
    let code_ = code.clone();
    let (code_, _) = parse_tokens(&code_).expect("we can parse the tokens for the expression");
    eof::<_, LexError>(code_).map_err(|e| {
      from_fn(move |f| {
        let err = match &e {
          nom::Err::Error(e) => e,
          nom::Err::Failure(e) => e,
          nom::Err::Incomplete(_) => panic!(),
        };
        f.write_str("Expected no input left to lex\n")?;
        writeln!(f, "When lexing the string:\n'{code}'")?;
        writeln!(f, "But got:\n'{}'", err.input)
      })
    })
    .unwrap();
  }
}
