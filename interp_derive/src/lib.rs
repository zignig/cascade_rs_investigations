use proc_macro::{self, TokenStream};
use quote::quote;

mod extract;

use syn::{
    parse,parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed,
};

#[proc_macro_attribute]
pub fn ingest(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    match extract::stuff(attr, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            println!("{}", err);
            TokenStream::new()
        }
    }
}

#[proc_macro_derive(Cover)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let description = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!(
                    "a struct with these named fields: {}",
                    quote! {#(#idents), *}
                )
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with {} unnamed fields", num_fields)
            }
            syn::Fields::Unit => format!("a unit struct"),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}", quote! {#(#vs),*})
        }
        syn::Data::Union(DataUnion {
            fields: FieldsNamed { named, .. },
            ..
        }) => {
            let idents = named.iter().map(|f| &f.ident);
            format!("a union with these named fields: {}", quote! {#(#idents),*})
        }
    };
    let name = format!("{}",quote! { #ident  });
    let output = quote! {
    impl #ident {
        fn attach(){
            (#name,#ident::generate);
        }
    }
    };
    output.into()
}
