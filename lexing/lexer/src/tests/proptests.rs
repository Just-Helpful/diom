use crate::parse_tokens;
use diom_syntax::expressions::Expression;
use diom_tokens::SpanToken;
use nom::{combinator::all_consuming, error::Error, Err, Offset, Parser};
use proptest::prelude::*;
use std::fmt::{from_fn, Debug};

proptest! {
  /// Tests that we can lex code produced from a syntax tree
  #[test]
  fn lex_code(expr: Expression<()>) {
    let code = format!("{expr}");
    quick_lex(&code);
  }
}

fn quick_lex(code: &str) -> Vec<SpanToken<'_>> {
  let (_, tokens) = all_consuming(parse_tokens())
    .parse(code)
    .map_err(|err| format_error(code, err))
    .unwrap();

  tokens
}

fn format_error<'a>(input_: &'a str, error: Err<Error<&'a str>>) -> impl Debug + 'a {
  from_fn(move |f| {
    writeln!(f, "`{input_}`")?;
    if let Err::Incomplete(n) = error {
      f.write_str(&" ".repeat(input_.len() + 1))?;
      write!(f, "^ Needed ({n:?})")?;
      return Ok(());
    }

    let (kind, Error { input, code }) = match &error {
      Err::Error(e) => ("error", e),
      Err::Failure(e) => ("failure", e),
      _ => unreachable!("we've handled incomplete"),
    };

    let i = input_.offset(input);
    f.write_str(&" ".repeat(i + 1))?;
    write!(f, "^ {code:?} ({kind})")
  })
}
