pub mod composed;
pub use composed::ComposedType;
pub mod literal;
pub use literal::LiteralType;

pub enum RawType {
  Composed(ComposedType),
  Literal(LiteralType),
}
