use std::collections::HashMap;

use crate::{ident::Ident, types::Type, values::Value};

pub struct Module<TI, VI> {
  pub types: HashMap<Ident<TI>, Type<TI>>,
  pub values: HashMap<Ident<VI>, Value<VI>>,
}
