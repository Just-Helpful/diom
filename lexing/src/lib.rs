pub mod lex;
pub use lex::parse_tokens;
pub mod tokens;
pub use tokens::{SpanToken, SpanTokens, Token};

#[cfg(test)]
pub mod test_helpers {
  pub fn arr_into<const N: usize, T, R: From<T>>(arr: [T; N]) -> [R; N] {
    arr.map(R::from)
  }
}
