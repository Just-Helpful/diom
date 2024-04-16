//! # Diom Literals
//!
//! Diom uses several types of literal values:
//! 1. number-like values:
//!   a. integers: `127i8`, `2047i16`, ...
//!   b. unsigned integers: `255u8`, `4095u16`, ...
//!   c. floating point numbers: `0.125f16`
//! 2. string like values:
//!   a. individual characters
//!   b. multicharacter strings
//! 3. boolean values
//!   a. these are effectively implemented as an enum
use super::Parsable;

pub mod numbers;
