use std::collections::{BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use std::slice::Iter;
use indexmap::IndexSet;

use crate::base::json_value::JsonValue;


pub struct JsonArray {
    vec: Vec<JsonValue>
}

impl Debug for JsonArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Clone for JsonArray {
    fn clone(&self) -> Self {
        JsonArray {
            vec: self.vec.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vec.clone_from(&source.vec);
    }
}
impl PartialEq for JsonArray {
    fn eq(&self, other: &Self) -> bool {
        self.vec.eq(&other.vec)
    }
}

impl Index<usize> for JsonArray {
    type Output = JsonValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            None => panic!("Key '{}' not found in JsonObject", index),
            Some(v) => v
        }
    }
}

impl JsonArray {
    pub fn new() -> Self {
        JsonArray {
            vec: Vec::new()
        }
    }

    pub fn push<V>(&mut self, v: V)
    where V: Into<JsonValue>
    {
        self.vec.push(v.into())
    }

    pub fn insert<V>(&mut self, i: usize, v: V)
    where V: Into<JsonValue>
    {
        self.vec.insert(i, v.into());
    }

    pub fn get(&self, i: usize) -> Option<&JsonValue>{
        self.vec.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut JsonValue>{
        self.vec.get_mut(i)
    }

    pub fn del(&mut self, i: usize) {
        self.vec.remove(i);
    }

    pub fn contains<V>(&self, v: V) -> bool
    where V: Into<JsonValue>,
    {
        self.vec.contains(&v.into())
    }

    pub fn iter(&self) -> Iter<JsonValue> {
        self.vec.iter()
    }

    pub fn values(&self) -> Vec<JsonValue> {
        self.vec.to_vec()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn filter<P>(&self, predicate: P) -> Self
    where P: Fn(&JsonValue) -> bool
    {
        let filtered_vec = self.vec.iter().filter(|v| predicate(v)).map(|v| v.clone()).collect();

        JsonArray {
            vec: filtered_vec
        }
    }

}

impl Display for JsonArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.vec.iter().peekable();

        while let Some(json) = iter.next() {
            write!(f, "{}", json)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl From<JsonValue> for JsonArray {
    fn from(value: JsonValue) -> Self {
        return match value {
            JsonValue::Array(arr) => arr,
            _ => JsonArray::new()
        }
    }
}

// Conversion from a Vec to JsonArray
impl<T> From<Vec<T>> for JsonArray
    where T: Into<JsonValue>
{
    fn from(vec: Vec<T>) -> Self {
        JsonArray {
            vec: vec.into_iter().map(Into::into).collect()
        }
    }
}

// Conversion from a fixed-size array to JsonArray
impl<T, const N: usize> From<[T; N]> for JsonArray
    where T: Into<JsonValue>
{
    fn from(arr: [T; N]) -> Self {
        JsonArray {
            vec: arr.into_iter().map(Into::into).collect()
        }
    }
}

// Conversion from a reference to Vec to JsonArray with Clone
impl<T> From<&Vec<T>> for JsonArray
    where T: Into<JsonValue> + Clone
{
    fn from(value: &Vec<T>) -> Self {
        JsonArray {
            vec: value.iter().cloned().map(Into::into).collect()
        }
    }
}

// Conversion from IndexSet to JsonArray
impl<T> From<IndexSet<T>> for JsonArray
    where T: Into<JsonValue>
{
    fn from(value: IndexSet<T>) -> Self {
        JsonArray {
            vec: value.into_iter().map(Into::into).collect()
        }
    }
}

// Conversion from HashSet to JsonArray
impl<T> From<HashSet<T>> for JsonArray
    where T: Into<JsonValue>
{
    fn from(value: HashSet<T>) -> Self {
        JsonArray {
            vec: value.into_iter().map(Into::into).collect()
        }
    }
}

// Conversion from BTreeSet to JsonArray
impl<T> From<BTreeSet<T>> for JsonArray
    where T: Into<JsonValue>
{
    fn from(value: BTreeSet<T>) -> Self {
        JsonArray {
            vec: value.into_iter().map(Into::into).collect()
        }
    }
}
