//! Integration tests for composite operations:
//! cursor, width, truncate, casemap across diverse scripts and edge cases.

// ==========================================================================
// CURSOR
// ==========================================================================

mod cursor_tests {
    use uniworld::cursor::{
        delete_backward, delete_forward, move_left, move_left_visual, move_right,
        move_right_visual, select_word,
    };

    #[test]
    fn cursor_devanagari_cluster() {
        // Devanagari: ka + vowel sign i (a single grapheme cluster)
        let s = "\u{0915}\u{093F}"; // one cluster
        assert_eq!(move_right(s, 0), s.len());
        assert_eq!(move_left(s, s.len()), 0);
    }

    #[test]
    fn cursor_thai_text() {
        // Thai text: multiple characters per grapheme in some cases
        let s = "\u{0E2A}\u{0E27}\u{0E31}\u{0E2A}\u{0E14}\u{0E35}";
        // Move right from 0 should land on a grapheme boundary (not mid-cluster)
        let pos = move_right(s, 0);
        assert!(pos > 0);
        assert!(s.is_char_boundary(pos));
    }

    #[test]
    fn cursor_emoji_flag_sequence() {
        // Regional indicator pair (flag): US
        let s = "\u{1F1FA}\u{1F1F8}"; // flag: US
        // Should be treated as one grapheme cluster
        assert_eq!(move_right(s, 0), s.len());
        assert_eq!(move_left(s, s.len()), 0);
    }

    #[test]
    fn cursor_emoji_skin_tone() {
        // Emoji with skin tone modifier
        let wave = "\u{1F44B}\u{1F3FD}"; // waving hand + medium skin tone
        assert_eq!(move_right(wave, 0), wave.len());
        assert_eq!(move_left(wave, wave.len()), 0);
    }

    #[test]
    fn cursor_mixed_script_word_select() {
        // English + space + Chinese
        let s = "hello \u{4F60}\u{597D}";
        let (start, end) = select_word(s, 2); // Inside "hello"
        assert_eq!(&s[start..end], "hello");
    }

    #[test]
    fn cursor_delete_backward_multibyte() {
        // Delete a CJK character (3 bytes in UTF-8)
        let s = "a\u{4E16}\u{754C}b"; // a + world (2 CJK chars) + b
        let cjk_end = "a\u{4E16}\u{754C}".len();
        let (result, pos) = delete_backward(s, cjk_end);
        // Should delete one CJK character (the second one)
        assert_eq!(result, "a\u{4E16}b");
        assert_eq!(pos, "a\u{4E16}".len());
    }

    #[test]
    fn cursor_delete_forward_emoji() {
        let s = "x\u{1F600}y"; // x + grinning face + y
        let (result, pos) = delete_forward(s, 1); // delete the emoji
        assert_eq!(result, "xy");
        assert_eq!(pos, 1);
    }

    #[test]
    fn visual_cursor_ltr_paragraph() {
        let s = "abc";
        // In pure LTR, visual == logical
        assert_eq!(move_right_visual(s, 0), move_right(s, 0));
        assert_eq!(move_left_visual(s, 3), move_left(s, 3));
    }

    #[test]
    fn visual_cursor_single_rtl_char() {
        // Single RTL character (Hebrew alef, 2 bytes).
        // Visual stops for RTL: [byte_end, byte_start] = [2, 0].
        // Byte 0 is the visual RIGHT edge; byte 2 is the visual LEFT edge.
        // move_right_visual from 0 (already rightmost) should stay at 0.
        // move_left_visual from 0 should move to 2 (visual left).
        // move_right_visual from 2 (leftmost) should move to 0 (visual right).
        let s = "\u{05D0}"; // Hebrew alef
        assert_eq!(move_right_visual(s, 0), 0);       // already at visual right edge
        assert_eq!(move_left_visual(s, 0), s.len());   // move left to visual left edge
        assert_eq!(move_right_visual(s, s.len()), 0);  // move right from left edge to right edge
        assert_eq!(move_left_visual(s, s.len()), s.len()); // already at visual left edge
    }
}

