use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Register)]
pub fn register(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl Test for #ident {
            #[doc = "Dingus"]
            fn test() -> bool {
                true
            }
        }
    };

    output.into()
}
