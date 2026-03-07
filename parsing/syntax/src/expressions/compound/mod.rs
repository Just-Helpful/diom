use super::Expression;

pub mod arrays;
pub use arrays::Array;
pub mod functions;
pub use functions::{Parameter, Function, FunctionArm};
pub mod structs;
pub use structs::Struct;
