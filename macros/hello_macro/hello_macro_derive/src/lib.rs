#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type,
    clippy::single_component_path_imports
)]
#[cfg(test)]
mod tests {
    use super::*;
}

use proc_macro::TokenStream;
use quote::quote;
use syn;

// This line tells Rust that this is the macro
// to call when someone does `#[derive(HelloMacro)]`.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// It's very common to split the derive macro into one function
// that parses the input (`hello_macro_derive`) and one that
// generates the code (`impl_hello_macro`).
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // `#name` will be replaced with `Pancakes` here.
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // Convert `gen` into a `TokenStream`.
    gen.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
    // todo!()
    // !
    // println!("attr: {:?}", attr);
    // println!("attr: {:?}", item);
    item
}

use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as LitStr);

    // Construct the output tokens
    let query = input.value();
    let expanded = quote! {
        {
            let test = "Executing SQL query";
            println!("{}: {}",test, #query);
            format!("{}: {}",test,#query)
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
