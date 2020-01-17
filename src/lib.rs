extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;

    for input in input.sig.inputs {
        match input {
            syn::FnArg::Receiver(_) => {
                panic!("aa");
            }
            syn::FnArg::Typed(t) => {
                let kind = t.ty.clone();
                let result = quote! {
                    fn #name(a: #kind) -> #kind { 42 + a }
                };
                return result.into();
            }
        };
    }

    let result = quote! {
        fn #name(a: u32) -> u32 { 42 + a }
    };
    result.into()
}
