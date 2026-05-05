//! JavaScript/WASM bindings for UniWorld via wasm-bindgen.
//!
//! Build with: wasm-pack build --features wasm

use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Segmentation (UAX #29)
// ---------------------------------------------------------------------------

/// Return grapheme cluster boundary byte offsets.
#[wasm_bindgen]
pub fn grapheme_boundaries(text: &str) -> Vec<usize> {
    crate::segment::grapheme_boundaries(text)
}

/// Return word boundary byte offsets (UAX #29 only).
#[wasm_bindgen]
pub fn word_boundaries(text: &str) -> Vec<usize> {
    crate::segment::word_boundaries(text, None)
}

/// Return word boundary byte offsets with dictionary segmentation for
/// Thai/Lao/Khmer/Myanmar. Falls back to UAX #29 for other scripts.
/// For mixed text, applies dictionary segmentation to SA-script runs
/// and UAX #29 to everything else.
#[wasm_bindgen]
pub fn word_boundaries_with_dictionary(text: &str) -> Vec<usize> {
    use crate::linebreak::dictionary::{language_for_codepoint, segment_words};
    use std::collections::BTreeSet;

    // Start with UAX #29 boundaries
    let mut boundaries: BTreeSet<usize> = BTreeSet::new();
    for b in crate::segment::word_boundaries(text, None) {
        boundaries.insert(b);
    }

    // Find runs of SA-script characters and apply dictionary segmentation
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (byte_start, ch) = chars[i];
        if let Some(lang) = language_for_codepoint(ch as u32) {
            // Find the end of this SA-script run
            let mut j = i + 1;
            while j < chars.len() {
                let (_, ch2) = chars[j];
                if language_for_codepoint(ch2 as u32) == Some(lang) {
                    j += 1;
                } else {
                    break;
                }
            }
            let byte_end = if j < chars.len() { chars[j].0 } else { text.len() };
            let run = &text[byte_start..byte_end];
            let dict_bounds = segment_words(run, lang);
            for b in dict_bounds {
                boundaries.insert(byte_start + b);
            }
            boundaries.insert(byte_start);
            i = j;
        } else {
            i += 1;
        }
    }

    boundaries.into_iter().collect()
}

/// Return sentence boundary byte offsets.
#[wasm_bindgen]
pub fn sentence_boundaries(text: &str) -> Vec<usize> {
    crate::segment::sentence_boundaries(text, None)
}

// ---------------------------------------------------------------------------
// Normalization (UAX #15)
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub fn normalize_nfc(text: &str) -> String {
    crate::normalize::nfc(text)
}

#[wasm_bindgen]
pub fn normalize_nfd(text: &str) -> String {
    crate::normalize::nfd(text)
}

#[wasm_bindgen]
pub fn normalize_nfkc(text: &str) -> String {
    crate::normalize::nfkc(text)
}

#[wasm_bindgen]
pub fn normalize_nfkd(text: &str) -> String {
    crate::normalize::nfkd(text)
}

// ---------------------------------------------------------------------------
// Case Mapping
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub fn to_lowercase(text: &str) -> String {
    crate::casemap::to_lowercase(text)
}

#[wasm_bindgen]
pub fn to_uppercase(text: &str) -> String {
    crate::casemap::to_uppercase(text)
}

#[wasm_bindgen]
pub fn to_titlecase(text: &str) -> String {
    crate::casemap::to_titlecase(text)
}

#[wasm_bindgen]
pub fn case_fold(text: &str) -> String {
    crate::casemap::case_fold(text)
}

// ---------------------------------------------------------------------------
// Display Width and Truncation
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub fn display_width(text: &str) -> u32 {
    crate::width::display_width(text)
}

#[wasm_bindgen]
pub fn truncate_graphemes(text: &str, max_graphemes: usize) -> String {
    crate::truncate::truncate_graphemes(text, max_graphemes).to_owned()
}

