use clonpcoat_view::parse::View;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let _parsed = syn::parse_macro_input!(input as View);
    quote! { () }.into()
    // let parsed = syn::parse_macro_input!(input as View);
    // quote! { parsed }.into()
    // The lines above are from the original code, but were commented for compilation
    // reasons. (#### Dive into later to understand! ####)
}
