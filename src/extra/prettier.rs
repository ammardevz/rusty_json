#![allow(dead_code)]

use std::error::Error;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::base::json_arr::JsonArray;
use crate::base::json_o::JsonObject;
use crate::base::json_v::JsonValue;

pub enum PrettierAble {
    Object(JsonObject),
    Array(JsonArray),
}

impl From<JsonObject> for PrettierAble {
    fn from(value: JsonObject) -> Self {
        PrettierAble::Object(value)
    }
}

impl From<JsonArray> for PrettierAble {
    fn from(value: JsonArray) -> Self {
        PrettierAble::Array(value)
    }
}

lazy_static! {
    static ref INDENT_LEVEL: Mutex<usize> = Mutex::new(2);
}

pub fn set_indent(indent_level: usize) {
    *INDENT_LEVEL.lock().unwrap() = indent_level;
}

pub fn get_indent() -> usize {
    *INDENT_LEVEL.lock().unwrap()
}

pub fn pretty_string<V: Into<PrettierAble>>(v: V) -> Result<String, Box<dyn Error>> {
    match v.into() {
        PrettierAble::Object(o) => Ok(pretty_object(&o, get_indent(), false)),
        PrettierAble::Array(a) => Ok(pretty_array(&a, get_indent(), false))
    }
}

fn pretty_object(json_object: &JsonObject, indent_level: usize, is_child: bool) -> String {
    let mut s = String::new();
    let mut data = json_object.iter().peekable();
    s.push_str("{\n");
    while let Some((k, v)) = data.next() {
        s.push_str(" ".repeat(indent_level).as_str());
        s.push_str(format!("\"{}\": {}", k, format_value(v, indent_level * 2, true)).as_str());
        if data.peek().is_some() {
            s.push_str(",\n");
        }
    }
    s.push('\n');
    if is_child {
        s.push_str(" ".repeat(indent_level - (indent_level / 2)).as_str()); // Adjusted indentation for nested objects
    }

    s.push_str("}");
    s
}


fn pretty_array(json_array: &JsonArray, indent_level: usize, is_child: bool) -> String {
    let mut s = String::new();
    let mut data = json_array.iter().peekable();

    s.push_str("[\n");
    while let Some(json_value) = data.next() {
        s.push_str(" ".repeat(indent_level).as_str());
        s.push_str(format!("{}",format_value(json_value, indent_level * 2, true)).as_str());
        if data.peek().is_some() {
            s.push_str(",\n");
        }
    }
    s.push('\n');
    if is_child {
        s.push_str(" ".repeat(indent_level - (indent_level / 2)).as_str()); // Adjusted indentation for nested objects
    }

    s.push_str("]");

    s
}

fn format_value(json_value: &JsonValue, indent_level: usize, is_child: bool) -> String{
    match json_value {
        JsonValue::String(v) => format!("\"{}\"", v),
        JsonValue::Number(v) => format!("{}", v),
        JsonValue::Boolean(v) => format!("{}", v),
        JsonValue::Object(v) => pretty_object(v, indent_level, is_child),
        JsonValue::Array(v) => pretty_array(v, indent_level, is_child),
        JsonValue::Null => "null".to_string()
    }
}

