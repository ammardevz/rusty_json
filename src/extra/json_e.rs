use crate::base::json_o::JsonObject;

pub trait JsonEntity {
    fn to_json_object(&self) -> JsonObject;
}

impl Into<JsonObject> for &dyn JsonEntity {
    fn into(self) -> JsonObject {
        self.to_json_object()
    }
}