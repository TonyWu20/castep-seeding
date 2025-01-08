use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(keyword_display))]
struct Opts {
    field: String,
    from: Option<Ident>,
}

#[proc_macro_derive(KeywordDisplay, attributes(keyword_display))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong option");
    let DeriveInput { ident, .. } = input;
    let field = opts.field;
    let field_text = quote! {
        fn field(&self) -> String {
            #field.to_string()
        }
    };

    let from = match opts.from {
        Some(x) => quote! {
            impl From<#x> for #ident {
                fn from(value: #x) -> Self {
                    Self(value)
                }
            }
        },
        None => quote! {},
    };

    let output = quote! {
        impl KeywordDisplay for #ident {
            #field_text
        }
        #from
    };
    output.into()
}
