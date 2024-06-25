use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Display, Formatter};

use indexmap::{IndexMap, IndexSet};
use crate::base::casting::CastError;

use crate::base::json_array::JsonArray;
use crate::base::json_object::JsonObject;

pub enum JsonValue {
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    Object(JsonObject),
    Array(JsonArray),
}

type ParseError = CastError;

impl JsonValue {
    pub fn parse<T>(&self) -> Result<T, ParseError>
        where
            T: for<'a> TryFrom<&'a JsonValue, Error = ParseError>,
    {
        match T::try_from(self) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}



impl Clone for JsonValue {
    fn clone(&self) -> Self {
        match self {
            JsonValue::String(s) => JsonValue::String(s.clone()),
            JsonValue::Number(n) => JsonValue::Number(*n),
            JsonValue::Null => JsonValue::Null,
            JsonValue::Boolean(b) => JsonValue::Boolean(*b),
            JsonValue::Object(obj) => JsonValue::Object(obj.clone()),
            JsonValue::Array(arr) => JsonValue::Array(arr.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (JsonValue::String(s1), JsonValue::String(s2)) => s1.clone_from(s2),
            (JsonValue::Number(n1), JsonValue::Number(n2)) => *n1 = *n2,
            (JsonValue::Boolean(b1), JsonValue::Boolean(b2)) => *b1 = *b2,
            (JsonValue::Object(o1), JsonValue::Object(o2)) => o1.clone_from(o2),
            (JsonValue::Array(a1), JsonValue::Array(a2)) => a1.clone_from(a2),
            (JsonValue::Null, _) => {}
            _ => {}
        }
    }
}


impl PartialEq for JsonValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JsonValue::String(s1), JsonValue::String(s2)) => s1 == s2,
            (JsonValue::Number(n1), JsonValue::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (JsonValue::Null, JsonValue::Null) => true,
            (JsonValue::Boolean(b1), JsonValue::Boolean(b2)) => b1 == b2,
            (JsonValue::Object(o1), JsonValue::Object(o2)) => o1 == o2,
            (JsonValue::Array(a1), JsonValue::Array(a2)) => a1 == a2,
            _ => false,
        }
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::String(string) => write!(f, "\"{}\"", string)?,
            JsonValue::Number(num) => write!(f, "{}", num)?,
            JsonValue::Null => write!(f, "null")?,
            JsonValue::Boolean(bool) => write!(f, "{}", bool)?,
            JsonValue::Object(obj) => write!(f, "{}", obj)?,
            JsonValue::Array(arr) => write!(f, "{}", arr)?,
        }
        Ok(())
    }
}

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        JsonValue::String(value)
    }
}

impl From<&String> for JsonValue {
    fn from(value: &String) -> Self {
        JsonValue::String(value.clone())
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(value.to_string())
    }
}

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&f32> for JsonValue {
    fn from(value: &f32) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        JsonValue::Number(value)
    }
}

impl From<&f64> for JsonValue {
    fn from(value: &f64) -> Self {
        JsonValue::Number(*value)
    }
}

impl From<i8> for JsonValue {
    fn from(value: i8) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&i8> for JsonValue {
    fn from(value: &i8) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<i16> for JsonValue {
    fn from(value: i16) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&i16> for JsonValue {
    fn from(value: &i16) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<i32> for JsonValue {
    fn from(value: i32) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&i32> for JsonValue {
    fn from(value: &i32) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<i64> for JsonValue {
    fn from(value: i64) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&i64> for JsonValue {
    fn from(value: &i64) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<i128> for JsonValue {
    fn from(value: i128) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&i128> for JsonValue {
    fn from(value: &i128) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<isize> for JsonValue {
    fn from(value: isize) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&isize> for JsonValue {
    fn from(value: &isize) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<u8> for JsonValue {
    fn from(value: u8) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&u8> for JsonValue {
    fn from(value: &u8) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<u16> for JsonValue {
    fn from(value: u16) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&u16> for JsonValue {
    fn from(value: &u16) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<u32> for JsonValue {
    fn from(value: u32) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&u32> for JsonValue {
    fn from(value: &u32) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<u64> for JsonValue {
    fn from(value: u64) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&u64> for JsonValue {
    fn from(value: &u64) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<u128> for JsonValue {
    fn from(value: u128) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&u128> for JsonValue {
    fn from(value: &u128) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<usize> for JsonValue {
    fn from(value: usize) -> Self {
        JsonValue::Number(value as f64)
    }
}

impl From<&usize> for JsonValue {
    fn from(value: &usize) -> Self {
        JsonValue::Number(*value as f64)
    }
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Boolean(value)
    }
}

impl From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        JsonValue::Object(value)
    }
}

impl From<JsonArray> for JsonValue {
    fn from(value: JsonArray) -> Self {
        JsonValue::Array(value)
    }
}

impl<T, const N: usize> From<[T; N]> for JsonValue
    where T: Into<JsonValue>
{
    fn from(value: [T; N]) -> Self {
        let mut arr = JsonArray::new();
        for object in value {
            arr.push(object.into());
        }
        JsonValue::Array(arr)
    }
}

impl<T> From<Vec<T>> for JsonValue
    where T: Into<JsonValue>
{
    fn from(value: Vec<T>) -> Self {
        let mut arr = JsonArray::new();
        for object in value {
            arr.push(object.into());
        }
        JsonValue::Array(arr)
    }
}

impl<T> From<&Vec<T>> for JsonValue
    where T: Into<JsonValue> + Clone
{
    fn from(value: &Vec<T>) -> Self {
        let mut arr = JsonArray::new();
        for object in value {
            arr.push(object.clone().into());
        }
        JsonValue::Array(arr)
    }
}

impl<T> From<IndexSet<T>> for JsonValue
    where T: Into<JsonValue>
{
    fn from(value: IndexSet<T>) -> Self {
        let mut arr = JsonArray::new();
        for object in value {
            arr.push(object.into());
        }
        JsonValue::Array(arr)
    }
}

impl<T> From<HashSet<T>> for JsonValue
    where T: Into<JsonValue>
{
    fn from(value: HashSet<T>) -> Self {
        let mut arr = JsonArray::new();
        for object in value {
            arr.push(object.into());
        }
        JsonValue::Array(arr)
    }
}

impl<T> From<BTreeSet<T>> for JsonValue
    where T: Into<JsonValue>
{
    fn from(value: BTreeSet<T>) -> Self {
        let mut arr = JsonArray::new();
        for object in value {
            arr.push(object.into());
        }
        JsonValue::Array(arr)
    }
}

impl<K, V> From<HashMap<K, V>> for JsonValue
    where K: Into<String>,
          V: Into<JsonValue>
{
    fn from(value: HashMap<K, V>) -> Self {
        let mut obj = JsonObject::new();
        for (k, v) in value {
            obj.set(k.into(), v.into());
        }
        JsonValue::Object(obj)
    }
}

impl<K, V> From<IndexMap<K, V>> for JsonValue
    where K: Into<String>,
          V: Into<JsonValue>
{
    fn from(value: IndexMap<K, V>) -> Self {
        let mut obj = JsonObject::new();
        for (k, v) in value {
            obj.set(k.into(), v.into());
        }
        JsonValue::Object(obj)
    }
}
