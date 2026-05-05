//! C FFI bindings for UniWorld.
//!
//! Provides a C-compatible API for use via P/Invoke (PowerShell/.NET) and
//! cbindgen-generated headers.
//!
//! Build with: cargo build --release (cdylib is in default crate-type)
//!
//! All strings are UTF-8 encoded. Returned strings must be freed with
//! `uniworld_free_string`. Returned arrays must be freed with
//! `uniworld_free_array`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Free a string previously returned by any `uniworld_*` function.
///
/// # Safety
/// `ptr` must be a valid pointer returned by a UniWorld function, or null.
#[no_mangle]
pub unsafe extern "C" fn uniworld_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

/// Free an array previously returned by a `uniworld_*` function.
///
/// # Safety
/// `ptr` must be a valid pointer returned by a UniWorld function, or null.
/// `len` must be the length returned alongside the pointer.
#[no_mangle]
pub unsafe extern "C" fn uniworld_free_array(ptr: *mut u32, len: u32) {
    if !ptr.is_null() && len > 0 {
        drop(Vec::from_raw_parts(ptr, len as usize, len as usize));
    }
}

/// Helper: convert C string to Rust &str, returning None for null/invalid UTF-8.
unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr).to_str().ok()
}

/// Helper: convert a Rust String to an owned C string. Caller must free with
/// `uniworld_free_string`.
fn string_to_cstring(s: String) -> *mut c_char {
    CString::new(s).map_or(std::ptr::null_mut(), |c| c.into_raw())
}

/// Helper: convert a Vec<usize> of byte offsets to a heap-allocated u32 array.
/// Writes the array length to `out_len`. Caller must free with `uniworld_free_array`.
fn offsets_to_array(offsets: Vec<usize>, out_len: &mut u32) -> *mut u32 {
    let arr: Vec<u32> = offsets.iter().map(|&o| o as u32).collect();
    *out_len = arr.len() as u32;
    let mut boxed = arr.into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}

// ---------------------------------------------------------------------------
// Normalization
// ---------------------------------------------------------------------------

/// Normalize to NFC. Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_normalize_nfc(text: *const c_char) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::normalize::nfc(s))
}

/// Normalize to NFD. Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_normalize_nfd(text: *const c_char) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::normalize::nfd(s))
}

/// Normalize to NFKC. Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_normalize_nfkc(text: *const c_char) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::normalize::nfkc(s))
}

/// Normalize to NFKD. Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_normalize_nfkd(text: *const c_char) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::normalize::nfkd(s))
}

// ---------------------------------------------------------------------------
// Case Mapping
// ---------------------------------------------------------------------------

/// Convert to lowercase. Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_to_lowercase(text: *const c_char) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::casemap::to_lowercase(s))
}

/// Convert to uppercase. Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_to_uppercase(text: *const c_char) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::casemap::to_uppercase(s))
}

// ---------------------------------------------------------------------------
// Display Width and Truncation
// ---------------------------------------------------------------------------

/// Compute display width of a UTF-8 string. Returns 0 for null input.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string, or null.
#[no_mangle]
pub unsafe extern "C" fn uniworld_display_width(text: *const c_char) -> u32 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return 0,
    };
    crate::width::display_width(s)
}

/// Truncate to max display columns without breaking grapheme clusters.
/// Caller must free the result with `uniworld_free_string`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_truncate_display_width(
    text: *const c_char,
    max_width: u32,
) -> *mut c_char {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    string_to_cstring(crate::truncate::truncate_display_width(s, max_width))
}

// ---------------------------------------------------------------------------
// Segmentation (UAX #29)
// ---------------------------------------------------------------------------

/// Get grapheme cluster boundary byte offsets.
/// Returns a heap-allocated u32 array; writes length to `out_len`.
/// Caller must free with `uniworld_free_array`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_grapheme_boundaries(
    text: *const c_char,
    out_len: *mut u32,
) -> *mut u32 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => {
            if !out_len.is_null() { *out_len = 0; }
            return std::ptr::null_mut();
        }
    };
    let offsets = crate::segment::grapheme_boundaries(s);
    if out_len.is_null() {
        return std::ptr::null_mut();
    }
    offsets_to_array(offsets, &mut *out_len)
}

