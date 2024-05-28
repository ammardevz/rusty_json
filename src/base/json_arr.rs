#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::slice::Iter;

use crate::base::json_v::JsonValue;

pub struct JsonArray (Vec<JsonValue>);

impl JsonArray {
    pub fn new() -> Self {
        JsonArray(Vec::new())
    }

    pub fn push<V: Into<JsonValue>>(&mut self,v: V) {
        self.0.push(v.into());
    }

    pub fn get(&self, index: usize) -> Option<&JsonValue> {
        self.0.get(index)
    }

    pub fn exists(self, index: usize) -> bool{
        self.0.get(index).is_some()
    }

    pub fn iter(&self) -> Iter<'_, JsonValue> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for JsonArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut data = self.0.iter().peekable();
        while let Some(json_value) = data.next() {
            write!(f, "{}", json_value)?;
            if data.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}