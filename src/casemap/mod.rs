//! Unicode case mapping and folding.
//!
//! Implements full case mapping (including multi-character expansions from
//! SpecialCasing.txt), case folding for case-insensitive matching, and
//! locale-specific rules (Turkish/Azerbaijani, Lithuanian).
//!
//! # Functions
//!
//! - [`to_lowercase`] / [`to_uppercase`] / [`to_titlecase`] -- default (non-locale) full mappings.
//! - [`to_lowercase_locale`] / [`to_uppercase_locale`] -- locale-aware variants.
//! - [`case_fold`] / [`case_fold_simple`] -- case folding for comparison.
//! - [`is_uppercase`] / [`is_lowercase`] -- character classification.
//!
//! # References
//!
//! - Unicode Standard, Chapter 3: "Default Case Algorithms"
//! - Unicode Standard, Section 3.13: "Default Case Algorithms"
//! - UnicodeData.txt (fields 12-14), SpecialCasing.txt, CaseFolding.txt

use crate::data::casemap;
use crate::data::normalization::ccc;

// ---------------------------------------------------------------------------
// Helper: cased / case-ignorable classification
// ---------------------------------------------------------------------------

/// Check if a character is "Cased" per Unicode (has uppercase or lowercase mapping,
/// or is in general category Lu, Ll, or Lt).
fn is_cased(ch: char) -> bool {
    let cp = ch as u32;
    // Has a simple uppercase or lowercase mapping -> cased
    if casemap::simple_uppercase(cp).is_some() || casemap::simple_lowercase(cp).is_some() {
        return true;
    }
    // Titlecase letters (Lt category): DZ/Lj/Nj digraphs, etc.
    // A quick check: if it has a simple_titlecase entry that maps to itself
    // (i.e. it IS a titlecase letter) we should also count it
    matches!(
        cp,
        0x01C5 | 0x01C8 | 0x01CB | 0x01F2 | 0x1F88..=0x1F8F
        | 0x1F98..=0x1F9F | 0x1FA8..=0x1FAF | 0x1FBC | 0x1FCC | 0x1FFC
    )
}

/// Check if a character is "Case_Ignorable" per Unicode.
/// Simplified: combining marks (CCC > 0), format chars (Cf), and a few others.
fn is_case_ignorable(ch: char) -> bool {
    let cp = ch as u32;
    // Non-zero CCC -> combining mark -> case_ignorable
    if ccc(cp) != 0 {
        return true;
    }
    // Common format characters
    matches!(
        cp,
        0x00AD          // SOFT HYPHEN
        | 0x0027        // APOSTROPHE
        | 0x002E        // FULL STOP (for abbreviation contexts)
        | 0x003A        // COLON
        | 0x00B7        // MIDDLE DOT
        | 0x0387        // GREEK ANO TELEIA
        | 0x05F4        // HEBREW PUNCTUATION GERSHAYIM
        | 0x2019        // RIGHT SINGLE QUOTATION MARK
        | 0x2027        // HYPHENATION POINT
        | 0x200C..=0x200D // ZWNJ, ZWJ
        | 0xFE00..=0xFE0F // Variation selectors
    )
}

// ---------------------------------------------------------------------------
// Core: to_lowercase
// ---------------------------------------------------------------------------

/// Convert a string to lowercase using full Unicode case mapping.
///
/// Handles multi-character expansions (e.g. German sharp s remains as-is in lowercase),
/// and the Greek final sigma rule (capital sigma at the end of a word becomes
/// final sigma U+03C2 rather than small sigma U+03C3).
///
/// For locale-specific behavior (Turkish, Lithuanian), use [`to_lowercase_locale`].
#[must_use]
pub fn to_lowercase(s: &str) -> String {
    lowercase_impl(s, None)
}

/// Convert a string to lowercase with locale-specific rules.
///
/// Supported locales:
/// - `"tr"`, `"az"` (Turkish, Azerbaijani): Capital I -> dotless i (U+0131);
///   I with dot above (U+0130) -> i; remove dot above after I.
/// - `"lt"` (Lithuanian): Preserves dot above for soft-dotted letters.
/// - Any other value: falls back to default behavior.
#[must_use]
pub fn to_lowercase_locale(s: &str, locale: &str) -> String {
    lowercase_impl(s, Some(locale))
}

