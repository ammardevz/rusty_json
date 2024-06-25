use std::error::Error;
use jsonparser::JSONValue;
use crate::base::{JsonArray, JsonObject, JsonValue};

pub struct JsonParser;

#[allow(dead_code)]
impl JsonParser {
    pub fn parse(json_str: &str) -> Result<JsonValue, Box<dyn Error>> {
        let mut parsed_json = jsonparser::JSONParser::new(json_str);
        let custom_json_value = convert_to_custom_value(parsed_json.parse()?)?;
        Ok(custom_json_value)
    }
}

// Function to convert parsed JSON data to custom JsonValue enum
fn convert_to_custom_value(parsed_json: JSONValue) -> Result<JsonValue, Box<dyn Error>> {
    match parsed_json {
        JSONValue::Object(obj) => {
            let mut custom_obj = JsonObject::new();
            for (key, value) in obj.iter() {
                custom_obj.set(key, convert_to_custom_value(value.clone())?);
            }
            Ok(JsonValue::Object(custom_obj))
        }
        JSONValue::Array(arr) => {
            let mut custom_arr = JsonArray::new();
            for value in arr.iter() {
                custom_arr.push(convert_to_custom_value(value.clone())?);
            }
            Ok(JsonValue::Array(custom_arr))
        }
        JSONValue::String(s) => Ok(JsonValue::String(s)),
        JSONValue::Number(f) => Ok(JsonValue::Number(f)),
        JSONValue::Boolean(b) => Ok(JsonValue::Boolean(b)),
        JSONValue::Null => Ok(JsonValue::Null),
    }
}