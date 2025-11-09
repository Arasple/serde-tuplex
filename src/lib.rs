//! Tuple serialization and lenient numeric parsing for serde.
//!
//! # Features
//!
//! - [`Tuple`] - Serialize/deserialize as `[1,2]` instead of `{"x":1,"y":2}`
//! - [`Lenient`] - Accept both `123` and `"123"` for numeric fields
//! - [`TupleLenient`] - Combine both
//!
//! # Examples
//!
//! ```
//! use serde_tuplex::{Lenient, Tuple, TupleLenient};
//!
//! #[derive(Lenient)]
//! struct User {
//!     id: u64,        // Accepts 100 or "100"
//!     name: String,
//! }
//!
//! #[derive(Tuple)]
//! struct Point { x: i32, y: i32 }  // JSON: [10,20]
//!
//! #[derive(TupleLenient)]
//! struct Data { count: u64 }  // Both features
//! ```
//!
//! ## Manual control
//!
//! ```
//! use serde::Deserialize;
//! use serde_tuplex::{lenient, lenient_option};
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     #[serde(deserialize_with = "lenient")]
//!     timeout: u64,
//!     
//!     // For Option<T>, add 'default' to handle missing fields
//!     #[serde(deserialize_with = "lenient_option", default)]
//!     retry: Option<u32>,
//!     
//!     port: u16,  // Strict
//! }
//! ```
//!
//! Use `#[serde_tuplex(skip)]` to disable lenient parsing:
//!
//! ```
//! use serde_tuplex::Lenient;
//!
//! #[derive(Lenient)]
//! struct Mixed {
//!     lenient: u64,
//!     #[serde_tuplex(skip)]
//!     strict: u32,
//! }
//! ```

mod de;
mod internal;

pub use de::{lenient, lenient_option};
pub use serde_tuplex_derive::{Lenient, Tuple, TupleLenient};

#[doc(hidden)]
pub mod __private {
    pub use crate::internal::{LenientValue, OptionalLenientValue};
}
