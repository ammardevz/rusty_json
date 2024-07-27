/// A simple library for JSON serialization and manipulation.
///
/// This library provides basic JSON functionality using macros and types defined
/// within the `rusty_json_serialization` crate. It is designed to be used for
/// straightforward JSON tasks in Rust projects.
///
/// # Features
/// - Serialization
///
/// # Usage
/// ## Example
///
/// ```rust
/// // Import the macro for creating JSON objects.
/// use rusty_json::json;
///
/// fn main() {
///     // Create a JSON object using the `json!` macro.
///     let me = json!({
///         name: "Ammar Dev",
///         age: null,        // Use `null` for None in JSON
///         hobbies: ["Coding"]
///     });
///
///     // Print the JSON object
///     println!("{:?}", me);
/// }
/// ```
///

// Conditional compilation for serialization feature
#[cfg(feature = "serialization")]
extern crate rusty_json_serialization;

// Public modules for library functionality
pub mod base;
pub mod extra;
