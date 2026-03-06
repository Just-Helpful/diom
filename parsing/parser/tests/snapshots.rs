use diom_lexer::parse_tokens;
use diom_parser::{parse_node, SyntaxNode};
use diom_tokens::SpanTokens;
use insta::assert_debug_snapshot;

type TestError<'a> = nom::error::Error<SpanTokens<'a>>;

#[test]
fn anonymous_function_creation() {
  let input = "() => 5";
  let (input, tokens) = parse_tokens(input).expect("we can parse the tokens for the expression");
  assert_eq!(input.len(), 0, "There shouldn't be any input left to lex");
  let tokens = SpanTokens(&tokens);

  let (tokens, node) =
    parse_node::<TestError>(tokens).expect("we can parse the syntax node for the expression");
  assert_eq!(
    tokens.len(),
    0,
    "There shouldn't be any tokens left to parse"
  );

  let SyntaxNode::Expression(expr) = node;
  assert_debug_snapshot!(expr);
}
