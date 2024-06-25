#![allow(dead_code)]

use crate::base::JsonArray;
use crate::base::JsonObject;
use crate::base::JsonValue;

// pub enum JsonFormatType {
//     Array(JsonArray),
//     Object(JsonObject),
//     Value(JsonValue)
// }
//
// impl Into<JsonFormatType> for JsonObject {
//     fn into(self) -> JsonFormatType {
//         JsonFormatType::Object(self)
//     }
// }
//
// impl Into<JsonFormatType> for JsonArray {
//     fn into(self) -> JsonFormatType {
//         JsonFormatType::Array(self)
//     }
// }


pub struct JsonFormatter {
    indent: usize,
    indent_char: char
}

impl JsonFormatter {
    pub fn builder() -> JsonFormatterBuilder {
        JsonFormatterBuilder::new()
    }

    pub fn format<T>(&self, target: T) -> String
        where
            T: Into<JsonValue>,
    {
        self.format_value(&target.into(), self.indent, false)
    }

    fn pretty_object(&self, json_object: &JsonObject, indent_level: usize, is_child: bool) -> String {
        let mut s = String::new();
        let mut data = json_object.iter().peekable();
        s.push_str("{\n");
        while let Some((k, v)) = data.next() {
            s.push_str(self.indent_str().repeat(indent_level).as_str());
            s.push_str(format!("\"{}\": {}", k, self.format_value(v, indent_level * 2, true)).as_str());
            if data.peek().is_some() {
                s.push_str(",\n");
            }
        }
        s.push('\n');
        if is_child {
            s.push_str(self.indent_str().repeat(indent_level - (indent_level / 2)).as_str());
        }
        s.push_str("}");
        s
    }

    fn pretty_array(&self, json_array: &JsonArray, indent_level: usize, is_child: bool) -> String {
        let mut s = String::new();
        let mut data = json_array.iter().peekable();
        s.push_str("[\n");
        while let Some(json_value) = data.next() {
            s.push_str(self.indent_str().repeat(indent_level).as_str());
            s.push_str(self.format_value(json_value, indent_level * 2, true).as_str());
            if data.peek().is_some() {
                s.push_str(",\n");
            }
        }
        s.push('\n');
        if is_child {
            s.push_str(self.indent_str().repeat(indent_level - (indent_level / 2)).as_str());
        }
        s.push_str("]");
        s
    }

    fn format_value(&self, json_value: &JsonValue, indent_level: usize, is_child: bool) -> String {
        match json_value {
            JsonValue::String(v) => format!("\"{}\"", v),
            JsonValue::Number(v) => format!("{}", v),
            JsonValue::Boolean(v) => format!("{}", v),
            JsonValue::Object(v) => self.pretty_object(v, indent_level, is_child),
            JsonValue::Array(v) => self.pretty_array(v, indent_level, is_child),
            JsonValue::Null => "null".to_string(),
        }
    }

    fn indent_str(&self) -> String {
        self.indent_char.to_string()
    }
}

pub struct JsonFormatterBuilder {
    indent: usize,
    indent_char: char
}

impl JsonFormatterBuilder {
    pub fn new() -> Self {
        JsonFormatterBuilder {
            indent: 0,
            indent_char: '\0'
        }
    }

    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    pub fn with_indent_char(mut self, char: char) -> Self {
        self.indent_char = char;
        self
    }

    pub fn build(self) -> JsonFormatter {
        JsonFormatter {
            indent: self.indent,
            indent_char: self.indent_char
        }
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        JsonFormatterBuilder::new()
            .with_indent(2)
            .with_indent_char(' ')
            .build()
    }
}
