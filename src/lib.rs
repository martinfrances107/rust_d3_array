#![allow(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! This crate contains helper functions for array manipulation, ordering, searching, summarizing, etc.
//!
//! <hr>
//!
//! Repository [rust_d3_array](<https://github.com/martinfrances107/rust_d3_array>)
//!
//! A port of [d3/d3-array](<https://github.com/d3/d3-array>)

extern crate num_traits;

/// Extent is the min and max values of a collection.
pub mod extent;
/// Helper functions for flatterning a collection.
pub mod merge;
/// Helper functions for generating a collection.
pub mod range;
