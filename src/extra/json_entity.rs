use crate::base::{JsonArray, JsonObject, JsonValue};
use crate::extra::json_parser::ConversationError;
use crate::extra::JsonParser;
use std::collections::{BTreeMap, HashMap};
use indexmap::IndexMap;
use std::hash::Hash;

/// Trait for types that can be converted to and from JSON.
pub trait JsonEntity: Clone {
    /// Convert the struct implementing this trait to a JSON value.
    fn to_json(&self) -> JsonValue;

    /// Parse raw JSON string into an instance of the implementing struct.
    fn from_json(raw: &str) -> Result<Self, ConversationError>
        where
            Self: Sized;
}

impl<T> JsonEntity for Vec<T>
    where
        T: Clone + Into<JsonValue> + TryFrom<JsonValue>,
        ConversationError: From<<T as TryFrom<JsonValue>>::Error>,
{
    fn to_json(&self) -> JsonValue {
        let arr = self.iter().map(|t| t.clone().into()).collect::<JsonArray>();
        JsonValue::Array(arr)
    }

    fn from_json(raw: &str) -> Result<Self, ConversationError>
        where
            Self: Sized,
    {
        let parsed = JsonParser::parse(raw)?;
        if let JsonValue::Array(arr) = parsed {
            arr.iter()
                .map(|value| T::try_from(value.clone()).map_err(|e| e.into()))
                .collect()
        } else {
            Err(ConversationError::GenericError("Expected JSON array".to_string()))
        }
    }
}

macro_rules! register_map {
    ($type:ident, $($extra_bounds:tt)*) => {
        impl<K, V> JsonEntity for $type<K, V>
        where
            K: Clone + ToString + From<String> + $($extra_bounds)*,
            V: Clone + Into<JsonValue> + TryFrom<JsonValue>,
            ConversationError: From<<V as TryFrom<JsonValue>>::Error>,
        {
            fn to_json(&self) -> JsonValue {
                let mut obj = JsonObject::new();
                for (k, v) in self {
                    obj.set(k.to_string(), v.clone().into());
                }
                JsonValue::Object(obj)
            }

            fn from_json(raw: &str) -> Result<Self, ConversationError>
            where
                Self: Sized,
            {
                let parsed = JsonParser::parse(raw)?;
                if let JsonValue::Object(obj) = parsed {
                    let mut map = Self::default();
                    for (k, v) in obj {
                        map.insert(K::from(k), V::try_from(v).map_err(|e| e.into())?);
                    }
                    Ok(map)
                } else {
                    Err(ConversationError::GenericError("Expected JSON object".to_string()))
                }
            }
        }
    };
}

// Register different map types
register_map!(HashMap, Eq + Hash);
register_map!(IndexMap, Eq + Hash);
register_map!(BTreeMap, Ord);