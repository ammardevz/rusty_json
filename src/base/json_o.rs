#![allow(dead_code)]

use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::base::json_v::JsonValue;

#[derive()]
pub struct JsonObject (HashMap<String, JsonValue>);

impl JsonObject {
    pub fn new() -> Self {
        JsonObject(HashMap::new())
    }

    pub fn set<K: Into<String>, V: Into<JsonValue>>(&mut self,k: K, v: V) {
        self.0.insert(k.into(), v.into());
    }

    pub fn get<K: Into<String>>(&self, k: K) -> Option<&JsonValue> {
        self.0.get(&k.into())
    }

    pub fn del<K: Into<String>>(&mut self, k: K) {
        self.0.remove(&k.into());
    }

    pub fn exists<K: Into<String>> (self, k: K) -> bool{
        self.0.get(&k.into()).is_some()
    }

    pub fn iter(&self) -> Iter<String, JsonValue> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

}

impl Display for JsonObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut data = self.0.iter().peekable();
        while let Some((k, v)) = data.next() {
            write!(f, "\"{}\": {}", k, v)?;
            if data.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}
