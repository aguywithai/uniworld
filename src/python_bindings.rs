//! Python bindings for UniWorld via PyO3.
//!
//! Exposes the core Unicode algorithms and composite operations as a Python
//! module. Build with `maturin build --features python`.

use pyo3::prelude::*;

// ---------------------------------------------------------------------------
// Segmentation (UAX #29)
// ---------------------------------------------------------------------------

/// Return grapheme cluster boundary byte offsets for a string.
#[pyfunction]
fn grapheme_boundaries(text: &str) -> Vec<usize> {
    crate::segment::grapheme_boundaries(text)
}

/// Return word boundary byte offsets for a string.
///
/// Args:
///     text: Input string.
///     locale: Optional locale string (currently unused, reserved for future).
#[pyfunction]
#[pyo3(signature = (text, locale=None))]
fn word_boundaries(text: &str, locale: Option<&str>) -> Vec<usize> {
    crate::segment::word_boundaries(text, locale)
}

/// Return sentence boundary byte offsets for a string.
#[pyfunction]
#[pyo3(signature = (text, locale=None))]
fn sentence_boundaries(text: &str, locale: Option<&str>) -> Vec<usize> {
    crate::segment::sentence_boundaries(text, locale)
}

// ---------------------------------------------------------------------------
// Normalization (UAX #15)
// ---------------------------------------------------------------------------

/// Normalize a string to NFC (Canonical Decomposition, followed by Canonical Composition).
#[pyfunction]
fn normalize_nfc(text: &str) -> String {
    crate::normalize::nfc(text)
}

/// Normalize a string to NFD (Canonical Decomposition).
#[pyfunction]
fn normalize_nfd(text: &str) -> String {
    crate::normalize::nfd(text)
}

/// Normalize a string to NFKC (Compatibility Decomposition, followed by Canonical Composition).
#[pyfunction]
fn normalize_nfkc(text: &str) -> String {
    crate::normalize::nfkc(text)
}

/// Normalize a string to NFKD (Compatibility Decomposition).
#[pyfunction]
fn normalize_nfkd(text: &str) -> String {
    crate::normalize::nfkd(text)
}

// ---------------------------------------------------------------------------
// Bidirectional Algorithm (UAX #9)
// ---------------------------------------------------------------------------

/// Resolve bidi levels for a paragraph string.
///
/// Returns a dict with:
///     paragraph_level: int (0=LTR, 1=RTL)
///     levels: list[int] (embedding level per character)
///     reorder: list[int] (visual order indices)
#[pyfunction]
fn bidi_resolve(py: Python<'_>, text: &str) -> PyResult<PyObject> {
    let info = crate::bidi::resolve(text, None);
    let dict = pyo3::types::PyDict::new(py);
    dict.set_item("paragraph_level", info.paragraph_level)?;
    dict.set_item("levels", info.levels)?;
    dict.set_item("reorder", info.reorder)?;
    Ok(dict.into())
}

// ---------------------------------------------------------------------------
// Line Breaking (UAX #14)
// ---------------------------------------------------------------------------

/// Return line break opportunities for a string.
///
/// Returns a list of (byte_offset, action) tuples where action is:
///   "mandatory" - hard line break
///   "allowed" - break opportunity
///   "prohibited" - no break permitted
///
/// Only returns entries where the action is "mandatory" or "allowed".
#[pyfunction]
fn line_break_opportunities(text: &str) -> Vec<(usize, String)> {
    let actions = crate::linebreak::line_break_opportunities(text);
    actions
        .iter()
        .enumerate()
        .filter_map(|(i, a)| match a {
            crate::linebreak::BreakAction::Mandatory => Some((i, "mandatory".to_owned())),
            crate::linebreak::BreakAction::Allowed => Some((i, "allowed".to_owned())),
            crate::linebreak::BreakAction::Prohibited => None,
        })
        .collect()
}

