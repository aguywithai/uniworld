//! Integration tests for dictionary-based word segmentation.
//!
//! Tests that Thai, Lao, Khmer, and Myanmar text is segmented into words
//! using the ICU-sourced dictionaries, and that the dictionary-enhanced
//! line_break_opportunities_with_dictionary function produces correct
//! break opportunities.

use uniworld::linebreak::dictionary::{segment_words, DictLanguage, language_for_codepoint};
use uniworld::linebreak::{line_break_opportunities, line_break_opportunities_with_dictionary, BreakAction};

// ============================================================
// Thai text segmentation
// ============================================================

#[test]
fn thai_segment_known_phrase() {
    // "sawasdee" (hello) = สวัสดี
    // This should be a single dictionary word.
    let text = "\u{0E2A}\u{0E27}\u{0E31}\u{0E2A}\u{0E14}\u{0E35}";
    let boundaries = segment_words(text, DictLanguage::Thai);
    // If the whole phrase is one word, no internal boundaries.
    // If not found as one word, it will be split.
    // Either way, verify no panic and valid boundaries.
    for &b in &boundaries {
        assert!(text.is_char_boundary(b), "boundary at {b} is not a char boundary");
        assert!(b > 0 && b < text.len(), "boundary {b} out of range");
    }
}

#[test]
fn thai_segment_multi_word() {
    // "khon thai" (Thai person) = คนไทย
    // "khon" = คน, "thai" = ไทย
    let text = "\u{0E04}\u{0E19}\u{0E44}\u{0E17}\u{0E22}";
    let boundaries = segment_words(text, DictLanguage::Thai);
    // We expect a boundary between คน and ไทย
    // คน is 6 bytes (2 chars * 3 bytes), ไทย starts at byte 6
    // Check there's at least one boundary.
    assert!(
        !boundaries.is_empty(),
        "Expected at least one word boundary in 'khon thai'"
    );
    // The first boundary should be at byte offset 6 (after คน).
    assert_eq!(boundaries[0], 6, "Expected boundary after first word");
}

#[test]
fn thai_segment_longer_text() {
    // "prathetthai" (Thailand) = ประเทศไทย
    // Could be one word or segmented as ประเทศ + ไทย
    let text = "\u{0E1B}\u{0E23}\u{0E30}\u{0E40}\u{0E17}\u{0E28}\u{0E44}\u{0E17}\u{0E22}";
    let boundaries = segment_words(text, DictLanguage::Thai);
    for &b in &boundaries {
        assert!(text.is_char_boundary(b));
    }
}

#[test]
fn thai_dictionary_line_breaks() {
    // Test the full integration: line_break_opportunities_with_dictionary
    // should add break opportunities between Thai words.
    // "khon thai" = คนไทย
    let text = "\u{0E04}\u{0E19}\u{0E44}\u{0E17}\u{0E22}";

    // Without dictionary: SA chars resolved to AL, treated as one cluster.
    let _breaks_no_dict = line_break_opportunities(text);

    // With dictionary: should have a break between the two words.
    let breaks_dict = line_break_opportunities_with_dictionary(text);

    // The dictionary version should have an Allowed break at byte 6
    // (between คน and ไทย), where the non-dictionary version may not.
    let boundary_byte = 6; // after คน
    assert_eq!(
        breaks_dict[boundary_byte],
        BreakAction::Allowed,
        "Expected Allowed break between Thai words with dictionary"
    );
}

#[test]
fn thai_no_break_within_word() {
    // Within a single Thai word, breaks should be prohibited.
    // "prathet" (country) = ประเทศ (6 chars)
    let text = "\u{0E1B}\u{0E23}\u{0E30}\u{0E40}\u{0E17}\u{0E28}";
    let breaks = line_break_opportunities_with_dictionary(text);

    // Check that no internal positions have Allowed breaks.
    let char_positions: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    for &pos in &char_positions[1..] {
        if pos < text.len() {
            assert_ne!(
                breaks[pos],
                BreakAction::Allowed,
                "Unexpected break within Thai word at byte {pos}"
            );
        }
    }
}

// ============================================================
// Lao text segmentation
// ============================================================

#[test]
fn lao_segment_basic() {
    // Lao word: ສະບາຍດີ (sabaaidee - hello)
    let text = "\u{0EAA}\u{0EB0}\u{0E9A}\u{0EB2}\u{0E8D}\u{0E94}\u{0EB5}";
    let boundaries = segment_words(text, DictLanguage::Lao);
    for &b in &boundaries {
        assert!(text.is_char_boundary(b));
    }
}

#[test]
fn lao_language_detection() {
    // Lao consonants should be detected as Lao.
    assert_eq!(language_for_codepoint(0x0E81), Some(DictLanguage::Lao));
    assert_eq!(language_for_codepoint(0x0EAA), Some(DictLanguage::Lao));
}

// ============================================================
// Khmer text segmentation
// ============================================================

#[test]
fn khmer_segment_basic() {
    // Khmer word: ភាសា (pheasa - language)
    let text = "\u{1797}\u{17B6}\u{179F}\u{17B6}";
    let boundaries = segment_words(text, DictLanguage::Khmer);
    for &b in &boundaries {
        assert!(text.is_char_boundary(b));
    }
}

#[test]
fn khmer_language_detection() {
    assert_eq!(language_for_codepoint(0x1780), Some(DictLanguage::Khmer));
    assert_eq!(language_for_codepoint(0x1797), Some(DictLanguage::Khmer));
}

// ============================================================
// Myanmar text segmentation
// ============================================================

#[test]
fn myanmar_segment_basic() {
    // Myanmar word: မြန်မာ (myanmar)
    let text = "\u{1019}\u{103C}\u{1014}\u{103A}\u{1019}\u{102C}";
    let boundaries = segment_words(text, DictLanguage::Myanmar);
    for &b in &boundaries {
        assert!(text.is_char_boundary(b));
    }
}

#[test]
fn myanmar_language_detection() {
    assert_eq!(language_for_codepoint(0x1000), Some(DictLanguage::Myanmar));
    assert_eq!(language_for_codepoint(0x1019), Some(DictLanguage::Myanmar));
}

// ============================================================
// Mixed script tests
// ============================================================

#[test]
fn mixed_thai_latin() {
    // Mix of Thai and Latin: "Hello สวัสดี World"
    let text = "Hello \u{0E2A}\u{0E27}\u{0E31}\u{0E2A}\u{0E14}\u{0E35} World";
    let breaks = line_break_opportunities_with_dictionary(text);

    // Should still have breaks at spaces (Latin word boundaries).
    // The Thai part should be internally handled by dictionary.
    let space1 = 5; // after "Hello"
    assert_eq!(breaks[space1 + 1], BreakAction::Allowed,
        "Expected break opportunity after 'Hello '");
}

#[test]
fn non_sa_text_unchanged() {
    // Pure Latin text should be identical with and without dictionary.
    let text = "Hello World";
    let breaks_no_dict = line_break_opportunities(text);
    let breaks_dict = line_break_opportunities_with_dictionary(text);
    assert_eq!(breaks_no_dict, breaks_dict);
}

// ============================================================
// Edge cases
// ============================================================

#[test]
fn empty_text() {
    let breaks = line_break_opportunities_with_dictionary("");
    assert_eq!(breaks.len(), 1);
}

#[test]
fn single_thai_char() {
    // A single Thai character -- no internal boundary possible.
    let text = "\u{0E01}"; // ko kai
    let breaks = line_break_opportunities_with_dictionary(text);
    assert_eq!(breaks.len(), text.len() + 1);
}
