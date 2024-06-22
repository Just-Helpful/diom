use crate::ident::Ident;

use super::Pattern;

pub struct Enum<I> {
  pub path: Vec<Ident<I>>,
  pub variant: Box<Pattern<I>>,
}
