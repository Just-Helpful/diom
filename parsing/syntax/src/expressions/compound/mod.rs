use super::Expression;

pub mod arrays;
pub use arrays::Array;
pub mod functions;
pub use functions::{Function, FunctionArm, Parameter, Parameters};
pub mod structs;
pub use structs::Struct;
