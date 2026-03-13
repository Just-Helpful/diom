use std::fmt::from_fn;

use crate::{parse_node, SyntaxNode};
use diom_lexer::parse_tokens;
use diom_syntax::expressions::Expression;
use diom_tokens::{SpanToken, SpanTokens};
use nom::combinator::eof;

type LexError<'a> = nom::error::Error<&'a str>;
type ParseError<'a> = nom::error::Error<SpanTokens<'a>>;

pub fn quick_lex(input: &str) -> Vec<SpanToken<'_>> {
  let (input, tokens) = parse_tokens(input).expect("we can parse the tokens for the expression");
  eof::<_, LexError>(input)
    .map_err(|e| {
      from_fn(move |f| {
        let err = match &e {
          nom::Err::Error(e) => e,
          nom::Err::Failure(e) => e,
          nom::Err::Incomplete(_) => panic!(),
        };
        f.write_str("Expected no input left to lex\n")?;
        writeln!(f, "When lexing the string:\n'{input}'")?;
        writeln!(f, "But got:\n'{}'", err.input)
      })
    })
    .unwrap();
  tokens
}

pub fn quick_parse(tokens: SpanTokens) -> Expression<SpanTokens> {
  let (tokens, node) =
    parse_node::<ParseError>(tokens).expect("we can parse the syntax node for the expression");
  eof::<_, ParseError>(tokens)
    .map_err(|e| {
      from_fn(move |f| {
        let err = match &e {
          nom::Err::Error(e) => e,
          nom::Err::Failure(e) => e,
          nom::Err::Incomplete(_) => panic!(),
        };
        f.write_str("Expected no input left to parse\n")?;
        writeln!(f, "When parsing the tokens:\n'{tokens}'")?;
        writeln!(f, "But got:\n'{}'", err.input)
      })
    })
    .unwrap();

  let SyntaxNode::Expression(expr) = node;
  expr
}