/// Return line break opportunity byte offsets (allowed or mandatory) for a string,
/// with dictionary segmentation for Southeast Asian scripts.
#[pyfunction]
fn line_break_opportunities_with_dictionary(text: &str) -> Vec<(usize, String)> {
    let actions = crate::linebreak::line_break_opportunities_with_dictionary(text);
    actions
        .iter()
        .enumerate()
        .filter_map(|(i, a)| match a {
            crate::linebreak::BreakAction::Mandatory => Some((i, "mandatory".to_owned())),
            crate::linebreak::BreakAction::Allowed => Some((i, "allowed".to_owned())),
            crate::linebreak::BreakAction::Prohibited => None,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Case Mapping
// ---------------------------------------------------------------------------

/// Convert string to lowercase (full Unicode mapping).
#[pyfunction]
#[pyo3(signature = (text, locale=None))]
fn to_lowercase(text: &str, locale: Option<&str>) -> String {
    match locale {
        Some(loc) => crate::casemap::to_lowercase_locale(text, loc),
        None => crate::casemap::to_lowercase(text),
    }
}

/// Convert string to uppercase (full Unicode mapping).
#[pyfunction]
#[pyo3(signature = (text, locale=None))]
fn to_uppercase(text: &str, locale: Option<&str>) -> String {
    match locale {
        Some(loc) => crate::casemap::to_uppercase_locale(text, loc),
        None => crate::casemap::to_uppercase(text),
    }
}

/// Convert string to title case.
#[pyfunction]
fn to_titlecase(text: &str) -> String {
    crate::casemap::to_titlecase(text)
}

/// Case fold for case-insensitive comparison (full mapping, strings may grow).
#[pyfunction]
fn case_fold(text: &str) -> String {
    crate::casemap::case_fold(text)
}

/// Simple case fold (single-char, length-preserving).
#[pyfunction]
fn case_fold_simple(text: &str) -> String {
    crate::casemap::case_fold_simple(text)
}

// ---------------------------------------------------------------------------
// Display Width
// ---------------------------------------------------------------------------

/// Compute display width of a string in column cells.
///
/// CJK/fullwidth chars = 2, combining marks = 0, most others = 1.
#[pyfunction]
fn display_width(text: &str) -> u32 {
    crate::width::display_width(text)
}

// ---------------------------------------------------------------------------
// Truncation
// ---------------------------------------------------------------------------

/// Truncate a string to at most max_graphemes grapheme clusters.
///
/// Never splits a grapheme cluster.
#[pyfunction]
fn truncate_graphemes(text: &str, max_graphemes: usize) -> String {
    crate::truncate::truncate_graphemes(text, max_graphemes).to_owned()
}

/// Truncate a string so its display width does not exceed max_width.
///
/// Never splits a grapheme cluster.
#[pyfunction]
fn truncate_display_width(text: &str, max_width: u32) -> String {
    crate::truncate::truncate_display_width(text, max_width)
}

// ---------------------------------------------------------------------------
// Cursor Navigation
// ---------------------------------------------------------------------------

/// Move cursor one grapheme cluster right (logical order).
#[pyfunction]
fn move_right(text: &str, current: usize) -> usize {
    crate::cursor::move_right(text, current)
}

/// Move cursor one grapheme cluster left (logical order).
#[pyfunction]
fn move_left(text: &str, current: usize) -> usize {
    crate::cursor::move_left(text, current)
}

/// Move cursor one grapheme cluster right (visual/bidi order).
#[pyfunction]
fn move_right_visual(text: &str, current: usize) -> usize {
    crate::cursor::move_right_visual(text, current)
}

/// Move cursor one grapheme cluster left (visual/bidi order).
#[pyfunction]
fn move_left_visual(text: &str, current: usize) -> usize {
    crate::cursor::move_left_visual(text, current)
}

/// Select the word at the given byte offset (double-click semantics).
///
/// Returns (start, end) byte offsets.
#[pyfunction]
fn select_word(text: &str, current: usize) -> (usize, usize) {
    crate::cursor::select_word(text, current)
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// UniWorld: correct Unicode text handling for every script.
///
/// This module provides Python bindings to the UniWorld Rust library,
/// implementing UAX #9 (bidi), UAX #14 (line breaking), UAX #15 (normalization),
/// UAX #29 (segmentation), plus case mapping, display width, cursor navigation,
/// and safe truncation.
#[pymodule]
fn uniworld(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Segmentation
    m.add_function(wrap_pyfunction!(grapheme_boundaries, m)?)?;
    m.add_function(wrap_pyfunction!(word_boundaries, m)?)?;
    m.add_function(wrap_pyfunction!(sentence_boundaries, m)?)?;

    // Normalization
    m.add_function(wrap_pyfunction!(normalize_nfc, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_nfd, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_nfkc, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_nfkd, m)?)?;

    // Bidi
    m.add_function(wrap_pyfunction!(bidi_resolve, m)?)?;

    // Line breaking
    m.add_function(wrap_pyfunction!(line_break_opportunities, m)?)?;
    m.add_function(wrap_pyfunction!(line_break_opportunities_with_dictionary, m)?)?;

    // Case mapping
    m.add_function(wrap_pyfunction!(to_lowercase, m)?)?;
    m.add_function(wrap_pyfunction!(to_uppercase, m)?)?;
    m.add_function(wrap_pyfunction!(to_titlecase, m)?)?;
    m.add_function(wrap_pyfunction!(case_fold, m)?)?;
    m.add_function(wrap_pyfunction!(case_fold_simple, m)?)?;

    // Display width
    m.add_function(wrap_pyfunction!(display_width, m)?)?;

    // Truncation
    m.add_function(wrap_pyfunction!(truncate_graphemes, m)?)?;
    m.add_function(wrap_pyfunction!(truncate_display_width, m)?)?;

    // Cursor navigation
    m.add_function(wrap_pyfunction!(move_right, m)?)?;
    m.add_function(wrap_pyfunction!(move_left, m)?)?;
    m.add_function(wrap_pyfunction!(move_right_visual, m)?)?;
    m.add_function(wrap_pyfunction!(move_left_visual, m)?)?;
    m.add_function(wrap_pyfunction!(select_word, m)?)?;

    Ok(())
}
