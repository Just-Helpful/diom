use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Float<I> {
  #[map_ignore]
  pub value: f64,
  pub info: I,
}
