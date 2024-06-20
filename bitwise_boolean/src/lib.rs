#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;

mod bitwise_boolean;

#[proc_macro_derive(BitwiseBoolean, attributes(bitwise_bool))]
pub fn bitwise_bool(input: TokenStream) -> TokenStream {
    bitwise_boolean::bitwise_boolean_derive(input)
}