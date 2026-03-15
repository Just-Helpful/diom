use crate::{
  errors::{display_err, ExtensibleError},
  parse_node, SyntaxNode,
};
use diom_lexer::parse_tokens;
use diom_syntax::expressions::Expression;
use diom_tokens::{SpanToken, SpanTokens};
use nom::{combinator::all_consuming, Parser};

pub fn quick_lex(code: &str) -> Vec<SpanToken<'_>> {
  let (_, tokens) = all_consuming(parse_tokens())
    .parse(code)
    .map_err(|err| display_err(err, code))
    .unwrap();

  tokens
}

pub fn quick_parse<'a>(code: &'a str, tokens: SpanTokens<'a>) -> Expression<SpanTokens<'a>> {
  let (_, node) = all_consuming(parse_node)
    .parse(tokens)
    .map_err(|err| err.map(|err: ExtensibleError<_>| err.map_input(|input| input.tokens[0].origin)))
    .map_err(|err| display_err(err, code))
    .unwrap();
  let SyntaxNode::Expression(expr) = node;
  expr
}
