use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::Index;
use indexmap::IndexMap;
use indexmap::map::{IntoIter, Iter, Keys, Values};
use crate::base::json_value::JsonValue;

pub struct JsonObject {
    index_map: IndexMap<String, JsonValue>
}

impl Debug for JsonObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}
impl Clone for JsonObject {
    fn clone(&self) -> Self {
        let mut cloned_map = IndexMap::new();
        for (key, value) in &self.index_map {
            cloned_map.insert(key.clone(), value.clone());
        }
        JsonObject {
            index_map: cloned_map,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.index_map.clear();
        for (key, value) in &source.index_map {
            self.index_map.insert(key.clone(), value.clone());
        }
    }
}
impl PartialEq for JsonObject {
    fn eq(&self, other: &Self) -> bool {
        self.index_map.eq(&other.index_map)
    }
}

impl Index<&str> for JsonObject {
    type Output = JsonValue;

    fn index(&self, index: &str) -> &Self::Output {
        match self.get(index) {
            Some(value) => value,
            None => panic!("Key '{}' not found in JsonObject", index),
        }
    }
}
impl JsonObject {
    pub fn new() -> Self {
        JsonObject {
            index_map: IndexMap::new(),
        }
    }

    pub fn set<K, V>(&mut self, k: K, v: V)
    where K: Into<String>,
          V: Into<JsonValue>,
    {
        self.index_map.insert(k.into(), v.into());
    }

    pub fn get<K>(&self, k: K) -> Option<&JsonValue>
    where K: Into<String>,
    {
        self.index_map.get(&k.into())
    }

    pub fn get_mut<K>(&mut self, k: K) -> Option<&mut JsonValue>
    where K: Into<String>,
    {
        self.index_map.get_mut(&k.into())
    }

    pub fn del<K>(&mut self, k: K)
    where K: Into<String>,
    {
        self.index_map.swap_remove(&k.into());
    }

    pub fn contains_key<K>(&self, k: K) -> bool
    where K: Into<String>,
    {
        self.index_map.contains_key(&k.into())
    }

    pub fn contains_value<V>(&self, v: V) -> bool
    where V: Into<JsonValue>,
    {
        let value = v.into();
        self.index_map.values().any(|x| *x == value)
    }

    pub fn iter(&self) -> Iter<String, JsonValue> {
        self.index_map.iter()
    }

    pub fn keys(&self) -> Keys<String, JsonValue> {
        self.index_map.keys()
    }

    pub fn values(&self) -> Values<String, JsonValue> {
        self.index_map.values()
    }

    pub fn len(&self) -> usize {
        self.index_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index_map.is_empty()
    }

    pub fn clear(&mut self) {
        self.index_map.clear()
    }

    pub fn merge(&mut self, other: JsonObject) {
        for (key, value) in other.into_iter() {
            self.set(key, value);
        }
    }

    pub fn filter<P>(&self, predicate: P) -> Self
    where P: Fn(&String, &JsonValue) -> bool,
    {
        let filtered_map: IndexMap<String, JsonValue> =
            self.index_map.iter().filter(|(k, v)| predicate(k, v)).map(|(k, v)| (k.clone(), v.clone())).collect();

        JsonObject {
            index_map: filtered_map,
        }
    }

}

impl IntoIterator for JsonObject {
    type Item = (String, JsonValue);
    type IntoIter = IntoIter<String, JsonValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.index_map.into_iter()
    }
}

impl Display for JsonObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{{")?;
        let mut iter = self.index_map.iter().peekable();
        while let Some((k, v)) = iter.next() {
            write!(f, "\"{}\": {}", k, v)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl From<JsonValue> for JsonObject {
    fn from(value: JsonValue) -> Self {
        return match value {
            JsonValue::Object(object) => object,
            _ => JsonObject::new()
        }
    }
}

impl<K, V> From<HashMap<K, V>> for JsonObject
where K: Into<String>,
      V: Into<JsonValue>
{
    fn from(map: HashMap<K, V>) -> Self {
        let mut obj = JsonObject::new();
        for (k, v) in map {
            obj.set(k.into(), v.into());
        }
        obj
    }
}

impl<K, V> From<IndexMap<K, V>> for JsonObject
    where
        K: Into<String>,
        V: Into<JsonValue>
{
    fn from(map: IndexMap<K, V>) -> Self {
        let map = map.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        JsonObject {
            index_map: map
        }
    }
}
