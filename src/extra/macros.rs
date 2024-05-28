#[macro_export]
macro_rules! json {
    // Match an entire JSON object with key-value pairs
    ({ $( $k:expr => $v:tt ),* }) => {{
        let mut obj = $crate::base::json_o::JsonObject::new();
        $(
            obj.set($k, json!($v));
        )*
        obj
    }};
    // Match an array of values
    ([ $( $elem:tt ),* ]) => {{
        let mut arr = $crate::base::json_arr::JsonArray::new();
        $(
            arr.push(json!($elem));
        )*
        arr
    }};
    // Match a string literal or other expressions
    ($other:expr) => {{
        $crate::base::json_v::JsonValue::from($other)
    }};
}