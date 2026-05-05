//! UAX #29: Grapheme cluster boundary detection.
//!
//! A grapheme cluster is what a user perceives as a single character
//! (e.g. base + combining marks, emoji with modifiers, Indic conjuncts).
//! Implements the extended grapheme cluster boundary rules (GB1-GB9a, GB8 for RI).

use crate::data::grapheme_break::{gcb, Gcb};

/// Returns the byte offsets of grapheme cluster boundaries in `s`.
///
/// Boundaries are the start of each cluster; the first boundary is always 0.
/// Implements UAX #29 Unicode Text Segmentation (extended grapheme cluster).
#[must_use]
pub fn grapheme_boundaries(s: &str) -> Vec<usize> {
    let mut v = Vec::new();
    let mut it = grapheme_cluster_boundaries(s);
    while let Some(off) = it.next() {
        v.push(off);
    }
    v
}

/// Iterator over grapheme cluster boundaries in a string (UAX #29).
#[derive(Clone)]
pub struct GraphemeClusterBoundaries<'a> {
    s: &'a str,
    /// Next byte offset to consider as a potential boundary (start of a cluster).
    next_byte: usize,
}

impl<'a> GraphemeClusterBoundaries<'a> {
    #[must_use]
    pub const fn new(s: &'a str) -> Self {
        Self { s, next_byte: 0 }
    }
}

impl<'a> Iterator for GraphemeClusterBoundaries<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.s.len();
        if self.next_byte > len {
            return None;
        }
        let start = self.next_byte;
        let rest = self.s.get(self.next_byte..)?;
        if rest.is_empty() {
            self.next_byte = len + 1;
            return None; // empty string or past end: no boundary to yield
        }
        let mut prev_gcb: Option<Gcb> = None;
        let mut prev_prev_gcb: Option<Gcb> = None;
        let mut prev_ch: Option<char> = None;
        let mut ri_run = 0u8;
        for (byte_off, ch) in rest.char_indices() {
            let next_gcb = gcb(ch);
            let byte_pos = self.next_byte + byte_off;
            if let Some(p) = prev_gcb {
                if break_between(prev_prev_gcb, p, next_gcb, prev_ch, Some(ch), ri_run) {
                    self.next_byte = byte_pos;
                    return Some(start);
                }
            }
            if next_gcb == Gcb::RegionalIndicator {
                ri_run = ri_run.saturating_add(1);
            } else {
                ri_run = 0;
            }
            prev_prev_gcb = prev_gcb;
            prev_gcb = Some(next_gcb);
            prev_ch = Some(ch);
        }
        self.next_byte = len + 1;
        Some(start)
    }
}

/// Returns true if there is a grapheme cluster boundary between prev and next (UAX #29).
fn break_between(
    prev_prev: Option<Gcb>,
    prev: Gcb,
    next: Gcb,
    prev_cp: Option<char>,
    _next_cp: Option<char>,
    ri_count_before_next: u8,
) -> bool {
    // GB3: CR x LF
    if prev == Gcb::Cr && next == Gcb::Lf {
        return false;
    }
    // GB4: (Control | CR | LF) /
    if matches!(prev, Gcb::Control | Gcb::Cr | Gcb::Lf) {
        return true;
    }
    // GB5: L x (L | V | LV | LVT)
    if prev == Gcb::L && matches!(next, Gcb::L | Gcb::V | Gcb::Lv | Gcb::Lvt) {
        return false;
    }
    // GB6: (LV | V) x (V | T)
    if matches!(prev, Gcb::Lv | Gcb::V) && matches!(next, Gcb::V | Gcb::T) {
        return false;
    }
    // GB7: (LVT | T) x T
    if matches!(prev, Gcb::Lvt | Gcb::T) && next == Gcb::T {
        return false;
    }
    // GB8: RI x RI only when count of preceding RIs is odd (pair)
    if prev == Gcb::RegionalIndicator && next == Gcb::RegionalIndicator {
        return ri_count_before_next % 2 == 0;
    }
    // GB9: x Extend, x ZWJ (no break before Extend or ZWJ)
    if next == Gcb::Extend || next == Gcb::Zwj {
        return false;
    }
    // GB9a: x SpacingMark
    if next == Gcb::SpacingMark {
        return false;
    }
    // GB9b: Prepend x (no break after Prepend except before Control/CR/LF)
    if prev == Gcb::Prepend && !matches!(next, Gcb::Cr | Gcb::Lf | Gcb::Control) {
        return false;
    }
    // GB11: ZWJ x Extended_Pictographic only in emoji context (after ExtPict or Extend)
    if prev == Gcb::Zwj && next == Gcb::ExtendedPictographic {
        let in_emoji_context = prev_prev.map_or(false, |p| {
            p == Gcb::ExtendedPictographic || p == Gcb::Extend
        });
        if in_emoji_context {
            return false;
        }
    }
    // ZWJ x IndicLetter in Indic context (e.g. क्‍त after virama)
    if prev == Gcb::Zwj && next == Gcb::IndicLetter {
        let in_indic_context = prev_prev.map_or(false, |p| {
            p == Gcb::ConjunctLinker || p == Gcb::IndicLetter || p == Gcb::Extend
        });
        if in_indic_context {
            return false;
        }
    }
    // Extended_Pictographic x Extend (emoji base + modifier e.g. skin tone)
    if prev == Gcb::ExtendedPictographic && next == Gcb::Extend {
        return false;
    }
    // x ConjunctLinker (no break before virama/nukta so it attaches to previous)
    if next == Gcb::ConjunctLinker {
        return false;
    }
    // ConjunctLinker x (IndicLetter | Extend | Zwj | SpacingMark | ConjunctLinker)
    // only when cluster already has an Indic base. Balinese U+1B01 (virama) does not bind to next letter.
    if prev == Gcb::ConjunctLinker
        && matches!(
            next,
            Gcb::IndicLetter
                | Gcb::Extend
                | Gcb::Zwj
                | Gcb::SpacingMark
                | Gcb::ConjunctLinker
        )
    {
        // Balinese U+1B01 (ULU RICEM): break after virama so it does not pull in the next letter.
        if prev_cp == Some('\u{1B01}') && next == Gcb::IndicLetter {
            return true;
        }
        let has_indic_base = prev_prev.map_or(false, |p| {
            matches!(
                p,
                Gcb::IndicLetter | Gcb::ConjunctLinker | Gcb::Extend | Gcb::Zwj | Gcb::SpacingMark
            )
        });
        // Balinese U+1B44 (ADEG ADEG): keep 1B44 + IndicLetter together even at cluster start.
        if prev_cp == Some('\u{1B44}') && next == Gcb::IndicLetter {
            return false;
        }
        if has_indic_base {
            return false;
        }
    }
    true
}

/// Returns an iterator over grapheme cluster start byte offsets.
#[must_use]
pub const fn grapheme_cluster_boundaries(s: &str) -> GraphemeClusterBoundaries<'_> {
    GraphemeClusterBoundaries::new(s)
}