// ==========================================================================
// WIDTH
// ==========================================================================

mod width_tests {
    use uniworld::width::{char_width, display_width};

    #[test]
    fn width_cjk_ideographs() {
        // Common CJK ideographs should be width 2
        assert_eq!(char_width('\u{4E00}'), 2); // CJK Unified Ideograph (one)
        assert_eq!(char_width('\u{9FFF}'), 2); // CJK Unified Ideograph
        assert_eq!(display_width("\u{4F60}\u{597D}"), 4); // ni hao
    }

    #[test]
    fn width_hangul() {
        // Hangul syllables are full-width
        assert_eq!(char_width('\u{AC00}'), 2); // ga (first Hangul syllable)
        assert_eq!(display_width("\u{D55C}\u{AE00}"), 4); // hangul (2 syllables)
    }

    #[test]
    fn width_katakana_fullwidth() {
        assert_eq!(char_width('\u{30A2}'), 2); // Katakana A
    }

    #[test]
    fn width_combining_sequence() {
        // e + combining acute + combining tilde = width 1 (base only)
        let s = "e\u{0301}\u{0303}";
        assert_eq!(display_width(s), 1);
    }

    #[test]
    fn width_zero_width_joiner() {
        // ZWJ itself has width 0
        assert_eq!(char_width('\u{200D}'), 0);
    }

    #[test]
    fn width_mixed_script_string() {
        // "Hello" (5) + CJK char (2) = 7
        let s = "Hello\u{4E16}";
        assert_eq!(display_width(s), 7);
    }

    #[test]
    fn width_empty_and_controls() {
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("\n"), 0);
        assert_eq!(display_width("\r\n"), 0);
        assert_eq!(display_width("\t"), 0); // tab is a control char (< 0x20)
    }

    #[test]
    fn width_latin_with_diacritics() {
        // Precomposed: e with acute (single char, width 1)
        assert_eq!(char_width('\u{00E9}'), 1);
        // Decomposed: e + combining acute (width 1 + 0 = 1)
        assert_eq!(display_width("e\u{0301}"), 1);
    }
}

// ==========================================================================
// TRUNCATE
// ==========================================================================

mod truncate_tests {
    use uniworld::truncate::{truncate_display_width, truncate_graphemes};

    #[test]
    fn truncate_graphemes_emoji_sequence() {
        // Family emoji: one grapheme cluster
        let family = "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}";
        // Truncate to 1 grapheme should keep the whole cluster
        let t = truncate_graphemes(family, 1);
        assert_eq!(t, family);
        // Truncate to 0 graphemes = empty
        assert_eq!(truncate_graphemes(family, 0), "");
    }

    #[test]
    fn truncate_graphemes_devanagari() {
        // Devanagari: ka + vowel sign i is one grapheme
        let cluster = "\u{0915}\u{093F}";
        let s = format!("{}x", cluster);
        let t = truncate_graphemes(&s, 1);
        assert_eq!(t, cluster);
    }

    #[test]
    fn truncate_width_cjk_boundary() {
        // CJK chars are width 2 each
        let s = "\u{4E00}\u{4E8C}\u{4E09}"; // one two three
        // Max width 4 -> should fit 2 CJK chars (width 4)
        let t = truncate_display_width(s, 4);
        assert_eq!(t, "\u{4E00}\u{4E8C}");
        // Max width 3 -> can only fit 1 CJK char (width 2)
        let t2 = truncate_display_width(s, 3);
        assert_eq!(t2, "\u{4E00}");
    }

    #[test]
    fn truncate_width_mixed_ascii_cjk() {
        let s = "ab\u{4E00}c"; // a(1) + b(1) + CJK(2) + c(1) = 5
        let t = truncate_display_width(s, 4);
        assert_eq!(t, "ab\u{4E00}"); // width 4
        let t2 = truncate_display_width(s, 3);
        assert_eq!(t2, "ab"); // CJK won't fit in remaining 1 column
    }

    #[test]
    fn truncate_empty() {
        assert_eq!(truncate_graphemes("", 5), "");
        assert_eq!(truncate_display_width("", 5), "");
    }

