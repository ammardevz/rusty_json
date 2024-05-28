use std::fmt::{Display, Formatter};

use crate::base::json_arr::JsonArray;
use crate::base::json_o::JsonObject;

pub enum JsonValue {
    String(String),
    Number(JsonNumber),
    Boolean(bool),
    Object(JsonObject),
    Array(JsonArray),
    Null,
}

pub enum JsonNumber {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
}

macro_rules! impl_json_value {
    ($type:ty, $variant:ident) => {
        impl From<$type> for JsonValue {
            fn from(value: $type) -> Self {
                JsonValue::$variant(value)
            }
        }
    };
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(value.to_string())
    }
}

impl From<JsonValue> for JsonArray {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Array(arr) => arr,
            _ => JsonArray::new(), // Default behavior when not converting from JsonValue::Array
        }
    }
}

impl From<JsonValue> for JsonObject {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Object(obj) => obj,
            _ => JsonObject::new(), // Alternatively, you can return an empty object or handle differently
        }
    }
}




impl_json_value!(String, String);
impl_json_value!(bool, Boolean);
impl_json_value!(JsonObject, Object);
impl_json_value!(JsonArray, Array);

macro_rules! impl_json_number {
    ($($type:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$type> for JsonNumber {
                fn from(value: $type) -> Self {
                    JsonNumber::$variant(value)
                }
            }

            impl From<$type> for JsonValue {
                fn from(value: $type) -> Self {
                    JsonValue::Number(JsonNumber::$variant(value))
                }
            }
        )*
    };
}

impl_json_number!(
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    i128 => I128,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    u128 => U128,
    f32 => F32,
    f64 => F64,
);

impl Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::String(v) => write!(f, "\"{}\"", v)?,
            JsonValue::Number(v) => {
                match v {
                    JsonNumber::I8(n) => write!(f, "{}", n)?,
                    JsonNumber::I16(n) => write!(f, "{}", n)?,
                    JsonNumber::I32(n) => write!(f, "{}", n)?,
                    JsonNumber::I64(n) => write!(f, "{}", n)?,
                    JsonNumber::I128(n) => write!(f, "{}", n)?,
                    JsonNumber::U8(n) => write!(f, "{}", n)?,
                    JsonNumber::U16(n) => write!(f, "{}", n)?,
                    JsonNumber::U32(n) => write!(f, "{}", n)?,
                    JsonNumber::U64(n) => write!(f, "{}", n)?,
                    JsonNumber::U128(n) => write!(f, "{}", n)?,
                    JsonNumber::F32(n) => write!(f, "{}", n)?,
                    JsonNumber::F64(n) => write!(f, "{}", n)?,
                }
            }
            JsonValue::Boolean(v) => write!(f, "{}", v)?,
            JsonValue::Object(v) => write!(f, "{}", v)?,
            JsonValue::Array(v) => write!(f, "{}", v)?,
            JsonValue::Null => write!(f, "null")?,
        }
        Ok(())
    }
}

impl Display for JsonNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNumber::I8(n) => write!(f, "{}", n)?,
            JsonNumber::I16(n) => write!(f, "{}", n)?,
            JsonNumber::I32(n) => write!(f, "{}", n)?,
            JsonNumber::I64(n) => write!(f, "{}", n)?,
            JsonNumber::I128(n) => write!(f, "{}", n)?,
            JsonNumber::U8(n) => write!(f, "{}", n)?,
            JsonNumber::U16(n) => write!(f, "{}", n)?,
            JsonNumber::U32(n) => write!(f, "{}", n)?,
            JsonNumber::U64(n) => write!(f, "{}", n)?,
            JsonNumber::U128(n) => write!(f, "{}", n)?,
            JsonNumber::F32(n) => write!(f, "{}", n)?,
            JsonNumber::F64(n) => write!(f, "{}", n)?,
        }
        Ok(())
    }
}
