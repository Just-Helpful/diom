use std::fmt::Debug;

/// A wrapper type that provides definitions for:
/// - `expression` layers
/// - `pattern` layers
/// - `type` layers
///
/// - `ident`ifiers and `path`s
pub trait SyntaxScope {
  /// The type of the sub-layers for expressions
  type Expression: Clone + Debug;
  /// The type of the sub-layers for patterns
  type Pattern: Clone + Debug;
  /// The type of the sub-layers for types
  type Type: Clone + Debug;

  /// The type of the sub-layers for identifiers
  type Ident: Clone + Debug;
  /// The type of the sub-layers for
  type Path: Clone + Debug;
}
