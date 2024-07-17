use std::collections::{BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;
use std::slice::Iter;
use indexmap::IndexSet;

use crate::base::json_value::JsonValue;


/// Represents a JSON array containing `JsonValue` elements.
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
    /// Creates a new, empty `JsonArray`.
    pub fn new() -> Self {
        JsonArray {
            vec: Vec::new()
        }
    }

    /// Appends a `JsonValue` element to the end of the array.
    pub fn push<V>(&mut self, v: V)
        where
            V: Into<JsonValue>
    {
        self.vec.push(v.into())
    }

    /// Inserts a `JsonValue` element at the specified index in the array.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn insert<V>(&mut self, i: usize, v: V)
        where
            V: Into<JsonValue>
    {
        self.vec.insert(i, v.into());
    }

    /// Retrieves a reference to the `JsonValue` element at the specified index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, i: usize) -> Option<&JsonValue>{
        self.vec.get(i)
    }

    /// Retrieves a mutable reference to the `JsonValue` element at the specified index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get_mut(&mut self, i: usize) -> Option<&mut JsonValue>{
        self.vec.get_mut(i)
    }

    /// Removes the `JsonValue` element at the specified index from the array.
    pub fn del(&mut self, i: usize) {
        self.vec.remove(i);
    }

    /// Checks if the array contains the specified `JsonValue` element.
    pub fn contains<V>(&self, v: V) -> bool
        where
            V: Into<JsonValue>,
    {
        self.vec.contains(&v.into())
    }

    /// Returns an iterator over the `JsonValue` elements in the array.
    pub fn iter(&self) -> Iter<'_, JsonValue> {
        self.vec.iter()
    }

    /// Returns a vector containing all `JsonValue` elements in the array.
    pub fn values(&self) -> Vec<JsonValue> {
        self.vec.clone()
    }

    /// Returns the number of `JsonValue` elements in the array.
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Removes all `JsonValue` elements from the array, leaving it empty.
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    /// Creates a new `JsonArray` containing only the `JsonValue` elements that satisfy the predicate.
    ///
    /// # Parameters
    ///
    /// - `predicate`: A closure that takes a reference to a `JsonValue` and returns `true` to include the element
    ///                in the filtered array, or `false` to exclude it.
    ///
    /// # Returns
    ///
    /// A new `JsonArray` containing only the `JsonValue` elements that satisfy the predicate.
    pub fn filter<P>(&self, predicate: P) -> Self
        where
            P: Fn(&JsonValue) -> bool
    {
        let filtered_vec = self.vec.iter().filter(|v| predicate(v)).cloned().collect();

        JsonArray {
            vec: filtered_vec
        }
    }

    /// Transforms each `JsonValue` element in the array using the provided closure `mapper`.
    ///
    /// # Parameters
    ///
    /// - `mapper`: A closure that takes a reference to a `JsonValue` and returns a new `JsonValue`.
    ///
    /// # Returns
    ///
    /// A new `JsonArray` where each `JsonValue` element has been transformed according to the closure `mapper`.
    pub fn map<F>(&self, mapper: F) -> JsonArray
        where
            F: Fn(&JsonValue) -> JsonValue,
    {
        let mapped_vec: Vec<JsonValue> = self.vec.iter().map(|v| mapper(v)).collect();

        JsonArray {
            vec: mapped_vec,
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
