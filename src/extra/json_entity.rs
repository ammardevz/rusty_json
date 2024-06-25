use crate::base::JsonValue;

pub trait JsonEntity {
    fn to_json(&self) -> JsonValue;
}

