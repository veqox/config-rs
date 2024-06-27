extern crate proc_macro;

use darling::FromField;
use proc_macro::TokenStream;
use quote::quote;

#[derive(FromField)]
#[darling(attributes(config))]
struct ConfigOptions {
    #[darling(default)]
    rename: Option<String>,
}

#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(_item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(_item as syn::DeriveInput);
    let name = input.ident;

    let mut initializers = Vec::new();

    if let syn::Data::Struct(data_struct) = input.data {
        if let syn::Fields::Named(fields_named) = data_struct.fields {
            for field in fields_named.named {
                let config_options = ConfigOptions::from_field(&field).unwrap();

                let field_name = field.ident.clone().unwrap();
                let field_type = field.ty.clone();
                let env_var_name = config_options
                    .rename
                    .unwrap_or_else(|| field_name.to_string().to_uppercase());

                initializers.push(quote! {
                    #field_name: {
                        let var = std::env::var(#env_var_name).or(dotenv::var(#env_var_name));

                        match var {
                            Ok(var) => match var.parse::<#field_type>() {
                                Ok(var) => Some(var),
                                Err(e) => {
                                    tracing::info!("Error parsing {}: {:?}", #env_var_name, e);
                                    None
                                }
                            },
                            Err(_) => {
                                tracing::error!("{} is not set", #env_var_name);
                                None
                            }
                        }
                    }?
                });
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn from_env() -> Option<Self> {
                Some(Self {
                    #(#initializers),*
                })
            }
        }
    };

    expanded.into()
}
