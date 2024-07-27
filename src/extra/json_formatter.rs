#![allow(dead_code)]

use std::ops::Sub;
use crate::base::{JsonArray, JsonObject, JsonValue};

/// A formatter for JSON values that allows customization of indentation.
pub struct JsonFormatter {
    indent: usize,
    indent_char: char,
    current_indent: usize,
}

impl JsonFormatter {
    /// Returns a new `JsonFormatterBuilder` to construct a `JsonFormatter`.
    pub fn builder() -> JsonFormatterBuilder {
        JsonFormatterBuilder::new()
    }

    /// Formats a JSON value into a pretty-printed string.
    ///
    /// # Arguments
    ///
    /// * `target` - The JSON value to format.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the JSON value.
    pub fn format(&mut self, target: &JsonValue) -> String {
        self.current_indent = self.indent;
        self.format_value(target, self.current_indent, false)
    }

    /// Formats a JSON object into a pretty-printed string.
    ///
    /// # Arguments
    ///
    /// * `json_object` - The JSON object to format.
    /// * `indent_level` - The current indentation level.
    /// * `is_child` - Whether this object is a child of another object or array.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the JSON object.
    fn pretty_object(&mut self, json_object: &JsonObject, indent_level: usize, is_child: bool) -> String {
        let mut s = String::new();
        let mut data = json_object.iter().peekable();
        s.push_str("{\n");
        while let Some((k, v)) = data.next() {
            self.current_indent = indent_level + self.indent;
            s.push_str(&self.indent_str().repeat(indent_level));
            s.push_str(&format!("\"{}\": {}", k, self.format_value(v, self.current_indent, true)));
            if data.peek().is_some() {
                s.push_str(",\n");
            }
        }
        s.push('\n');
        if is_child {
            s.push_str(&self.indent_str().repeat(indent_level.sub(self.indent)));
        }
        s.push('}');
        s
    }

    /// Formats a JSON array into a pretty-printed string.
    ///
    /// # Arguments
    ///
    /// * `json_array` - The JSON array to format.
    /// * `indent_level` - The current indentation level.
    /// * `is_child` - Whether this array is a child of another object or array.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the JSON array.
    fn pretty_array(&mut self, json_array: &JsonArray, indent_level: usize, is_child: bool) -> String {
        let mut s = String::new();
        let mut data = json_array.iter().peekable();
        s.push_str("[\n");
        while let Some(json_value) = data.next() {
            self.current_indent = indent_level + self.indent;
            s.push_str(&self.indent_str().repeat(indent_level));
            s.push_str(&self.format_value(json_value, self.current_indent, true));
            if data.peek().is_some() {
                s.push_str(",\n");
            }
        }
        s.push('\n');
        if is_child {
            s.push_str(&self.indent_str().repeat(indent_level.sub(self.indent)));
        }
        s.push(']');
        s
    }

    /// Formats a JSON value into a string.
    ///
    /// # Arguments
    ///
    /// * `json_value` - The JSON value to format.
    /// * `indent_level` - The current indentation level.
    /// * `is_child` - Whether this value is a child of an object or array.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the JSON value.
    fn format_value(&mut self, json_value: &JsonValue, indent_level: usize, is_child: bool) -> String {
        match json_value {
            JsonValue::String(v) => format!("\"{}\"", v),
            JsonValue::Number(v) => v.to_string(),
            JsonValue::Boolean(v) => v.to_string(),
            JsonValue::Object(v) => self.pretty_object(v, indent_level, is_child),
            JsonValue::Array(v) => self.pretty_array(v, indent_level, is_child),
            JsonValue::Null => "null".to_string(),
        }
    }

    /// Returns the indentation string based on the formatter's settings.
    fn indent_str(&self) -> String {
        self.indent_char.to_string().repeat(self.indent)
    }
}

/// A builder for constructing `JsonFormatter` instances.
pub struct JsonFormatterBuilder {
    indent: usize,
    indent_char: char,
}

impl JsonFormatterBuilder {
    /// Creates a new `JsonFormatterBuilder` with default settings.
    pub fn new() -> Self {
        JsonFormatterBuilder {
            indent: 0,
            indent_char: '\0',
        }
    }

    /// Sets the indentation size.
    ///
    /// # Arguments
    ///
    /// * `indent` - The number of indent characters to use for each level.
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    /// Sets the indentation character.
    ///
    /// # Arguments
    ///
    /// * `char` - The character to use for indentation.
    pub fn with_indent_char(mut self, char: char) -> Self {
        self.indent_char = char;
        self
    }

    /// Builds and returns a `JsonFormatter` with the configured settings.
    pub fn build(self) -> JsonFormatter {
        JsonFormatter {
            indent: self.indent,
            indent_char: self.indent_char,
            current_indent: 0,
        }
    }
}

impl Default for JsonFormatter {
    /// Returns a `JsonFormatter` with default settings (2 spaces for indentation).
    fn default() -> Self {
        JsonFormatterBuilder::new()
            .with_indent(2)
            .with_indent_char(' ')
            .build()
    }
}
