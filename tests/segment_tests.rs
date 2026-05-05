//! Tests for UAX #29 text segmentation (grapheme, word, sentence).
//! Conformance tests in conformance_grapheme.rs (and conformance_word when data present).

use uniworld::segment::{
    grapheme_boundaries, grapheme_cluster_boundaries, sentence_boundaries, word_boundaries,
};

#[test]
fn grapheme_boundaries_empty() {
    let b = grapheme_boundaries("");
    assert!(b.is_empty());
}

#[test]
fn grapheme_boundaries_ascii() {
    let b = grapheme_boundaries("abc");
    assert_eq!(b, [0, 1, 2]);
}

#[test]
fn grapheme_cluster_boundaries_iterator() {
    let s = "ab";
    let b: Vec<usize> = grapheme_cluster_boundaries(s).collect();
    assert_eq!(b, [0, 1]);
}

/// Base + combining mark = one cluster (UAX #29 GB9).
#[test]
fn grapheme_base_plus_extend() {
    // e + combining acute accent (U+0301) = one grapheme cluster
    let s = "e\u{0301}";
    let b = grapheme_boundaries(s);
    assert_eq!(b, [0], "e + acute should be one cluster");
}

/// CR LF = one cluster (UAX #29 GB3).
#[test]
fn grapheme_crlf() {
    let s = "a\r\nb";
    let b = grapheme_boundaries(s);
    // Byte offsets: 0=a, 1=\r, 2=\n, 3=b -> cluster starts at 0, 1, 3
    assert_eq!(b, [0, 1, 3], "a, CRLF, b");
}

/// Two Regional_Indicator = one cluster (flag emoji, UAX #29 GB8).
#[test]
fn grapheme_regional_indicator_pair() {
    // Two RIs form one cluster (e.g. flag)
    let s = "\u{1F1E6}\u{1F1E8}"; // AE = regional indicator A + E
    let b = grapheme_boundaries(s);
    assert_eq!(b, [0], "two RIs = one cluster");
}

/// Four RIs = two clusters (two flags).
#[test]
fn grapheme_four_regional_indicators() {
    let s = "\u{1F1E6}\u{1F1E8}\u{1F1E6}\u{1F1E8}";
    let b = grapheme_boundaries(s);
    assert_eq!(b, [0, 8], "four RIs = two clusters");
}

// --- Word boundaries (UAX #29) ---

#[test]
fn word_boundaries_empty() {
    let b = word_boundaries("", None);
    assert_eq!(b, [0]);
}

#[test]
fn word_boundaries_hello_world() {
    let b = word_boundaries("hello world", None);
    assert_eq!(b, [0, 5, 6], "hello | space | world");
}

#[test]
fn word_boundaries_single_word() {
    let b = word_boundaries("hello", None);
    assert_eq!(b, [0]);
}

// --- Sentence boundaries (UAX #29) ---

#[test]
fn sentence_boundaries_empty() {
    let b = sentence_boundaries("", None);
    assert_eq!(b, [0]);
}

#[test]
fn sentence_boundaries_single_sentence() {
    let b = sentence_boundaries("Hello world", None);
    assert_eq!(b, [0], "no sentence break without terminator");
}

#[test]
fn sentence_boundaries_period_uppercase() {
    // "Hello. World" -> two sentences
    let b = sentence_boundaries("Hello. World", None);
    assert_eq!(b, [0, 7], "Hello. | World");
}

#[test]
fn sentence_boundaries_abbreviation() {
    // "U.S. Army" -> SB7 prevents break (Upper ATerm x Upper)
    let b = sentence_boundaries("U.S.", None);
    assert_eq!(b, [0], "abbreviation should be one sentence");
}

#[test]
fn sentence_boundaries_crlf() {
    // CR LF is one paragraph separator
    let b = sentence_boundaries("a.\r\nb", None);
    // a. CR LF b -> break after LF
    assert_eq!(b, [0, 4], "a.CRLF | b");
}

#[test]
fn sentence_boundaries_exclamation() {
    let b = sentence_boundaries("Wow! Great.", None);
    assert_eq!(b, [0, 5], "Wow! | Great.");
}
