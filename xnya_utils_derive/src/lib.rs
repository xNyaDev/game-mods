use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed};

#[proc_macro_derive(TypedFields)]
pub fn typed_fields_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let fields = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named
    } else {
        panic!("Only named fields are supported");
    };

    let match_arms = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! {
            stringify!(#name) => Some(stringify!(#ty)),
        }
    });

    let result = quote! {
        impl TypedFields for #ident {
            fn get_field_type(field: &str) -> Option<&'static str> {
                match field {
                    #(#match_arms)*
                    _ => None
                }
            }
        }
    };

    result.into()
}

#[proc_macro_derive(ReflectionFields)]
pub fn reflection_fields_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let fields = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named
    } else {
        panic!("Only named fields are supported");
    };

    let match_arms = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        quote! {
            stringify!(#name) => Some(format!("{:?}", self.#name)),
        }
    });

    let result = quote! {
        impl ReflectionFields for #ident {
            fn get_field_value(&self, field: &str) -> Option<String> {
                match field {
                    #(#match_arms)*
                    _ => None
                }
            }
        }
    };

    result.into()
}
