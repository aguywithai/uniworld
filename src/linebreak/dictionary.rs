//! Dictionary-based word segmentation for Southeast Asian scripts.
//!
//! Provides word boundary detection for Thai, Lao, Khmer, and Myanmar text
//! using longest-match dictionary lookup. These scripts (UAX #14 Line_Break
//! class SA) do not use spaces between words, requiring dictionary-based
//! analysis to find valid line break points.
//!
//! Dictionary data sourced from ICU (International Components for Unicode)
//! break iterator dictionaries, licensed under the Unicode License.

use std::collections::HashSet;
use std::sync::OnceLock;

/// Which script/language a dictionary covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictLanguage {
    Thai,
    Lao,
    Khmer,
    Myanmar,
}

// Embedded dictionary data (cleaned word lists, one word per line).
static THAI_DATA: &str = include_str!("../data/dictionaries/thai.dict");
static LAO_DATA: &str = include_str!("../data/dictionaries/lao.dict");
static KHMER_DATA: &str = include_str!("../data/dictionaries/khmer.dict");
static MYANMAR_DATA: &str = include_str!("../data/dictionaries/myanmar.dict");

// Lazily initialized dictionary sets.
static THAI_DICT: OnceLock<Dictionary> = OnceLock::new();
static LAO_DICT: OnceLock<Dictionary> = OnceLock::new();
static KHMER_DICT: OnceLock<Dictionary> = OnceLock::new();
static MYANMAR_DICT: OnceLock<Dictionary> = OnceLock::new();

/// A dictionary for one language: a set of words and the max word length.
struct Dictionary {
    words: HashSet<&'static str>,
    max_word_len: usize,
}

impl Dictionary {
    /// Build a dictionary from a newline-separated word list.
    fn from_data(data: &'static str) -> Self {
        let mut words = HashSet::new();
        let mut max_len: usize = 0;
        for line in data.lines() {
            let w = line.trim();
            if !w.is_empty() {
                let byte_len = w.len();
                if byte_len > max_len {
                    max_len = byte_len;
                }
                words.insert(w);
            }
        }
        Dictionary {
            words,
            max_word_len: max_len,
        }
    }

    /// Check if a word exists in the dictionary.
    fn contains(&self, word: &str) -> bool {
        self.words.contains(word)
    }
}

/// Get the dictionary for a language, initializing lazily on first use.
fn get_dict(lang: DictLanguage) -> &'static Dictionary {
    match lang {
        DictLanguage::Thai => THAI_DICT.get_or_init(|| Dictionary::from_data(THAI_DATA)),
        DictLanguage::Lao => LAO_DICT.get_or_init(|| Dictionary::from_data(LAO_DATA)),
        DictLanguage::Khmer => KHMER_DICT.get_or_init(|| Dictionary::from_data(KHMER_DATA)),
        DictLanguage::Myanmar => {
            MYANMAR_DICT.get_or_init(|| Dictionary::from_data(MYANMAR_DATA))
        }
    }
}

/// Determine which dictionary language a code point belongs to, if any.
/// Returns None for code points outside the SA script ranges.
pub fn language_for_codepoint(cp: u32) -> Option<DictLanguage> {
    match cp {
        // Thai: U+0E01-U+0E3A, U+0E40-U+0E4E, U+0E50-U+0E5B
        0x0E01..=0x0E3A | 0x0E40..=0x0E4E | 0x0E50..=0x0E5B => Some(DictLanguage::Thai),
        // Lao: U+0E81-U+0EDF (main block)
        0x0E81..=0x0EDF => Some(DictLanguage::Lao),
        // Myanmar: U+1000-U+109F, U+AA60-U+AA7F (Myanmar Extended-A)
        0x1000..=0x109F | 0xAA60..=0xAA7F => Some(DictLanguage::Myanmar),
        // Khmer: U+1780-U+17FF, U+19E0-U+19FF (Khmer Symbols)
        0x1780..=0x17FF | 0x19E0..=0x19FF => Some(DictLanguage::Khmer),
        // Tai Tham: U+1A20-U+1AAF -- treat as Lao-family (no separate dict)
        // New Tai Lue: U+1980-U+19DF
        // Tai Le: U+1950-U+197F
        // These don't have ICU dictionaries; fall through to None.
        _ => None,
    }
}

