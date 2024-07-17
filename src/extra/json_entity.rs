use crate::base::JsonValue;
use crate::extra::json_parser::ConversationError;

/// Trait for types that can be converted to and from JSON.
pub trait JsonEntity {
    /// Convert the struct implementing this trait to a JSON value.
    fn to_json(&self) -> JsonValue;

    /// Parse raw JSON string into an instance of the implementing struct.
    fn from_raw(raw: &str) -> Result<Self, ConversationError>
        where
            Self: Sized;
}
