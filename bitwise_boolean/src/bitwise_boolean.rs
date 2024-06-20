extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta, NestedMeta};

pub fn bitwise_boolean_derive(input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut flag_names = Vec::new();
    let mut field_name = None;

    // check if the input is a struct
    if let Data::Struct(data_struct) = &input.data {

        // check if the struct has a field with the #[bitwise_bool] attribute
        if let Fields::Named(fields_named) = &data_struct.fields {

            // iterate over the fields and check if they have the #[bitwise_bool] attribute
            for field in &fields_named.named {
                for attr in &field.attrs {
                    if attr.path.is_ident("bitwise_bool") {
                        field_name = Some(field.ident.clone().unwrap());

                        // parse the attribute's value as a list of identifiers
                        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                            for nested_meta in meta_list.nested {
                                if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                                    if let Some(ident) = path.get_ident() {
                                        flag_names.push(ident.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let field_name = field_name.expect("Expected a field with #[bitwise_bool] attribute");

    let mut getters = Vec::new();
    let mut setters = Vec::new();

    for (i, flag_name) in flag_names.iter().enumerate() {
        let flag_ident = syn::Ident::new(flag_name, proc_macro2::Span::call_site());
        let setter_ident = syn::Ident::new(&format!("set_{}", flag_name), proc_macro2::Span::call_site());
        
        // calculate flag bit position
        let bit: u8 = 1 << i;

        // generate getter and setter methods
        getters.push(quote! {
            pub fn #flag_ident(&self) -> bool {
                self.#field_name & #bit != 0
            }
        });
        setters.push(quote! {
            pub fn #setter_ident(&mut self, value: bool) {
                if value {
                    self.#field_name |= #bit;
                } else {
                    self.#field_name &= !#bit;
                }
            }
        });
    }
    
    let expanded = quote! {
        impl #name {
            #(#getters)*
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}