/// Segment a string slice into words using longest-match dictionary lookup.
///
/// Returns a vector of byte offsets where word boundaries occur.
/// Each boundary is at the START of a new word (i.e., break opportunities).
/// The first word starts at offset 0 (implicit), so offsets[0] is the start
/// of the second word, etc.
///
/// Uses a forward longest-match algorithm with backtracking:
/// 1. At each position, find the longest word in the dictionary
/// 2. If found, advance past the word and record the boundary
/// 3. If not found, advance by one character (unknown word handling)
pub fn segment_words(text: &str, lang: DictLanguage) -> Vec<usize> {
    let dict = get_dict(lang);
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut boundaries: Vec<usize> = Vec::new();
    let mut pos: usize = 0;

    while pos < len {
        // Try longest match first, decreasing length.
        let remaining = &text[pos..];
        let max_try = remaining.len().min(dict.max_word_len);
        let mut matched_len: usize = 0;

        // Try all valid UTF-8 boundaries from longest to shortest.
        let mut try_len = max_try;
        while try_len > 0 {
            // Ensure we're at a valid UTF-8 boundary.
            if remaining.is_char_boundary(try_len) && dict.contains(&remaining[..try_len]) {
                matched_len = try_len;
                break;
            }
            try_len -= 1;
        }

        if matched_len > 0 {
            // Found a word; advance past it.
            pos += matched_len;
            if pos < len {
                boundaries.push(pos);
            }
        } else {
            // No dictionary match. Advance by one character.
            // This handles unknown words and individual characters.
            let ch_len = remaining
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(1);
            pos += ch_len;
            if pos < len {
                boundaries.push(pos);
            }
        }
    }

    boundaries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thai_dict_loads() {
        let dict = get_dict(DictLanguage::Thai);
        assert!(dict.words.len() > 20000);
        // Common Thai words that should be in the dictionary.
        assert!(dict.contains("\u{0E01}\u{0E23}")); // "kr" (to)
    }

    #[test]
    fn test_lao_dict_loads() {
        let dict = get_dict(DictLanguage::Lao);
        assert!(dict.words.len() > 20000);
    }

    #[test]
    fn test_khmer_dict_loads() {
        let dict = get_dict(DictLanguage::Khmer);
        assert!(dict.words.len() > 50000);
    }

    #[test]
    fn test_myanmar_dict_loads() {
        let dict = get_dict(DictLanguage::Myanmar);
        assert!(dict.words.len() > 30000);
    }

    #[test]
    fn test_language_for_codepoint() {
        // Thai
        assert_eq!(language_for_codepoint(0x0E01), Some(DictLanguage::Thai));
        assert_eq!(language_for_codepoint(0x0E44), Some(DictLanguage::Thai));
        // Lao
        assert_eq!(language_for_codepoint(0x0E81), Some(DictLanguage::Lao));
        // Myanmar
        assert_eq!(language_for_codepoint(0x1000), Some(DictLanguage::Myanmar));
        // Khmer
        assert_eq!(language_for_codepoint(0x1780), Some(DictLanguage::Khmer));
        // Latin -- not SA
        assert_eq!(language_for_codepoint(0x0041), None);
    }

    #[test]
    fn test_segment_basic_thai() {
        // The word list should contain common Thai words.
        // This is a smoke test; real validation needs known Thai text.
        let _dict = get_dict(DictLanguage::Thai);
        // Just verify segmentation doesn't panic on Thai text.
        let text = "\u{0E2A}\u{0E27}\u{0E31}\u{0E2A}\u{0E14}\u{0E35}"; // "sawasdee" approx
        let _ = segment_words(text, DictLanguage::Thai);
    }
}
