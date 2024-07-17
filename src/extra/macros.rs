/// Creates a `JsonValue` from a JSON-like syntax.
///
/// This macro allows you to create JSON objects, arrays, and values using a syntax
/// that closely resembles JSON. It supports nested structures and automatically
/// converts Rust types to their JSON equivalents.
///
/// # Examples
///
/// Creating a JSON object:
/// ```
/// use rusty_json::base::JsonValue;
/// use rusty_json::json;
///
/// let person = json!({
///     name: "Alice",
///     age: 30,
///     is_student: false,
///     hobbies: ["reading", "cycling"],
///     address: null
/// });
///
/// assert_eq!(person["name"].parse::<String>().unwrap(), "Alice");
/// assert_eq!(person["age"].parse::<i64>().unwrap(), 30);
/// assert_eq!(person["is_student"].parse::<bool>().unwrap(), false);
/// assert_eq!(person["hobbies"][0].parse::<String>().unwrap(), "reading");
/// assert_eq!(person["address"], JsonValue::Null);
/// ```
///
/// Creating a JSON array:
/// ```
/// use rusty_json::base::JsonValue;
/// use rusty_json::json;
///
/// let numbers = json!([1, 2, null, 4, 5]);
///
/// assert_eq!(numbers[0].parse::<i64>().unwrap(), 1);
/// assert_eq!(numbers[2], JsonValue::Null);
/// assert_eq!(numbers[4].parse::<i64>().unwrap(), 5);
/// ```
///
/// Creating a simple JSON value:
/// ```
/// use rusty_json::json;
///
/// let string_value = json!("Hello, World!");
/// let number_value = json!(42);
/// let bool_value = json!(true);
/// let null_value = json!(null);
///
/// assert_eq!(string_value.parse::<String>().unwrap(), "Hello, World!");
/// assert_eq!(number_value.parse::<i64>().unwrap(), 42);
/// assert_eq!(bool_value.parse::<bool>().unwrap(), true);
/// assert_eq!(null_value.parse::<Option<String>>().unwrap(), None);
/// ```
///
/// # Note
///
/// This macro uses `$crate` to refer to the current crate. Ensure that the
/// necessary types (`JsonObject`, `JsonArray`, `JsonValue`) are available
/// in your crate's base module or adjust the paths accordingly.
#[macro_export]
macro_rules! json {
    // Match an entire JSON object with key-value pairs
    ({ $( $k:ident : $v:tt ),* }) => {{
        let mut obj = $crate::base::JsonObject::new();
        $(
            obj.set(stringify!($k), json!($v));
        )*
        $crate::base::JsonValue::Object(obj)
    }};

    // Match an array of values
    ([ $( $elem:tt ),* ]) => {{
        let mut arr = $crate::base::JsonArray::new();
        $(
            arr.push(json!($elem));
        )*
        $crate::base::JsonValue::Array(arr)
    }};

    // Match a null value
    (null) => {{
        $crate::base::JsonValue::Null
    }};

    // Match a string literal or other expressions
    ($other:expr) => {{
        $crate::base::JsonValue::from($other)
    }};
}
