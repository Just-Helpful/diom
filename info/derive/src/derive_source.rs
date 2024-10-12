use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
  spanned::Spanned, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Ident, WhereClause,
};

pub fn derive_source(input: DeriveInput) -> TokenStream {
  let info_ty = match &input.data {
    Data::Enum(data) => enum_type(data),
    Data::Struct(data) => struct_type(data),
    _ => panic!("can only `derive` InfoSource for structs and enums"),
  };

  let name = &input.ident;
  let (impl_gens, ty_gens, where_clause) = input.generics.split_for_impl();
  quote_spanned! {input.span()=>
    #[automatically_derived]
    impl #impl_gens InfoSource for #name #ty_gens #where_clause {
      type Info = #info_ty;
    }
  }
}

pub fn enum_type(data: &DataEnum) -> TokenStream {
  let Some(variant) = data.variants.first() else {
    panic!("expected at least one enum variant")
  };

  assert!(
    variant.fields.len() <= 1,
    "expected variants no more than one field"
  );
  let field = &variant
    .fields
    .iter()
    .next()
    .expect("expected variants to have at least one field");
  assert!(field.ident.is_none(), "expected variants to be tuple-like");
  let field_ty = &field.ty;
  quote_spanned! {field_ty.span() => <#field_ty as InfoSource>::Info}
}

pub fn struct_type(data: &DataStruct) -> TokenStream {
  let info_field = data.fields.iter().find_map(|field| {
    let ident = field.ident.as_ref()?;
    (*ident == "info").then_some(field)
  });
  let Some(info_ty) = info_field.map(|field| &field.ty) else {
    panic!("could not find the `info` field on the struct")
  };

  quote! {#info_ty}
}

/// Adds the `Self: InfoSource` to a where clause
pub fn source_clause(type_name: &Ident, clause: Option<&WhereClause>) -> proc_macro2::TokenStream {
  if let Some(clause) = clause {
    let optn_sep = (!clause.predicates.trailing_punct()).then(Comma::default);
    quote! { #clause #optn_sep Self: InfoSource<Info = #type_name> }
  } else {
    quote! { where Self: InfoSource<Info = #type_name> }
  }
}
