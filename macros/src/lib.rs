extern crate proc_macro;

use quote::quote;
use std::env;

#[proc_macro]
/// Create a `static FJP_VERSION: &str`
pub fn fjp_version(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use env::var;

    let version = var("COMMIT").map_or_else(
        |_| env!("CARGO_PKG_VERSION").to_string(),
        |v| {
            if v.is_empty() {
                env!("CARGO_PKG_VERSION").to_string()
            } else {
                concat!(env!("CARGO_PKG_VERSION"), '-').to_string() + &v
            }
        },
    );

    let tokens = quote! {
        static FJP_VERSION: &str = #version;
    };

    tokens.into()
}
