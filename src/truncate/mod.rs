//! Safe string truncation utilities.
//!
//! - Never split a grapheme cluster (UAX #29)
//! - Optionally respect display width constraints (see `crate::width`)

use crate::segment::grapheme_boundaries;
use crate::width::display_width;

/// Truncate `text` to at most `max_graphemes` grapheme clusters.
///
/// The result is always a valid UTF-8 string and never cuts through a
/// grapheme cluster.
#[must_use]
pub fn truncate_graphemes(text: &str, max_graphemes: usize) -> &str {
    if max_graphemes == 0 || text.is_empty() {
        return "";
    }
    let bounds = grapheme_boundaries(text);
    if bounds.len() <= max_graphemes {
        return text;
    }
    let byte_cut = bounds[max_graphemes];
    &text[..byte_cut]
}

/// Truncate `text` so that its display width (see [`crate::width::display_width`])
/// does not exceed `max_width`. Truncation is done on grapheme boundaries.
///
/// Returns a newly allocated `String`.
#[must_use]
pub fn truncate_display_width(text: &str, max_width: u32) -> String {
    if max_width == 0 || text.is_empty() {
        return String::new();
    }

    let mut bounds = grapheme_boundaries(text);
    if *bounds.last().unwrap_or(&0) != text.len() {
        bounds.push(text.len());
    }
    let mut acc_width: u32 = 0;
    let mut last_byte: usize = 0;

    for window in bounds.windows(2) {
        let start = window[0];
        let end = window[1];
        let cluster = &text[start..end];
        let w = display_width(cluster);
        if acc_width + w > max_width {
            break;
        }
        acc_width += w;
        last_byte = end;
    }

    if last_byte == 0 {
        String::new()
    } else {
        text[..last_byte].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_by_graphemes() {
        let s = "a👩\u{200d}💻b"; // a, woman technologist, b
        let t = truncate_graphemes(s, 2);
        // Should keep "a" + the emoji cluster and drop trailing 'b'.
        assert!(t.starts_with('a'));
        assert!(!t.ends_with('b'));
    }

    #[test]
    fn truncate_by_width_simple() {
        let s = "a一b"; // width 1,2,1
        let t = truncate_display_width(s, 3);
        // Can fit "a一" (width 3) but not the trailing "b".
        assert_eq!(t, "a一");
    }
}
