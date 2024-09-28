use crate::derive_source::source_clause;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, GenericParam, Ident};

pub fn derive_map(input: DeriveInput) -> TokenStream {
  assert!(
    input.generics.params.len() == 1,
    "can only `derive` InfoMap for types with 1 generic"
  );
  let param = &input.generics.params[0];
  let GenericParam::Type(gen_type) = param else {
    panic!("cannot `derive` InfoMap for types with a generic const or lifetime")
  };
  let type_name = &gen_type.ident;

  let name = &input.ident;
  let info_fn = match &input.data {
    Data::Enum(data) => enum_fn(name, data),
    Data::Struct(data) => struct_fn(name, data),
    _ => panic!("can only `derive` InfoMap for structs and enums"),
  };

  let (impl_gens, ty_gens, optn_clause) = input.generics.split_for_impl();
  let where_clause = source_clause(type_name, optn_clause);

  quote_spanned! {input.span()=>
    #[automatically_derived]
    impl #impl_gens InfoMap for #name #ty_gens #where_clause {
      type GenericSelf<I0> = #name<I0>;
      #info_fn
    }
  }
}

pub fn enum_fn(name: &Ident, data: &DataEnum) -> TokenStream {
  let arms = data.variants.iter().map(|variant| {
    let variant = &variant.ident;
    let span = variant.span();
    quote_spanned! {span =>
      #name::#variant(inner) => #name::#variant(InfoMap::map(inner, f))
    }
  });

  let span = data.variants.span();
  quote_spanned! {span =>
    #[inline]
    fn map<R>(self, f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      match self {
        #(#arms),*
      }
    }
  }
}

pub fn struct_fn(name: &Ident, data: &DataStruct) -> TokenStream {
  let fields = data.fields.iter().filter_map(|field| {
    let name = field.ident.as_ref()?;
    if name == "info" {
      return None;
    };
    let ignored = field.attrs.iter().any(|attr| {
      attr
        .path()
        .get_ident()
        .is_some_and(|name| name == "map_ignore")
    });

    let value = if !ignored {
      quote! { InfoMap::map(self.#name, &mut f) }
    } else {
      quote! { self.#name }
    };
    Some(quote! { #name: #value, })
  });

  quote! {
    #[inline]
    fn map<R>(self, mut f: impl FnMut(Self::Info) -> R) -> Self::GenericSelf<R> {
      #name {
        #(#fields)*
        info: f(self.info)
      }
    }
  }
}
