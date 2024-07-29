//! A simple library for JSON serialization and manipulation.
//!
//! This library provides basic JSON functionality using macros and types defined
//! within the `rusty_json_serialization` crate. It is designed to be used for
//! straightforward JSON tasks in Rust projects.
//!
//! > **Note:** If you need a highly optimized and widely used JSON crate, consider using `serde_json`. While `Rusty Json` is designed to be simple and lightweight, it is still in its early stages and may not be as performant or feature-rich as `serde_json`.
//!
//! # Features
//! - Serialization
//!
//! # Usage
//!
//! ## Example
//!
//! ```
//! // Import the macro for creating JSON objects.
//! use rusty_json::json;
//!
//! fn main() {
//!     // Create a JSON object using the `json!` macro.
//!     let me = json!({
//!         name: "Ammar Dev",
//!         age: null,
//!         hobbies: ["Coding"]
//!     });
//!
//!     // Print the JSON object
//!     println!("{}", me);
//! }
//! ```
//!
//! The `json!` macro allows you to easily create JSON objects using Rust syntax,
//! making it convenient to work with JSON data in your applications.
//!
//! # Conditional Compilation
//!
//! Conditional compilation is used for enabling the serialization feature:
//!
//! ```
//! #[cfg(feature = "serialization")]
//! extern crate rusty_json_serialization;
//! ```
//!
//! This allows you to include or exclude functionality based on feature flags.
//!
//! # Public Modules
//!
//! - `base`: Contains base functionality for JSON operations.
//! - `extra`: Includes additional utilities and extensions.
//!
//! For more information, please refer to the module-level documentation in `base` and `extra`.

/// `base` module for JSON operations.
///
/// This module provides the core types and functionality needed for working
/// with JSON data. It includes the definitions for JSON values, objects, and
/// arrays, as well as utilities for manipulating these types.
///
/// # Types
/// - `JsonValue`: Represents a JSON value, which can be a string, number, boolean, null, object, or array.
/// - `JsonObject`: Represents a JSON object, which is a collection of key-value pairs.
/// - `JsonArray`: Represents a JSON array, which is an ordered list of values.
pub mod base;

/// `extra` module for additional JSON utilities.
///
/// This module includes extended functionality and utilities for working
/// with JSON data, such as parsing and conversion traits.
///
/// # Features
/// - `JsonParser`: Provides functionality to parse JSON strings into `JsonValue`.
/// - `JsonEntity`: Trait for types that can be converted to and from JSON.
/// - `ConversationError`: Error type for handling conversion errors.
pub mod extra;

#[cfg(feature = "serialization")]
extern crate rusty_json_serialization;
