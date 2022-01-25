extern crate proc_macro;

mod expand_merge;

use crate::expand_merge::merge_expand;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[track_caller]
#[proc_macro_derive(Merge)]
pub fn merge(input: TokenStream) -> TokenStream {
    merge_expand(parse_macro_input!(input as DeriveInput)).into()
}
