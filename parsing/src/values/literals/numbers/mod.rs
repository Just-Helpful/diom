use std::str::FromStr;

use super::Parsable;

pub trait ParsableNum: FromStr {
  const IDENT: &'static str;
}

#[macro_export]
macro_rules! impl_parsable_nums {
  ($($num_type:ty),*) => {$(
    impl ParsableNum for $num_type {
      const IDENT: &'static str = stringify!($num_type);
    }
  )*};
}

impl_parsable_nums!(i8, i16, i32, i64, i128);
impl_parsable_nums!(u8, u16, u32, u64, u128);
impl_parsable_nums!(f32, f64);

pub mod floating;
pub mod signed;
pub mod unsigned;
