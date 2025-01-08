use darling::{FromAttributes, FromDeriveInput};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Ident};

#[derive(FromAttributes, Default)]
#[darling(default, attributes(param_display))]
struct ParamFieldOpt {
    display: Option<Expr>,
}

#[proc_macro_derive(ParamDisplay, attributes(param_display))]
pub fn derive_param_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let data = if let syn::Data::Struct(data) = &input.data {
        data
    } else {
        unimplemented!()
    };
    let fields = data.fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let opts = ParamFieldOpt::from_attributes(&f.attrs).expect("Wrong option");
        let output_func = match opts.display {
            Some(e) => quote! {
                #e
            },
            None => quote! {
                output()
            },
        };
        quote! {
            self.#name.map(|v| v.#output_func)
        }
    });
    let expanded = quote! {
        impl Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let output = [
                    #(#fields, )*
                ].into_iter().flatten().collect::<Vec<String>>().join("\n");
                write!(f, "{output}")
            }
        }
    };
    expanded.into()
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(keyword_display))]
struct Opts {
    field: String,
    from: Option<Ident>,
    value: Option<Ident>,
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

    let value = match opts.value {
        Some(x) => quote! {
            impl #ident {
                pub fn value(&self) -> #x {
                    self.0
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
        #value
    };
    output.into()
}
