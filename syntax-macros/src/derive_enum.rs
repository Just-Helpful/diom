extern crate proc_macro2;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, DataEnum};

pub fn enum_info_body(data: &DataEnum) -> TokenStream {
  let arms: TokenStream = data
    .variants
    .iter()
    .flat_map(|variant| {
      let name = &variant.ident;
      let span = variant.span();
      quote_spanned! {span => Self::#name(inner) => InfoSource::info(inner),}
    })
    .collect();

  let span = data.variants.span();
  quote_spanned! {span =>
    match &self { #arms }
  }
}

pub fn enum_info_type(data: &DataEnum) -> TokenStream {
  let Some(variant) = data.variants.first() else {
    panic!("expected at least one enum variant")
  };

  assert!(
    variant.fields.len() == 1,
    "expected variants to have one field"
  );
  let field = &variant.fields.iter().next().unwrap();
  assert!(field.ident.is_none(), "expected variants to be tuple-like");
  let field_ty = &field.ty;
  quote_spanned! {field_ty.span() => <#field_ty as InfoSource>::Info}
}
