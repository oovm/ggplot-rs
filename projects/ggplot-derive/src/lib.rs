extern crate proc_macro;

mod expand_merge;
mod expand_merge_add;
mod expand_merge_serde;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use crate::expand_merge::merge_expand;

#[proc_macro_derive(Merge)]
pub fn merge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    merge_expand(input).into()
}

#[proc_macro_derive(MergeAdd)]
pub fn merge_add(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    merge_expand(input).into()
}

#[proc_macro_derive(MergeSerde)]
pub fn merge_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    merge_expand(input).into()
}