#[wasm_bindgen]
pub fn truncate_display_width(text: &str, max_width: u32) -> String {
    crate::truncate::truncate_display_width(text, max_width)
}

// ---------------------------------------------------------------------------
// Cursor Navigation
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub fn move_right(text: &str, current: usize) -> usize {
    crate::cursor::move_right(text, current)
}

#[wasm_bindgen]
pub fn move_left(text: &str, current: usize) -> usize {
    crate::cursor::move_left(text, current)
}

// ---------------------------------------------------------------------------
// Bidi (UAX #9)
// ---------------------------------------------------------------------------

/// Return the resolved embedding level for each character in the text.
/// Result is a Vec<u8> with one entry per char (indexed by char position).
/// Level 0 = LTR, odd levels = RTL.
#[wasm_bindgen]
pub fn bidi_levels(text: &str) -> Vec<u8> {
    let info = crate::bidi::resolve(text, None);
    info.levels
}

/// Return the paragraph embedding level (0 = LTR, 1 = RTL).
#[wasm_bindgen]
pub fn bidi_paragraph_level(text: &str) -> u8 {
    let info = crate::bidi::resolve(text, None);
    info.paragraph_level
}

/// Return the visual reorder indices for the text.
/// reorder[visual_position] = logical_char_index.
/// Characters removed by X9 are omitted.
#[wasm_bindgen]
pub fn bidi_reorder(text: &str) -> Vec<usize> {
    let info = crate::bidi::resolve(text, None);
    info.reorder
}

/// Return visual cursor stop byte offsets in screen-left-to-right order.
/// The same byte offset may appear more than once at bidi boundaries.
/// Callers should navigate by index, not by searching for byte offsets.
#[wasm_bindgen]
pub fn visual_cursor_stops(text: &str) -> Vec<u32> {
    crate::cursor::visual_cursor_stops(text)
        .iter()
        .map(|&s| s as u32)
        .collect()
}

/// Move cursor one grapheme cluster to the right in visual (screen) order.
/// `stop_hint` is the callers current stop index (pass 0xFFFFFFFF if unknown).
/// Returns a two-element array: [new_byte_offset, new_stop_index].
#[wasm_bindgen]
pub fn move_right_visual(text: &str, current: usize, stop_hint: u32) -> Vec<u32> {
    let (off, idx) = crate::cursor::move_right_visual_indexed(
        text,
        current,
        stop_hint as usize,
    );
    vec![off as u32, idx as u32]
}

/// Move cursor one grapheme cluster to the left in visual (screen) order.
/// `stop_hint` is the callers current stop index (pass 0xFFFFFFFF if unknown).
/// Returns a two-element array: [new_byte_offset, new_stop_index].
#[wasm_bindgen]
pub fn move_left_visual(text: &str, current: usize, stop_hint: u32) -> Vec<u32> {
    let (off, idx) = crate::cursor::move_left_visual_indexed(
        text,
        current,
        stop_hint as usize,
    );
    vec![off as u32, idx as u32]
}

// ---------------------------------------------------------------------------
// Line Breaking (UAX #14)
// ---------------------------------------------------------------------------

/// Return byte offsets where line breaks are allowed or mandatory.
/// Returns a flat array of pairs: [offset, action, offset, action, ...].
/// action: 0 = Mandatory, 1 = Allowed, 2 = Prohibited.
/// Only Mandatory and Allowed entries are included (Prohibited are omitted
/// for compactness).
#[wasm_bindgen]
pub fn line_break_opportunities(text: &str) -> Vec<u32> {
    let actions = crate::linebreak::line_break_opportunities_with_dictionary(text);
    let mut result = Vec::new();
    for (i, action) in actions.iter().enumerate() {
        let code = match action {
            crate::linebreak::BreakAction::Mandatory => 0u32,
            crate::linebreak::BreakAction::Allowed => 1u32,
            crate::linebreak::BreakAction::Prohibited => continue,
        };
        result.push(i as u32);
        result.push(code);
    }
    result
}
