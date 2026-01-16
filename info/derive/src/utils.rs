use quote::ToTokens;
use syn::{DeriveInput, Expr, Ident};

/// Finds the name of the generic type used to store `Info`
pub fn info_generic(input: &DeriveInput) -> Ident {
  let info_attr = input
    .attrs
    .iter()
    .find(|attr| attr.path().get_ident().is_some_and(|name| name == "info"));

  let ty_params: Vec<_> = input.generics.type_params().collect();
  let Some(attr) = info_attr else {
    if ty_params.len() == 1 {
      return ty_params[0].ident.clone();
    }

    panic!(
      "\
      cannot determine the type parameter used for info.\n\
      help: either add a `#[info = I]` attribute\n\
      help: or only use one type parameter\
      "
    );
  };

  let Ok(attr_list) = attr.meta.require_list() else {
    panic!(
      "\
      expected an `#[info(I)]` format attribute for the info type\n\
      but got an `{attr_tokens}` format attribute instead.\n\
      help: please use the `#[info(I)]` format.\
      ",
      attr_tokens = attr.to_token_stream()
    );
  };

  let Ok(Expr::Path(path)) = &attr_list.parse_args() else {
    panic!(
      "\
      expected a type parameter for the info type\n\
      but got the `{value_tokens}` value instead.\
      ",
      value_tokens = attr_list.to_token_stream()
    );
  };

  let Some(name) = path.path.get_ident() else {
    panic!(
      "\
      expected a type parameter for the info type\n\
      but got a full path `{path_tokens}` instead.\
      ",
      path_tokens = path.path.to_token_stream()
    )
  };

  if !ty_params.iter().any(|param| &param.ident == name) {
    panic!(
      "\
      the type identifier provided in `#[info = {name}]`\n\
      was not a type parameter to the data type.\n\
      help: use an identifier that is a type parameter\
      ",
      name = name.to_token_stream()
    )
  }

  name.clone()
}
