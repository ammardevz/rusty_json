#[macro_export]
macro_rules! json {
    // Match an entire JSON object with key-value pairs
    ({ $( $k:ident : $v:tt ),* }) => {{
        let mut obj = $crate::base::JsonObject::new();
        $(
            obj.set(stringify!($k), json!($v));
        )*
        obj
    }};
    // Match an array of values
    ([ $( $elem:tt ),* ]) => {{
        let mut arr = $crate::base::JsonArray::new();
        $(
            arr.push(json!($elem));
        )*
        arr
    }};
    // Match a string literal or other expressions
    ($other:expr) => {{
        $crate::base::JsonValue::from($other)
    }};
}