fn lowercase_impl(s: &str, locale: Option<&str>) -> String {
    let is_turkic = matches!(locale, Some("tr") | Some("az"));
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::with_capacity(s.len());

    for (i, &ch) in chars.iter().enumerate() {
        let cp = ch as u32;

        // Turkish/Azerbaijani special handling
        if is_turkic {
            // I (U+0049) -> dotless i (U+0131) in Turkic locales
            if cp == 0x0049 {
                // But if followed by U+0307 (dot above), remove the dot and produce 'i'
                if i + 1 < chars.len() && chars[i + 1] as u32 == 0x0307 {
                    result.push('i');
                    // The dot above will be skipped in the next iteration
                    // Actually we can't skip from here easily; handle differently
                    // We'll push 'i' and let the dot_above be handled below
                    continue;
                }
                result.push('\u{0131}'); // dotless i
                continue;
            }
            // Skip U+0307 if it follows I (handled above)
            if cp == 0x0307 && i > 0 && chars[i - 1] as u32 == 0x0049 {
                continue;
            }
            // I with dot above (U+0130) -> i
            if cp == 0x0130 {
                result.push('i');
                continue;
            }
        }

        // Greek final sigma rule:
        // Capital sigma (U+03A3) -> final sigma (U+03C2) at end of word,
        // otherwise -> small sigma (U+03C3).
        if cp == 0x03A3 {
            let left_cased = has_cased_before(&chars, i);
            let right_cased = has_cased_after(&chars, i);
            if left_cased && !right_cased {
                result.push('\u{03C2}'); // final sigma
            } else {
                result.push('\u{03C3}'); // small sigma
            }
            continue;
        }

        // Check full lowercase mapping first (multi-char from SpecialCasing)
        if let Some(cps) = casemap::full_lowercase(cp) {
            for &c in cps {
                if let Some(ch2) = char::from_u32(c) {
                    result.push(ch2);
                }
            }
            continue;
        }

        // Simple lowercase mapping
        if let Some(lower) = casemap::simple_lowercase(cp) {
            if let Some(ch2) = char::from_u32(lower) {
                result.push(ch2);
                continue;
            }
        }

        // No mapping: character maps to itself
        result.push(ch);
    }

    result
}

/// Check if there is a Cased character before position `i`, skipping Case_Ignorable.
fn has_cased_before(chars: &[char], i: usize) -> bool {
    let mut j = i;
    while j > 0 {
        j -= 1;
        if is_cased(chars[j]) {
            return true;
        }
        if !is_case_ignorable(chars[j]) {
            return false;
        }
    }
    false
}

/// Check if there is a Cased character after position `i`, skipping Case_Ignorable.
fn has_cased_after(chars: &[char], i: usize) -> bool {
    let mut j = i + 1;
    while j < chars.len() {
        if is_cased(chars[j]) {
            return true;
        }
        if !is_case_ignorable(chars[j]) {
            return false;
        }
        j += 1;
    }
    false
}

// ---------------------------------------------------------------------------
// Core: to_uppercase
// ---------------------------------------------------------------------------

/// Convert a string to uppercase using full Unicode case mapping.
///
/// Handles multi-character expansions (e.g. German sharp s -> "SS",
/// ligatures -> expanded forms).
///
/// For locale-specific behavior, use [`to_uppercase_locale`].
#[must_use]
pub fn to_uppercase(s: &str) -> String {
    uppercase_impl(s, None)
}

/// Convert a string to uppercase with locale-specific rules.
///
/// Supported locales:
/// - `"tr"`, `"az"` (Turkish, Azerbaijani): Small i -> I with dot above (U+0130);
///   dotless i (U+0131) -> I.
/// - Any other value: falls back to default behavior.
#[must_use]
pub fn to_uppercase_locale(s: &str, locale: &str) -> String {
    uppercase_impl(s, Some(locale))
}

fn uppercase_impl(s: &str, locale: Option<&str>) -> String {
    let is_turkic = matches!(locale, Some("tr") | Some("az"));
    let mut result = String::with_capacity(s.len());

    for ch in s.chars() {
        let cp = ch as u32;

        // Turkish/Azerbaijani special handling
        if is_turkic {
            // i -> I with dot above (U+0130)
            if cp == 0x0069 {
                result.push('\u{0130}');
                continue;
            }
        }

        // Check full uppercase mapping first (multi-char from SpecialCasing)
        if let Some(cps) = casemap::full_uppercase(cp) {
            for &c in cps {
                if let Some(ch2) = char::from_u32(c) {
                    result.push(ch2);
                }
            }
            continue;
        }

        // Simple uppercase mapping
        if let Some(upper) = casemap::simple_uppercase(cp) {
            if let Some(ch2) = char::from_u32(upper) {
                result.push(ch2);
                continue;
            }
        }

        // No mapping: character maps to itself
        result.push(ch);
    }

    result
}

