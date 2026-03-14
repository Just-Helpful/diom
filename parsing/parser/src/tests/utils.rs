use crate::{parse_node, SyntaxNode};
use diom_lexer::parse_tokens;
use diom_syntax::expressions::Expression;
use diom_tokens::{SpanToken, SpanTokens};
use nom::{combinator::all_consuming, Err, Offset, Parser};
use nom_language::error::VerboseError;
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
      err.map(|err: VerboseError<_>| VerboseError {
        errors: err
          .errors
          .into_iter()
          .map(|(input, kind)| (input.0[0].origin, kind))
          .collect(),
      })
    })
    .map_err(|err| format_error(code, err))
    .unwrap();
  let SyntaxNode::Expression(expr) = node;
  expr
}

fn format_error<'a>(input_: &'a str, error: Err<VerboseError<&'a str>>) -> impl Debug + 'a {
  from_fn(move |f| {
    write!(f, "\n`{input_}`")?;
    if let Err::Incomplete(n) = error {
      write!(f, "\n{}", " ".repeat(input_.len() + 1))?;
      write!(f, "^ Needed ({n:?})")?;
      return Ok(());
    }

    let (kind, VerboseError { errors }) = match &error {
      Err::Error(e) => ("error", e),
      Err::Failure(e) => ("failure", e),
      _ => unreachable!("we've handled incomplete"),
    };
    write!(f, " had {}", kind)?;

    for (input, kind) in errors {
      let i = input_.offset(input);
      write!(f, "\n{}", " ".repeat(i + 1))?;
      write!(f, "^ {kind:?}")?;
    }
    Ok(())
  })
}