/// Get word boundary byte offsets (UAX #29).
/// Returns a heap-allocated u32 array; writes length to `out_len`.
/// Caller must free with `uniworld_free_array`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_word_boundaries(
    text: *const c_char,
    out_len: *mut u32,
) -> *mut u32 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => {
            if !out_len.is_null() { *out_len = 0; }
            return std::ptr::null_mut();
        }
    };
    let offsets = crate::segment::word_boundaries(s, None);
    if out_len.is_null() {
        return std::ptr::null_mut();
    }
    offsets_to_array(offsets, &mut *out_len)
}

/// Get sentence boundary byte offsets (UAX #29).
/// Returns a heap-allocated u32 array; writes length to `out_len`.
/// Caller must free with `uniworld_free_array`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_sentence_boundaries(
    text: *const c_char,
    out_len: *mut u32,
) -> *mut u32 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => {
            if !out_len.is_null() { *out_len = 0; }
            return std::ptr::null_mut();
        }
    };
    let offsets = crate::segment::sentence_boundaries(s, None);
    if out_len.is_null() {
        return std::ptr::null_mut();
    }
    offsets_to_array(offsets, &mut *out_len)
}

// ---------------------------------------------------------------------------
// Bidi (UAX #9)
// ---------------------------------------------------------------------------

/// Get resolved bidi embedding levels (one per character/code point).
/// Returns a heap-allocated u8 array; writes length to `out_len`.
/// Caller must free with `uniworld_free_array` (cast to *mut u32 with byte len).
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_bidi_levels(
    text: *const c_char,
    out_len: *mut u32,
) -> *mut u8 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => {
            if !out_len.is_null() { *out_len = 0; }
            return std::ptr::null_mut();
        }
    };
    let info = crate::bidi::resolve(s, None);
    let len = info.levels.len();
    if out_len.is_null() {
        return std::ptr::null_mut();
    }
    *out_len = len as u32;
    let mut boxed = info.levels.into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}

/// Free a u8 array returned by `uniworld_bidi_levels`.
///
/// # Safety
/// `ptr` must be a valid pointer returned by `uniworld_bidi_levels`, or null.
#[no_mangle]
pub unsafe extern "C" fn uniworld_free_u8_array(ptr: *mut u8, len: u32) {
    if !ptr.is_null() && len > 0 {
        drop(Vec::from_raw_parts(ptr, len as usize, len as usize));
    }
}

/// Get bidi paragraph level (0 = LTR, 1 = RTL).
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_bidi_paragraph_level(text: *const c_char) -> u8 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return 0,
    };
    let info = crate::bidi::resolve(s, None);
    info.paragraph_level
}

// ---------------------------------------------------------------------------
// Line Breaking (UAX #14)
// ---------------------------------------------------------------------------

/// Get line break opportunities as a flat array of (byte_offset, action) pairs.
/// action: 0 = Mandatory, 1 = Allowed. Prohibited entries are omitted.
/// Returns a heap-allocated u32 array of pairs; writes total length to `out_len`.
/// Caller must free with `uniworld_free_array`.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_line_break_opportunities(
    text: *const c_char,
    out_len: *mut u32,
) -> *mut u32 {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => {
            if !out_len.is_null() { *out_len = 0; }
            return std::ptr::null_mut();
        }
    };
    let breaks = crate::linebreak::line_break_opportunities_with_dictionary(s);
    let mut pairs: Vec<u32> = Vec::new();
    for (i, action) in breaks.iter().enumerate() {
        match action {
            crate::linebreak::BreakAction::Mandatory => {
                pairs.push(i as u32);
                pairs.push(0);
            }
            crate::linebreak::BreakAction::Allowed => {
                pairs.push(i as u32);
                pairs.push(1);
            }
            _ => {}
        }
    }
    if out_len.is_null() {
        return std::ptr::null_mut();
    }
    *out_len = pairs.len() as u32;
    let mut boxed = pairs.into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}

// ---------------------------------------------------------------------------
// Cursor
// ---------------------------------------------------------------------------

/// Move cursor one grapheme cluster right. Returns new byte offset.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_move_right(text: *const c_char, current: usize) -> usize {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return current,
    };
    crate::cursor::move_right(s, current)
}

/// Move cursor one grapheme cluster left. Returns new byte offset.
///
/// # Safety
/// `text` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn uniworld_move_left(text: *const c_char, current: usize) -> usize {
    let s = match cstr_to_str(text) {
        Some(s) => s,
        None => return current,
    };
    crate::cursor::move_left(s, current)
}
