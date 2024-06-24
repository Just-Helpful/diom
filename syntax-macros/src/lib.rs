extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod derive_enum;
use derive_enum::{enum_info_body, enum_info_type};
mod derive_struct;
use derive_struct::struct_info_type;

#[proc_macro_derive(InfoSource)]
pub fn derive_info_source(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let (info_body, info_ty) = match &input.data {
    Data::Struct(data) => (quote! {&self.info}, struct_info_type(data)),
    Data::Enum(data) => (enum_info_body(data), enum_info_type(data)),
    _ => panic!("can only `derive` InfoSource for structs and enums"),
  };

  let name = input.ident;
  let (impl_gens, ty_gens, where_clause) = input.generics.split_for_impl();
  quote! {
    #[automatically_derived]
    impl #impl_gens InfoSource for #name #ty_gens #where_clause {
      type Info = #info_ty;
      fn info(&self) -> &Self::Info {
        #info_body
      }
    }
  }
  .into()
}
