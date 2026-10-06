use clonpcoat_view::parse::View;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn view(tokens: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(tokens as View);
    quote! { #parsed }.into()
}
