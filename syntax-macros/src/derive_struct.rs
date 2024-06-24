use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;

pub fn struct_info_type(data: &DataStruct) -> TokenStream {
  let info_field = data.fields.iter().find_map(|field| {
    let ident = field.ident.as_ref()?;
    (*ident == "info").then_some(field)
  });
  let Some(info_ty) = info_field.map(|field| &field.ty) else {
    panic!("could not find the `info` field on the struct")
  };

  quote! {#info_ty}
}
