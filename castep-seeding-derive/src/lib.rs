use darling::{FromAttributes, FromDeriveInput};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput, Expr, Ident};

#[derive(FromAttributes, Default)]
#[darling(default, attributes(param_display))]
struct ParamFieldOpt {
    use_ref: Option<bool>,
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
        let field = match opts.use_ref {
            Some(true) => quote! {
                self.#name.as_ref()
            },
            Some(false) | None => quote! {
                self.#name
            },
        };
        let output_func = match opts.display {
            Some(e) => quote! {
                #e
            },
            None => quote! {
                output()
            },
        };
        quote! {
            #field.map(|v| v.#output_func)
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
    specified_fields: Option<bool>,
    direct_display: Option<bool>,
    display_format: Option<String>,
    from: Option<Ident>,
    value: Option<Ident>,
    borrowed_value: Option<Ident>,
}

#[derive(FromAttributes, Default)]
#[darling(default, attributes(keyword_display))]
struct EnumAttrs {
    field: String,
    display_format: Option<String>,
}

fn data_enum_display_impl(data_enum: &DataEnum, struct_ident: &Ident) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|v| {
        let name = &v.ident;
        let opts = EnumAttrs::from_attributes(&v.attrs).expect("Wrong attrs");
        let display_format = if let Some(s) = opts.display_format {
            quote!(#s)
        } else {
            quote!("{}")
        };
        match &v.fields {
            // Don't expect this in enum
            syn::Fields::Named(_) => unimplemented!(),
            syn::Fields::Unnamed(_) => quote! {
                #struct_ident::#name(t) => write!(f, #display_format, t)},
            syn::Fields::Unit => quote! {#struct_ident::#name => write!(f, "{:?}", self)},
        }
    });
    quote! {
        impl Display for #struct_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#variants,)*
                }
            }
        }
    }
}

fn data_enum_field_impl(data_enum: &DataEnum, struct_ident: &Ident) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|v| {
        let name = &v.ident;
        let variant_expr = match v.fields {
            // Don't expect this in enum
            syn::Fields::Named(_) => unimplemented!(),
            syn::Fields::Unnamed(_) => quote! {#struct_ident::#name(_)},
            syn::Fields::Unit => quote! {#struct_ident::#name},
        };
        let opts = EnumAttrs::from_attributes(&v.attrs).expect("Wrong attrs");
        let field = opts.field;
        quote! {
            #variant_expr => #field.to_string()
        }
    });
    quote! {
        fn field(&self) -> String {
            match self {
                #(#variants,)*
            }
        }
    }
}

#[proc_macro_derive(KeywordDisplay, attributes(keyword_display))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong option");
    let DeriveInput { ident, .. } = input;
    let field = opts.field;
    let field_text = match &input.data {
        syn::Data::Struct(_) => {
            quote! {
                fn field(&self) -> String {
                    #field.to_string()
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            if let Some(true) = opts.specified_fields {
                data_enum_field_impl(data_enum, &ident)
            } else {
                quote! {
                    fn field(&self) -> String {
                        #field.to_string()
                    }
                }
            }
        }
        syn::Data::Union(_) => unimplemented!(),
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

    let borrowed_value = if let Some(x) = opts.borrowed_value {
        quote! {
            impl #ident {
                pub fn value(&self) -> &#x {
                    &self.0
                }
            }
        }
    } else {
        quote! {}
    };

    let direct_display = if let Some(false) = opts.direct_display {
        quote! {}
    } else {
        let expr = if let Some(expr) = opts.display_format {
            quote! {#expr}
        } else {
            quote! {"{}"}
        };
        match input.data {
            syn::Data::Struct(_) => {
                quote! {
                impl Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, #expr, self.0)
                    }
                }
                }
            }
            syn::Data::Enum(data_enum) => data_enum_display_impl(&data_enum, &ident),
            // We don't use union so far
            syn::Data::Union(_) => unimplemented!(),
        }
    };

    let output = quote! {
        impl KeywordDisplay for #ident {
            #field_text
        }
        #from
        #value
        #borrowed_value
        #direct_display
    };
    output.into()
}
