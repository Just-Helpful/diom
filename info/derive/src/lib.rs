use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive_map;
use derive_map::derive_map;
mod derive_ref;
use derive_ref::derive_ref;
mod derive_source;
use derive_source::derive_source;

#[proc_macro_derive(InfoSource)]
pub fn derive_info_source(input: TokenStream) -> TokenStream {
  derive_source(parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(InfoRef)]
pub fn derive_info_ref(input: TokenStream) -> TokenStream {
  derive_ref(parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(InfoMap, attributes(map_ignore))]
pub fn derive_info_map(input: TokenStream) -> TokenStream {
  derive_map(parse_macro_input!(input as DeriveInput)).into()
}
