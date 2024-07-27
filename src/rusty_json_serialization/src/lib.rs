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
            let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();

            quote! {
                impl rusty_json::extra::JsonEntity for #name {
                    fn to_json(&self) -> rusty_json::base::JsonValue {
                        let mut obj = rusty_json::base::JsonObject::new();
                        #(
                            obj.set(#field_strings, &self.#field_names);
                        )*
                        rusty_json::base::JsonValue::Object(obj)
                    }

                    fn from_json(raw: &str) -> Result<Self, rusty_json::extra::ConversationError>
                        where
                            Self: Sized,
                    {
                        let value: rusty_json::base::JsonValue = rusty_json::extra::JsonParser::parse(raw)?;

                        if let rusty_json::base::JsonValue::Object(obj) = value {
                            #(
                                let #field_names = obj.get(#field_strings)
                                    .ok_or_else(|| rusty_json::extra::ConversationError::GenericError(format!("{} field missing or invalid", #field_strings)))?
                                    .parse::<#field_types>()?;
                            )*

                            Ok(#name {
                                #(#field_names,)*
                            })
                        } else {
                            Err(rusty_json::extra::ConversationError::GenericError("Expected JSON object".to_string()))
                        }
                    }
                }

                impl From<#name> for rusty_json::base::JsonValue {
                    fn from(value: #name) -> Self {
                        value.to_json()
                    }
                }

                impl From<&#name> for rusty_json::base::JsonValue {
                    fn from(value: &#name) -> Self {
                        value.to_json()
                    }
                }

                impl TryFrom<rusty_json::base::JsonValue> for #name {
                    type Error = rusty_json::base::casting::CastError;

                    fn try_from(value: rusty_json::base::JsonValue) -> Result<Self, Self::Error> {
                        if let rusty_json::base::JsonValue::Object(obj) = value {
                            #(
                                let #field_names = obj.get(#field_strings)
                                    .ok_or_else(|| rusty_json::base::casting::CastError::InvalidType)?
                                    .parse::<#field_types>()?;
                            )*

                            Ok(#name {
                                #(#field_names,)*
                            })
                        } else {
                            Err(rusty_json::base::casting::CastError::InvalidType)
                        }
                    }
                }

                impl TryFrom<&rusty_json::base::JsonValue> for #name {
                    type Error = rusty_json::base::casting::CastError;

                    fn try_from(value: &rusty_json::base::JsonValue) -> Result<Self, Self::Error> {
                        if let rusty_json::base::JsonValue::Object(ref obj) = *value {
                            #(
                                let #field_names = obj.get(#field_strings)
                                    .ok_or_else(|| rusty_json::base::casting::CastError::InvalidType)?
                                    .parse::<#field_types>()?;
                            )*

                            Ok(#name {
                                #(#field_names,)*
                            })
                        } else {
                            Err(rusty_json::base::casting::CastError::InvalidType)
                        }
                    }
                }
            }
        },
        Data::Enum(data_enum) => {
            let variant_names: Vec<_> = data_enum.variants.iter().map(|variant| &variant.ident).collect();
            let variant_strings: Vec<_> = variant_names.iter().map(|variant| variant.to_string()).collect();

            quote! {
                impl rusty_json::extra::JsonEntity for #name {
                    fn to_json(&self) -> rusty_json::base::JsonValue {
                        match self {
                            #(
                                #name::#variant_names => rusty_json::base::JsonValue::String(#variant_strings.to_string()),
                            )*
                        }
                    }

                    fn from_json(raw: &str) -> Result<Self, rusty_json::extra::ConversationError>
                        where
                            Self: Sized,
                    {
                        let value: rusty_json::base::JsonValue = rusty_json::extra::JsonParser::parse(raw)?;

                        match value {
                            rusty_json::base::JsonValue::String(ref s) => {
                                match s.as_str() {
                                    #(
                                        #variant_strings => Ok(#name::#variant_names),
                                    )*
                                    _ => Err(rusty_json::extra::ConversationError::GenericError("Unexpected variant".to_string()))
                                }
                            },
                            _ => Err(rusty_json::extra::ConversationError::GenericError("Expected JSON string".to_string()))
                        }
                    }
                }

                impl From<#name> for rusty_json::base::JsonValue {
                    fn from(value: #name) -> Self {
                        value.to_json()
                    }
                }

                impl From<&#name> for rusty_json::base::JsonValue {
                    fn from(value: &#name) -> Self {
                        value.to_json()
                    }
                }

                impl TryFrom<rusty_json::base::JsonValue> for #name {
                    type Error = rusty_json::base::casting::CastError;

                    fn try_from(value: rusty_json::base::JsonValue) -> Result<Self, Self::Error> {
                        if let rusty_json::base::JsonValue::String(ref s) = value {
                            match s.as_str() {
                                #(
                                    #variant_strings => Ok(#name::#variant_names),
                                )*
                                _ => Err(rusty_json::base::casting::CastError::InvalidType)
                            }
                        } else {
                            Err(rusty_json::base::casting::CastError::InvalidType)
                        }
                    }
                }

                impl TryFrom<&rusty_json::base::JsonValue> for #name {
                    type Error = rusty_json::base::casting::CastError;

                    fn try_from(value: &rusty_json::base::JsonValue) -> Result<Self, Self::Error> {
                        if let rusty_json::base::JsonValue::String(ref s) = *value {
                            match s.as_str() {
                                #(
                                    #variant_strings => Ok(#name::#variant_names),
                                )*
                                _ => Err(rusty_json::base::casting::CastError::InvalidType)
                            }
                        } else {
                            Err(rusty_json::base::casting::CastError::InvalidType)
                        }
                    }
                }
            }
        },
        _ => unimplemented!()
    };

    gen.into()
}
