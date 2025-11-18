pub mod arrays;
use arrays::Array;
pub mod ignored;
use ignored::Ignored;
pub mod rest;
use rest::Rest;
pub mod structs;
use structs::Struct;
pub mod tuples;
use tuples::Tuple;

use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub enum Pattern<I> {
    Array(Array<I>),
    Struct(Struct<I>),
    Tuple(Tuple<I>),
    Ignored(Ignored<I>),
    Var(Ident<I>),
}
