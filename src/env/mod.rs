//! This module provides environment variable utilities.
//! 
//! Features:
//! - `get_required`
//! - `get_or_default`
//! - `get_parsed`
//! - `get_parsed_or_default`
//! - `get_bool`
//! - `get_list`
//! - `parse_memory_size`
//!
//! Example:
//! ```
//! use common_utils_rs::env::*;
//! let val = get_or_default("HOST", "127.0.0.1");
//! ```
#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "env")]
pub use env::*;