    #[test]
    fn truncate_no_truncation_needed() {
        let s = "hello";
        assert_eq!(truncate_graphemes(s, 100), s);
        assert_eq!(truncate_display_width(s, 100), s);
    }

    #[test]
    fn truncate_regional_indicator_pair() {
        // Flag: US (two regional indicators = one grapheme cluster)
        let flag = "\u{1F1FA}\u{1F1F8}";
        let s = format!("{}abc", flag);
        // Truncate to 1 grapheme: keep just the flag
        let t = truncate_graphemes(&s, 1);
        assert_eq!(t, flag);
    }
}

// ==========================================================================
// CASEMAP
// ==========================================================================

mod casemap_tests {
    use uniworld::casemap::{
        case_fold, case_fold_locale, case_fold_simple, is_lowercase, is_uppercase, to_lowercase,
        to_lowercase_locale, to_titlecase, to_uppercase, to_uppercase_locale,
    };

    #[test]
    fn casemap_latin_extended() {
        // Latin small letter sharp s -> SS
        assert_eq!(to_uppercase("\u{00DF}"), "SS");
        // Latin capital letter I with dot above -> i + combining dot above (special)
        assert_eq!(to_lowercase("\u{0130}"), "i\u{0307}");
    }

    #[test]
    fn casemap_greek_uppercase() {
        // Greek lowercase -> uppercase
        let lower = "\u{03B1}\u{03B2}\u{03B3}"; // alpha beta gamma
        let upper = to_uppercase(lower);
        assert_eq!(upper, "\u{0391}\u{0392}\u{0393}");
    }

    #[test]
    fn casemap_cyrillic() {
        // Cyrillic: a few chars
        let lower = "\u{0430}\u{0431}\u{0432}"; // a, be, ve
        let upper = to_uppercase(lower);
        assert_eq!(upper, "\u{0410}\u{0411}\u{0412}"); // A, BE, VE
        assert_eq!(to_lowercase(&upper), lower);
    }

    #[test]
    fn casemap_turkish_roundtrip() {
        // Turkish: i <-> I with dot above, dotless i <-> I
        assert_eq!(to_uppercase_locale("i", "tr"), "\u{0130}"); // i -> I with dot
        assert_eq!(to_lowercase_locale("I", "tr"), "\u{0131}"); // I -> dotless i
        assert_eq!(to_uppercase_locale("i", "az"), "\u{0130}"); // Azerbaijani same as Turkish
    }

    #[test]
    fn casemap_titlecase_sentence() {
        assert_eq!(to_titlecase("the quick brown fox"), "The Quick Brown Fox");
    }

    #[test]
    fn casemap_case_fold_comparison() {
        // Case folding should make case-insensitive comparison possible
        assert_eq!(case_fold("HELLO"), case_fold("hello"));
        assert_eq!(case_fold("Stra\u{00DF}e"), case_fold("STRASSE"));
    }

    #[test]
    fn casemap_fold_locale_turkic() {
        // Turkic folding: I -> dotless i (U+0131)
        let folded = case_fold_locale("I", "tr");
        assert_eq!(folded, "\u{0131}");
    }

    #[test]
    fn casemap_fold_simple_no_expansion() {
        // Simple fold should not expand string
        let s = "Stra\u{00DF}e";
        let f = case_fold_simple(s);
        assert_eq!(f.chars().count(), s.chars().count());
    }

    #[test]
    fn casemap_classification() {
        assert!(is_uppercase('A'));
        assert!(is_uppercase('\u{0391}')); // Greek capital alpha
        assert!(is_lowercase('a'));
        assert!(is_lowercase('\u{03B1}')); // Greek small alpha
        assert!(!is_uppercase('1'));
        assert!(!is_lowercase('!'));
    }

    #[test]
    fn casemap_no_case_scripts() {
        // CJK characters have no case
        let cjk = "\u{4E16}\u{754C}";
        assert_eq!(to_uppercase(cjk), cjk);
        assert_eq!(to_lowercase(cjk), cjk);
    }
}