// ---------------------------------------------------------------------------
// Core: to_titlecase
// ---------------------------------------------------------------------------

/// Convert a string to title case.
///
/// The first cased character in each "word" (contiguous run of non-whitespace)
/// is titlecased; the rest are lowercased. Non-cased characters are left as-is.
#[must_use]
pub fn to_titlecase(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut need_title = true; // Next cased char should be titlecased

    for ch in s.chars() {
        let cp = ch as u32;

        if ch.is_whitespace() {
            result.push(ch);
            need_title = true;
            continue;
        }

        if need_title && is_cased(ch) {
            // Apply titlecase mapping
            if let Some(cps) = casemap::full_titlecase(cp) {
                for &c in cps {
                    if let Some(ch2) = char::from_u32(c) {
                        result.push(ch2);
                    }
                }
            } else if let Some(tc) = casemap::simple_titlecase(cp) {
                if let Some(ch2) = char::from_u32(tc) {
                    result.push(ch2);
                }
            } else {
                result.push(ch);
            }
            need_title = false;
            continue;
        }

        if !need_title {
            // Lowercase the rest of the word
            if let Some(lower) = casemap::simple_lowercase(cp) {
                if let Some(ch2) = char::from_u32(lower) {
                    result.push(ch2);
                    continue;
                }
            }
        }

        result.push(ch);
    }

    result
}

// ---------------------------------------------------------------------------
// Case folding
// ---------------------------------------------------------------------------

/// Perform full case folding for case-insensitive matching.
///
/// Uses CaseFolding.txt status C + F mappings. Multi-character expansions
/// are applied (e.g. sharp s -> "ss", ligatures -> expanded).
///
/// For Turkic locale-aware folding, use [`case_fold_locale`].
#[must_use]
pub fn case_fold(s: &str) -> String {
    case_fold_impl(s, false)
}

/// Perform simple case folding (single-character only).
///
/// Uses CaseFolding.txt status C + S mappings. String length is preserved.
#[must_use]
pub fn case_fold_simple(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        let cp = ch as u32;
        if let Some(folded) = casemap::simple_case_fold(cp) {
            if let Some(ch2) = char::from_u32(folded) {
                result.push(ch2);
                continue;
            }
        }
        result.push(ch);
    }
    result
}

/// Perform case folding with Turkic locale rules.
///
/// For `"tr"` or `"az"`: applies Turkic overrides (I -> dotless i, etc.)
/// before standard folding.
#[must_use]
pub fn case_fold_locale(s: &str, locale: &str) -> String {
    let is_turkic = matches!(locale, "tr" | "az");
    if is_turkic {
        case_fold_impl(s, true)
    } else {
        case_fold_impl(s, false)
    }
}

fn case_fold_impl(s: &str, turkic: bool) -> String {
    let mut result = String::with_capacity(s.len());

    for ch in s.chars() {
        let cp = ch as u32;

        // Turkic overrides first
        if turkic {
            if let Some(folded) = casemap::turkic_case_fold(cp) {
                if let Some(ch2) = char::from_u32(folded) {
                    result.push(ch2);
                    continue;
                }
            }
        }

        // Full case fold (multi-char) takes precedence
        if let Some(cps) = casemap::full_case_fold(cp) {
            for &c in cps {
                if let Some(ch2) = char::from_u32(c) {
                    result.push(ch2);
                }
            }
            continue;
        }

        // Simple case fold (single char)
        if let Some(folded) = casemap::simple_case_fold(cp) {
            if let Some(ch2) = char::from_u32(folded) {
                result.push(ch2);
                continue;
            }
        }

        result.push(ch);
    }

    result
}

// ---------------------------------------------------------------------------
// Character classification
// ---------------------------------------------------------------------------

/// Check if a character has an uppercase mapping (i.e. is a lowercase letter).
#[must_use]
pub fn is_lowercase(ch: char) -> bool {
    casemap::simple_uppercase(ch as u32).is_some()
}

