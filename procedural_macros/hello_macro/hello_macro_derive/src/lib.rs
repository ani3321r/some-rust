extern crate proc_macro;

use proc_macro::TokenStream; // proc_macro is used to manipulate rust code
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream{
    // constructing rust code as a syntax tree that can be manipulated
    let ast = syn::parse(input).unwrap();

    // trait implimentation
    impl_hello_macro(&ast)
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream{
    let name = &ast.ident;
    let gen = quote!{
        impl HelloMacro for #name{
            fn hello_macro(){
                println!(
                    "Hello, {}",
                    stringify!(#name)
                );
            }
        }
    };
    gen.into()
}
