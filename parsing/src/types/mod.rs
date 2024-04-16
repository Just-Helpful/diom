//! # Diom type parsing
//!
//! Types in Diom are defined via 'uninitialised variable' syntax,
//! i.e. `let uint32: u32` defines `uint32` as a subtype of `u32`.
//!
//! I'm planning to support:
//!
//! 1. Named/Unnamed tuples
//! 2. Named/Unnamed structs
//! 3. Named/Unnamed enums
//! 4. Named/Unnamed functions
//! 5. Unique types
//! 6. Subtypes
//!
//! See the individual modules for more details