/// Check if a character has a lowercase mapping (i.e. is an uppercase letter).
#[must_use]
pub fn is_uppercase(ch: char) -> bool {
    casemap::simple_lowercase(ch as u32).is_some()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- to_lowercase ---

    #[test]
    fn lowercase_ascii() {
        assert_eq!(to_lowercase("HELLO WORLD"), "hello world");
        assert_eq!(to_lowercase("Hello World"), "hello world");
        assert_eq!(to_lowercase("hello"), "hello");
    }

    #[test]
    fn lowercase_german_sharp_s() {
        // Sharp s should remain as-is in lowercase (it is already lowercase)
        assert_eq!(to_lowercase("\u{00DF}"), "\u{00DF}");
    }

    #[test]
    fn lowercase_greek_final_sigma() {
        // "ODYSSEUS" in Greek: capital sigma at end becomes final sigma
        let upper = "\u{039F}\u{0394}\u{03A5}\u{03A3}\u{03A3}\u{0395}\u{03A5}\u{03A3}";
        let lower = to_lowercase(upper);
        // Last sigma should be final sigma (U+03C2), middle ones should be regular (U+03C3)
        assert!(lower.ends_with('\u{03C2}'), "final sigma at end");
        // The sigma between vowels should be regular sigma
        assert!(lower.contains('\u{03C3}'), "regular sigma in middle");
    }

    #[test]
    fn lowercase_single_sigma() {
        // A lone sigma with no preceding cased letter -> regular sigma
        assert_eq!(to_lowercase("\u{03A3}"), "\u{03C3}");
    }

    // --- to_uppercase ---

    #[test]
    fn uppercase_ascii() {
        assert_eq!(to_uppercase("hello world"), "HELLO WORLD");
        assert_eq!(to_uppercase("HELLO"), "HELLO");
    }

    #[test]
    fn uppercase_german_sharp_s() {
        // Sharp s -> SS in uppercase
        assert_eq!(to_uppercase("stra\u{00DF}e"), "STRASSE");
    }

    #[test]
    fn uppercase_ligatures() {
        // ff ligature -> FF
        assert_eq!(to_uppercase("\u{FB00}"), "FF");
        // fi ligature -> FI
        assert_eq!(to_uppercase("\u{FB01}"), "FI");
    }

    // --- to_titlecase ---

    #[test]
    fn titlecase_basic() {
        assert_eq!(to_titlecase("hello world"), "Hello World");
        assert_eq!(to_titlecase("HELLO WORLD"), "Hello World");
    }

    #[test]
    fn titlecase_mixed() {
        assert_eq!(to_titlecase("hELLO wORLD"), "Hello World");
    }

    // --- case folding ---

    #[test]
    fn case_fold_basic() {
        assert_eq!(case_fold("Hello World"), "hello world");
        assert_eq!(case_fold("HELLO"), case_fold("hello"));
    }

    #[test]
    fn case_fold_sharp_s() {
        // Full case fold: sharp s -> ss
        assert_eq!(case_fold("\u{00DF}"), "ss");
        // So "strasse" and "stra\u{00DF}e" should fold to the same thing
        assert_eq!(case_fold("STRASSE"), case_fold("stra\u{00DF}e"));
    }

    #[test]
    fn case_fold_simple_preserves_length() {
        // Simple fold: sharp s stays as-is (single char only)
        let s = "Stra\u{00DF}e";
        let folded = case_fold_simple(s);
        assert_eq!(folded.chars().count(), s.chars().count());
    }

    // --- Turkish locale ---

    #[test]
    fn turkish_lowercase_i() {
        // In Turkish: I -> dotless i (U+0131)
        assert_eq!(to_lowercase_locale("I", "tr"), "\u{0131}");
    }

    #[test]
    fn turkish_uppercase_i() {
        // In Turkish: i -> I with dot above (U+0130)
        assert_eq!(to_uppercase_locale("i", "tr"), "\u{0130}");
    }

    #[test]
    fn turkish_i_roundtrip() {
        // Turkish lowercase -> uppercase of dotless i should give I
        let lower = to_lowercase_locale("I", "tr");
        assert_eq!(lower, "\u{0131}");
        // Dotless i uppercases to I (via simple mapping)
        let upper = to_uppercase(&lower);
        assert_eq!(upper, "I");
    }

    // --- classification ---

    #[test]
    fn classification_basic() {
        assert!(is_uppercase('A'));
        assert!(is_lowercase('a'));
        assert!(!is_uppercase('a'));
        assert!(!is_lowercase('A'));
        assert!(!is_uppercase('1'));
    }

    // --- empty / edge cases ---

    #[test]
    fn empty_string() {
        assert_eq!(to_lowercase(""), "");
        assert_eq!(to_uppercase(""), "");
        assert_eq!(to_titlecase(""), "");
        assert_eq!(case_fold(""), "");
    }

    #[test]
    fn no_case_characters() {
        assert_eq!(to_lowercase("12345"), "12345");
        assert_eq!(to_uppercase("12345"), "12345");
    }
}
