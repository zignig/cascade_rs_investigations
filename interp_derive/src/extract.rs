use proc_macro2::{  TokenStream as TokenStream2, };
use quote::{ quote, ToTokens};
use syn::{
    parse::{Nothing},
    parse2,
    Ident, Result
};

pub fn stuff<T: Into<TokenStream2>, E: Into<TokenStream2>>(
    attr: T,
    tokens: E,
) -> Result<TokenStream2> {
    let attr = attr.into();
    let stuff: syn::Item = parse2(tokens.into()).expect("FNORD");
    println!(
        "to me: \"{}\"",
        stuff.clone().into_token_stream().to_string()
    );
    let ident = match stuff.clone() {
        syn::Item::Mod(mod_name) => {
            println!(
                "the_mod: \"{}\"",
                mod_name.clone().into_token_stream().to_string()
            );
            Some(mod_name.ident)
        }
        syn::Item::Fn(item_fn) => Some(item_fn.sig.ident),
        _ => None,
    };
    let ident = match ident {
        Some(ident) => {
            if let Ok(_) = parse2::<Nothing>(attr.clone()) {
                ident
            } else {
                parse2::<Ident>(attr)?
            }
        }
        None => parse2::<Ident>(attr)?,
    };
    let output = quote! {
        fn extracted(){
            #ident
        }
    };
    Ok(output)
}
