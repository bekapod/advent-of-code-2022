extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Priority)]
pub fn priority_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Could not parse input");
    impl_priority(&ast)
}

fn impl_priority(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Priority for #name {
            fn get_priority_of_item(&self) -> u32 {
                let possible_items = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
                let index = possible_items.iter().position(|&r| self == r).expect("Could not find item in possible items");
                (index + 1).try_into().expect("Could not convert index to u32")
            }
        }
    };
    gen.into()
}