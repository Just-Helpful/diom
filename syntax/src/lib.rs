/// # AST nodes for Diom
///
/// The nodes here fall into 2 broad categories:
/// 1. nodes for Diom typing
/// 2. nodes for Diom values
extern crate diom_syntax_macros;
use diom_syntax_macros::InfoSource;

pub mod expressions;
pub mod ident;
pub mod path;
pub mod patterns;
pub mod traits;
pub mod types;

pub use traits::{InfoSource, Span};
