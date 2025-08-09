use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

#[proc_macro_derive(FromJsonVal, attributes(json))]
pub fn derive_from_json_val(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let syn::Data::Struct(data) = input.data {
        data.fields
    } else {
        panic!("FromJsonVal can only be derived for structs");
    };

    let field_inits = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let mut key = field_name.to_string();

        for attr in &f.attrs {
            if attr.path().is_ident("json") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename") {
                        let lit: syn::LitStr = meta.value()?.parse()?;
                        key = lit.value();
                    }
                    Ok(())
                });
            }
        }

        let key_lit = syn::LitStr::new(&key, f.ident.as_ref().unwrap().span());

        match &f.ty {
            Type::Path(type_path) => {
                let type_ident = type_path.path.segments.last().unwrap().ident.to_string();

                match type_ident.as_str() {
                    "String" => quote! {
                        #field_name: val.get_string(#key_lit).unwrap_or_default()
                    },
                    "usize" => quote! {
                        #field_name: val.get_number(#key_lit).unwrap_or_default()
                    },
                    "f64" => quote! {
                        #field_name: val.get_float(#key_lit).unwrap_or_default()
                    },
                    "bool" => quote! {
                        #field_name: val.get_bool(#key_lit).unwrap_or(false)
                    },
                    _ => {
                        quote! {
                            #field_name: {
                                if let Some(v) = val.get(#key_lit) {
                                    <_ as FromJsonVal>::from_json(v).unwrap_or_default()
                                } else {
                                    Default::default()
                                }
                            }
                        }
                    }
                }
            }
            _ => panic!("Unsupported field type for {}", field_name),
        }
    });

    let expanded = quote! {
        impl FromJsonVal for #name {
            fn from_json(val: &JsonVal) -> Result<Self, String> {
                Ok(Self{
                    #(#field_inits),*
                })
            }
        }
    };
    TokenStream::from(expanded)
}
