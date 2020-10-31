use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};

#[proc_macro]
pub fn example(_: TokenStream) -> TokenStream {
    let spanned = quote_spanned!(Span::mixed_site() => var);
    let unspanned = quote!(var);
    let tokens = quote! {
        let #spanned = 10;
        println!("spanned: {}", #spanned);
        println!("unspanned: {}", #unspanned);
    };
    tokens.into()
}
