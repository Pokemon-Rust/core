mod controller;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(ControllerOwnership)]
pub fn controller_ownership_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    controller::impl_controller_ownership(&ast)
}