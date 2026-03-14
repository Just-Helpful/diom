use crate::{parse_node, SyntaxNode};
use diom_lexer::parse_tokens;
use diom_syntax::expressions::Expression;
use diom_tokens::{SpanToken, SpanTokens};
use nom::{combinator::all_consuming, error::Error, Err, Offset, Parser};
use std::fmt::{from_fn, Debug};

pub fn quick_lex(code: &str) -> Vec<SpanToken<'_>> {
  let (_, tokens) = all_consuming(parse_tokens())
    .parse(code)
    .map_err(|err| format_error(code, err))
    .unwrap();

  tokens
}

pub fn quick_parse<'a>(code: &'a str, tokens: SpanTokens<'a>) -> Expression<SpanTokens<'a>> {
  let (_, node) = all_consuming(parse_node)
    .parse(tokens)
    .map_err(|err| {
      err.map(|err: Error<_>| Error {
        input: err.input.0[0].origin,
        code: err.code,
      })
    })
    .map_err(|err| format_error(code, err))
    .unwrap();
  let SyntaxNode::Expression(expr) = node;
  expr
}

fn format_error<'a>(input_: &'a str, error: Err<Error<&'a str>>) -> impl Debug + 'a {
  from_fn(move |f| {
    writeln!(f, "\n`{input_}`")?;
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
