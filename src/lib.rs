//! A simple library for JSON serialization and manipulation.
//!
//! This library provides basic JSON functionality using macros and types defined
//! within the `rusty_json_serialization` crate. It is designed to be used for
//! straightforward JSON tasks in Rust projects.
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
pub mod base;
pub mod extra;

#[cfg(feature = "serialization")]
extern crate rusty_json_serialization;
