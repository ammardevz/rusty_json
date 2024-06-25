use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

extern crate proc_macro;

#[proc_macro_derive(JsonEntity)]
pub fn derive_json_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = match &input.data {
        Data::Struct(data_struct) => {
            let fields: Vec<_> = match &data_struct.fields {
                Fields::Named(fields) => fields.named.iter().collect(),
                _ => panic!("JsonEntity can only be derived for structs with named fields."),
            };

            let field_names: Vec<_> = fields.iter().map(|field| &field.ident).collect();
            let field_strings: Vec<_> = field_names.iter().map(|field| field.as_ref().unwrap().to_string()).collect();

            quote! {
                // Implement JsonEntity for #name
                impl rusty_json::extra::JsonEntity for #name {
                    fn to_json(&self) -> rusty_json::base::JsonValue {
                        let mut obj = rusty_json::base::JsonObject::new();
                        #(
                            obj.set(#field_strings, &self.#field_names);
                        )*
                        rusty_json::base::JsonValue::Object(obj)
                    }
                }

                // Implement Into<JsonValue> for #name
                impl Into<rusty_json::base::JsonValue> for #name {
                    fn into(self) -> rusty_json::base::JsonValue {
                        self.to_json()
                    }
                }

                // Implement JsonEntity for & #name
                impl rusty_json::extra::JsonEntity for & #name {
                    fn to_json(&self) -> rusty_json::base::JsonValue {
                        let mut obj = rusty_json::base::JsonObject::new();
                        #(
                            obj.set(#field_strings, &self.#field_names);
                        )*
                        rusty_json::base::JsonValue::Object(obj)
                    }
                }

                // Implement Into<JsonValue> for & #name
                impl Into<rusty_json::base::JsonValue> for & #name {
                    fn into(self) -> rusty_json::base::JsonValue {
                        self.to_json()
                    }
                }
            }

        },
        Data::Enum(data_enum) => {
            let variant_names: Vec<_> = data_enum.variants.iter().map(|variant| &variant.ident).collect();
            let variant_strings: Vec<_> = variant_names.iter().map(|variant| variant.to_string()).collect();

            quote! {
                // Implement JsonEntity for #name
                impl rusty_json::extra::JsonEntity for #name {
                    fn to_json(&self) -> rusty_json::base::JsonValue {
                        match self {
                            #(
                                #name::#variant_names => rusty_json::base::JsonValue::String(#variant_strings.to_string()),
                            )*
                        }
                    }
                }

                // Implement Into<JsonValue> for #name
                impl Into<rusty_json::base::JsonValue> for #name {
                    fn into(self) -> rusty_json::base::JsonValue {
                        self.to_json()
                    }
                }

                // Implement JsonEntity for & #name
                impl rusty_json::extra::JsonEntity for & #name {
                    fn to_json(&self) -> rusty_json::base::JsonValue {
                        match *self {
                            #(
                                #name::#variant_names => rusty_json::base::JsonValue::String(#variant_strings.to_string()),
                            )*
                        }
                    }
                }

                // Implement Into<JsonValue> for & #name
                impl Into<rusty_json::base::JsonValue> for & #name {
                    fn into(self) -> rusty_json::base::JsonValue {
                        self.to_json()
                    }
                }
            }

        },
        _ => unimplemented!()
    };

    gen.into()
}
