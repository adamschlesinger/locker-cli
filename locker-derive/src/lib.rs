/// todo
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// todo
#[proc_macro_derive(Bar)]
pub fn bar(input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    // let name = "";
    //
    // let expanded = quote! {
    //     impl Bar for #name {
    //         fn hello_world() {
    //             println!("Hello, World! My name is {}", stringify!(#name));
    //         }
    //     }
    // };

    // return TokenStream::from(expanded);
    let expanded = quote! {
        impl Moo {
            pub fn qwe() {}
        }
    };

    TokenStream::from(expanded)
}
