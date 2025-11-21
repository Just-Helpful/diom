use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Char<I> {
  #[map_ignore]
  pub value: char,
  pub info: I,
}
