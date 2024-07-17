use thiserror::Error;
use crate::base::json_value;

use crate::base::json_value::JsonValue;

/// Errors that can occur during casting operations.
#[derive(Debug, Error)]
pub enum CastError {
    /// Indicates that the type for casting is invalid.
    #[error("Invalid type for cast")]
    InvalidType,

    /// Indicates that the value is out of the acceptable range for casting.
    #[error("Value out of range")]
    OutOfRange,

    /// Indicates that a required field was not found in the JSON structure.
    #[error("Field not found: {0}")]
    FieldNotFound(String),
}


impl TryFrom<JsonValue> for String {
    type Error = CastError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(string) => Ok(string),
            JsonValue::Number(num) => Ok(num.to_string()),
            JsonValue::Null => Ok("null".to_string()),
            JsonValue::Boolean(bool) => Ok(bool.to_string()),
            JsonValue::Object(object) => Ok(object.to_string()),
            JsonValue::Array(array) => Ok(array.to_string())
        }
    }
}

impl TryFrom<&JsonValue> for String {
    type Error = CastError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::String(string) => Ok(string.to_string()),
            JsonValue::Number(num) => Ok(num.to_string()),
            JsonValue::Null => Ok("null".to_string()),
            JsonValue::Boolean(bool) => Ok(bool.to_string()),
            JsonValue::Object(object) => Ok(object.to_string()),
            JsonValue::Array(array) => Ok(array.to_string())
        }
    }
}

impl TryFrom<JsonValue> for bool {
    type Error = CastError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Boolean(bool) => Ok(bool),
            _ => Err(CastError::InvalidType)
        }
    }
}

impl TryFrom<&JsonValue> for bool {
    type Error = CastError;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Boolean(bool) => Ok(*bool),
            _ => Err(CastError::InvalidType)
        }
    }
}

/// Implement TryFrom<&JsonValue> for Vec<T> where T: TryFrom<&JsonValue>
impl<T> TryFrom<&JsonValue> for Vec<T>
    where
        T: for<'a> TryFrom<&'a JsonValue, Error = CastError>,
{
    type Error = CastError;

    fn try_from(json_value: &JsonValue) -> Result<Self, Self::Error> {
        match json_value {
            JsonValue::Array(arr) => {
                arr.iter()
                    .map(|item| T::try_from(item))
                    .collect::<Result<Vec<T>, _>>()
            }
            _ => Err(CastError::InvalidType),
        }
    }
}

/// Implement TryFrom<JsonValue> for Vec<T> where T: Into<JsonValue>
/// Implement TryFrom<JsonValue> for Vec<T> where T: Into<JsonValue>
impl<T> TryFrom<JsonValue> for Vec<T>
    where
        T: TryFrom<JsonValue, Error = CastError>,
{
    type Error = CastError;

    fn try_from(json_value: JsonValue) -> Result<Self, Self::Error> {
        match json_value {
            JsonValue::Array(arr) => {
                arr.iter().into_iter()
                    .map(|value: &JsonValue| T::try_from(value.clone()))
                    .collect::<Result<Vec<T>, _>>()
            }
            _ => Err(CastError::InvalidType),
        }
    }
}


/// Registering TryFrom .... for integers
macro_rules! register_tfi {
    ($type:ty) => {
        impl TryFrom<JsonValue> for $type {
            type Error = CastError;

            fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
                match value {
                    JsonValue::Number(num) => {
                        if num.fract() != 0.0 {
                            return Err(CastError::InvalidType)
                        }

                        let int_value = num as i64;
                        if int_value < Self::MIN as i64 || int_value > Self::MAX as i64 {
                            Err(CastError::OutOfRange)
                        }else {
                            Ok(int_value as Self)
                        }
                    },
                    _ => Err(CastError::InvalidType)
                }
            }
        }

        impl TryFrom<&JsonValue> for $type {
            type Error = CastError;

            fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
                match value {
                    JsonValue::Number(num) => {
                        if num.fract() != 0.0 {
                            return Err(CastError::InvalidType)
                        }

                        let int_value = *num as i64;
                        if int_value < Self::MIN as i64 || int_value > Self::MAX as i64 {
                            Err(CastError::OutOfRange)
                        }else {
                            Ok(int_value as Self)
                        }
                    },
                    _ => Err(CastError::InvalidType)
                }
            }
        }
    };
}

/// Registering TryFrom .... for floats
macro_rules! register_tff {
    ($type:ty) => {
        impl TryFrom<JsonValue> for $type {
            type Error = CastError;

            fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
                match value {
                    JsonValue::Number(num) => Ok(num as Self),
                    _ => Err(CastError::InvalidType)
                }
            }
        }

        impl TryFrom<&JsonValue> for $type {
            type Error = CastError;

            fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
                match value {
                    JsonValue::Number(num) => Ok(*num as Self),
                    _ => Err(CastError::InvalidType)
                }
            }
        }
    };
}

/// Registering TryFrom .... for unsigned integers
macro_rules! register_tfu {
    ($type:ty) => {
        impl TryFrom<JsonValue> for $type {
            type Error = CastError;

            fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
                match value {
                    JsonValue::Number(num) => {
                        if num.fract() != 0.0 {
                            return Err(CastError::InvalidType);
                        }

                        let int_value = num as u64;
                        if int_value > Self::MAX as u64 {
                            Err(CastError::OutOfRange)
                        } else {
                            Ok(int_value as Self)
                        }
                    }
                    _ => Err(CastError::InvalidType),
                }
            }
        }

        impl TryFrom<&JsonValue> for $type {
            type Error = CastError;

            fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
                match value {
                    JsonValue::Number(num) => {
                        if num.fract() != 0.0 {
                            return Err(CastError::InvalidType);
                        }

                        let int_value = *num as u64;
                        if int_value > Self::MAX as u64 {
                            Err(CastError::OutOfRange)
                        } else {
                            Ok(int_value as Self)
                        }
                    }
                    _ => Err(CastError::InvalidType),
                }
            }
        }
    };
}

register_tfi!(i8);
register_tfi!(i16);
register_tfi!(i32);
register_tfi!(i64);
register_tfi!(i128);
register_tfi!(isize);

register_tfu!(u8);
register_tfu!(u16);
register_tfu!(u32);
register_tfu!(u64);
register_tfu!(u128);
register_tfu!(usize);

register_tff!(f32);
register_tff!(f64);