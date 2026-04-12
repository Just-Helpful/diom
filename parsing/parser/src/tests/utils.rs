use crate::{parse_node, SyntaxError, SyntaxNode};
use diom_lexer::{errors::TokensError, parse_tokens};
use diom_syntax::expressions::Expression;
use diom_tokens::{SpanToken, SpanTokens};
use nom::{combinator::all_consuming, Err, Parser};
use nom_yuck::{on, Annotated, DisplayAs, On};

pub fn quick_lex<'a, E>(code: &'a str) -> Vec<SpanToken<'a>>
where
  E: TokensError<'a>,
  Err<E>: DisplayAs<On<&'a str>>,
{
  let (_, tokens) = all_consuming(parse_tokens::<E>())
    .parse(code)
    .map_err(|err| err.display_with(on(code)))
    .unwrap();

  tokens
}

pub fn quick_parse<'a: 'b, 'b, E>(
  code: &'a str,
  tokens: SpanTokens<'b>,
) -> Expression<SpanTokens<'b>>
where
  E: SyntaxError<'b>,
  Err<E>: DisplayAs<Annotated<&'b str>>,
{
  let (_, node) = all_consuming(parse_node)
    .parse(tokens)
    .map_err(|err| err.display_with(on(code)))
    .unwrap();
  let SyntaxNode::Expression(expr) = node;
  expr
}

#[macro_export]
/// Asserts that the given code can be parsed by the compiler
macro_rules! assert_parses {
  ($code:expr) => {{
    use crate::tests::utils::{quick_lex, quick_parse};
    use diom_tokens::SpanTokens;
    let tokens = quick_lex(&$code);
    let tokens = SpanTokens::new(&tokens, &$code);
    quick_parse(&$code, tokens);
  }};
}
