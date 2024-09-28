use crate::derive_source::source_clause;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataEnum, DeriveInput, GenericParam};

pub fn derive_ref(input: DeriveInput) -> TokenStream {
  assert!(
    input.generics.params.len() == 1,
    "can only `derive` InfoMap for types with 1 generic"
  );
  let param = &input.generics.params[0];
  let GenericParam::Type(gen_type) = param else {
    panic!("cannot `derive` InfoMap for types with a generic const or lifetime")
  };
  let gen_name = &gen_type.ident;

  let name = &input.ident;
  let info_body = match &input.data {
    Data::Enum(data) => enum_body(data),
    Data::Struct(_) => quote! { &self.info },
    _ => panic!("can only `derive` InfoMap for structs and enums"),
  };

  let (impl_gens, ty_gens, optn_clause) = input.generics.split_for_impl();
  let where_clause = source_clause(gen_name, optn_clause);

  quote_spanned! {input.span()=>
    #[automatically_derived]
    impl #impl_gens InfoRef for #name #ty_gens #where_clause {
      #[inline]
      fn info(&self) -> &Self::Info {
        #info_body
      }
    }
  }
}

pub fn enum_body(data: &DataEnum) -> TokenStream {
  let arms = data.variants.iter().flat_map(|variant| {
    let name = &variant.ident;
    let span = variant.span();
    quote_spanned! {span =>
      Self::#name(inner) => InfoRef::info(inner),
    }
  });

  let span = data.variants.span();
  quote_spanned! {span =>
    match self {
      #(#arms)*
    }
  }
}
