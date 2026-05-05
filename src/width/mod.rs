//! Display width calculation for terminal and fixed-width contexts.
//!
//! This module provides a simple, Unicode-aware display width model suitable
//! for terminal-style rendering:
//! - Full-width CJK characters count as width 2
//! - Combining marks and zero-width joiners count as width 0
//! - Most other characters count as width 1
//!
//! The model is intentionally conservative and deterministic; it does not
//! attempt locale-specific tailoring for ambiguous-width characters.

use crate::data::grapheme_break::gcb;
use crate::data::grapheme_break::Gcb;
use crate::data::line_break::is_east_asian_wide;

/// Compute the display width of a string in column cells.
///
/// - Full-width East Asian characters: width 2
/// - Combining marks / ZWJ: width 0
/// - Control characters (except newline): width 0
/// - Everything else: width 1
#[must_use]
pub fn display_width(s: &str) -> u32 {
    s.chars().map(char_width).sum()
}

/// Compute the display width of a single Unicode scalar.
#[must_use]
pub fn char_width(ch: char) -> u32 {
    let cp = ch as u32;

    // Newline and other control chars do not occupy a column cell.
    if ch == '\n' || ch == '\r' || ch == '\u{0008}' {
        return 0;
    }

    // Combining marks and zero-width joiners have width 0.
    let g = gcb(ch);
    if matches!(g, Gcb::Extend | Gcb::Zwj | Gcb::SpacingMark | Gcb::ConjunctLinker) {
        return 0;
    }

    // C0/C1 control range: width 0.
    if (cp <= 0x1F) || (0x7F..=0x9F).contains(&cp) {
        return 0;
    }

    // East Asian wide/full-width characters occupy 2 cells.
    if is_east_asian_wide(cp) {
        return 2;
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_ascii() {
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("abc"), 3);
    }

    #[test]
    fn width_cjk() {
        // 一 is East Asian wide.
        let s = "a一b";
        assert_eq!(display_width(s), 1 + 2 + 1);
    }

    #[test]
    fn width_combining() {
        let s = "e\u{0301}"; // e + combining acute
        assert_eq!(display_width(s), 1);
    }
}
