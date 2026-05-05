//! UniWorld: correct Unicode text handling for every script.
//!
//! Implements Unicode algorithms: UAX #9 (bidi), UAX #14 (line breaking),
//! UAX #29 (segmentation), UAX #15 (normalization), plus composite operations
//! (cursor navigation, display width, safe truncation, case mapping).

pub mod bidi;
pub mod casemap;
pub mod data;
pub mod linebreak;
pub mod normalize;
pub mod segment;
pub mod cursor;
pub mod width;
pub mod truncate;

#[cfg(feature = "python")]
mod python_bindings;

#[cfg(feature = "wasm")]
mod wasm_bindings;

#[cfg(feature = "cffi")]
pub mod c_bindings;
