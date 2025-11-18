//! Infix operators
//!
//! ## Warning
//!
//! These are only used during parsing!<br>
//! They will be translated into field calls.
use super::Expression;
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Infix<I> {
    pub value: Box<Expression<I>>,
    pub name: Ident<I>,
    pub other: Box<Expression<I>>,
    pub info: I,
}
