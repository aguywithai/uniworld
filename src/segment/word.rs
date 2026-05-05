//! UAX #29: Word boundary detection.
//!
//! Used for word selection (double-click), spell checking, word-level operations.
//! Implements default word boundary rules (WB1-WB999); locale tailoring reserved for future.
//!
//! Architecture:
//!   1. Pre-collect characters, their WB properties, and byte offsets.
//!   2. For each adjacent pair, apply rules WB3-WB3c on RAW properties.
//!   3. WB4 (Extend/Format/ZWJ transparency): do not break before transparent chars.
//!   4. For WB5-WB999: resolve prev/prev_prev/next_next by skipping transparent chars.

use crate::data::word_break::{is_extended_pictographic, wb, Wb};

/// Returns the byte offsets of word boundaries in `s` (start of each word).
///
/// Boundaries are the start of each word; the first boundary is always 0.
/// Implements UAX #29 default word boundary rules (WB1-WB999).
/// `locale` reserved for future tailoring (e.g. Thai, dictionary).
#[must_use]
pub fn word_boundaries(s: &str, _locale: Option<&str>) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return vec![0];
    }

    // Pre-compute WB property and byte offset for each character
    let wbs: Vec<Wb> = chars.iter().map(|&c| wb(c)).collect();
    let offsets: Vec<usize> = {
        let mut v = Vec::with_capacity(n);
        let mut off = 0usize;
        for &c in &chars {
            v.push(off);
            off += c.len_utf8();
        }
        v
    };

    let mut out = vec![0]; // WB1: sot boundary
    let mut ri_count: u32 = 0; // consecutive RI count in resolved stream (for WB15/16)

    // Initialize ri_count for the first character
    if !is_transparent(wbs[0]) && wbs[0] == Wb::RegionalIndicator {
        ri_count = 1;
    }

    for i in 1..n {
        let prev_raw = wbs[i - 1];
        let next_raw = wbs[i];

        // WB3: CR x LF
        if prev_raw == Wb::Cr && next_raw == Wb::Lf {
            continue; // no break
        }
        // WB3a: (Newline|CR|LF) /
        if matches!(prev_raw, Wb::Newline | Wb::Cr | Wb::Lf) {
            out.push(offsets[i]);
            ri_count = 0;
            continue;
        }
        // WB3b: / (Newline|CR|LF)
        if matches!(next_raw, Wb::Newline | Wb::Cr | Wb::Lf) {
            out.push(offsets[i]);
            ri_count = 0;
            continue;
        }
        // WB3c: ZWJ x Extended_Pictographic (raw adjacency only)
        if prev_raw == Wb::Zwj && is_extended_pictographic(chars[i]) {
            continue; // no break
        }
        // WB3d: WSegSpace x WSegSpace (do not break between horizontal whitespace)
        // Applied on raw properties (before WB4) so spaces stay together.
        if prev_raw == Wb::WSegSpace && next_raw == Wb::WSegSpace {
            continue; // no break
        }
        // WB4: X (Extend|Format|ZWJ)* -> X
        // Do not break before Extend, Format, or ZWJ.
        if is_transparent(next_raw) {
            continue; // no break
        }

        // --- From here, next_raw is non-transparent. Apply WB5-WB999 with resolved properties. ---
        let next = next_raw;
        let prev = resolve_prev(&wbs, i);
        let prev_prev = resolve_prev_prev(&wbs, i);
        let next_next = resolve_next_next(&wbs, i);

        let brk = match prev {
            Some(p) => apply_wb_rules(p, prev_prev, next, next_next, ri_count),
            None => true, // all preceding are transparent -> break
        };

        // Update RI counter for WB15/16
        if next == Wb::RegionalIndicator {
            if brk {
                ri_count = 1;
            } else {
                ri_count += 1;
            }
        } else {
            ri_count = 0;
        }

        if brk {
            out.push(offsets[i]);
        }
    }
    out
}

/// True if this WB value is transparent per WB4 (Extend, Format, ZWJ).
fn is_transparent(w: Wb) -> bool {
    matches!(w, Wb::Extend | Wb::Format | Wb::Zwj)
}

/// Find the WB of the nearest non-transparent character before position `i`.
fn resolve_prev(wbs: &[Wb], i: usize) -> Option<Wb> {
    let mut j = i;
    while j > 0 {
        j -= 1;
        if !is_transparent(wbs[j]) {
            return Some(wbs[j]);
        }
    }
    None
}

/// Find the WB of the second non-transparent character before position `i`.
fn resolve_prev_prev(wbs: &[Wb], i: usize) -> Option<Wb> {
    // First find the nearest non-transparent
    let mut j = i;
    while j > 0 {
        j -= 1;
        if !is_transparent(wbs[j]) {
            break;
        }
        if j == 0 {
            return None;
        }
    }
    // Now find the one before that
    while j > 0 {
        j -= 1;
        if !is_transparent(wbs[j]) {
            return Some(wbs[j]);
        }
    }
    None
}

/// Find the WB of the nearest non-transparent character after position `i`.
fn resolve_next_next(wbs: &[Wb], i: usize) -> Option<Wb> {
    let mut j = i + 1;
    while j < wbs.len() {
        if !is_transparent(wbs[j]) {
            return Some(wbs[j]);
        }
        j += 1;
    }
    None
}

/// Apply WB5-WB999 on resolved properties. Returns true if there should be a break.
///
/// `prev`: resolved previous non-transparent WB.
/// `prev_prev`: resolved second-previous non-transparent WB (for WB7/WB11/WB7c).
/// `next`: current non-transparent WB.
/// `next_next`: resolved next non-transparent WB after current (for WB6/WB7b/WB12).
/// `ri_count`: number of consecutive RIs in the resolved stream up to and including prev.
fn apply_wb_rules(
    prev: Wb,
    prev_prev: Option<Wb>,
    next: Wb,
    next_next: Option<Wb>,
    ri_count: u32,
) -> bool {
    let ah_letter = |w: Wb| matches!(w, Wb::ALetter | Wb::HebrewLetter);
    // MidNumLetQ = MidNumLet | Single_Quote (UAX #29 notation)
    let mid_letter = |w: Wb| matches!(w, Wb::MidLetter | Wb::MidNumLet | Wb::SingleQuote);
    let mid_num = |w: Wb| matches!(w, Wb::MidNum | Wb::MidNumLet | Wb::SingleQuote);

    // WB5: AHLetter x AHLetter
    if ah_letter(prev) && ah_letter(next) {
        return false;
    }
    // WB6: AHLetter x (MidLetter|MidNumLetQ) AHLetter
    if ah_letter(prev) && mid_letter(next) {
        if let Some(nn) = next_next {
            if ah_letter(nn) {
                return false;
            }
        }
    }
    // WB7: AHLetter (MidLetter|MidNumLetQ) x AHLetter
    if let Some(pp) = prev_prev {
        if ah_letter(pp) && mid_letter(prev) && ah_letter(next) {
            return false;
        }
    }
    // WB7a: Hebrew_Letter x Single_Quote
    if prev == Wb::HebrewLetter && next == Wb::SingleQuote {
        return false;
    }
    // WB7b: Hebrew_Letter x Double_Quote Hebrew_Letter
    if prev == Wb::HebrewLetter && next == Wb::DoubleQuote {
        if let Some(nn) = next_next {
            if nn == Wb::HebrewLetter {
                return false;
            }
        }
    }
    // WB7c: Hebrew_Letter Double_Quote x Hebrew_Letter
    if let Some(pp) = prev_prev {
        if pp == Wb::HebrewLetter && prev == Wb::DoubleQuote && next == Wb::HebrewLetter {
            return false;
        }
    }
    // WB8: Numeric x Numeric
    if prev == Wb::Numeric && next == Wb::Numeric {
        return false;
    }
    // WB9: AHLetter x Numeric
    if ah_letter(prev) && next == Wb::Numeric {
        return false;
    }
    // WB10: Numeric x AHLetter
    if prev == Wb::Numeric && ah_letter(next) {
        return false;
    }
    // WB11: Numeric (MidNum|MidNumLetQ) x Numeric
    if let Some(pp) = prev_prev {
        if pp == Wb::Numeric && mid_num(prev) && next == Wb::Numeric {
            return false;
        }
    }
    // WB12: Numeric x (MidNum|MidNumLetQ) Numeric
    if prev == Wb::Numeric && mid_num(next) {
        if let Some(nn) = next_next {
            if nn == Wb::Numeric {
                return false;
            }
        }
    }
    // WB13: Katakana x Katakana
    if prev == Wb::Katakana && next == Wb::Katakana {
        return false;
    }
    // WB13a: (AHLetter|Numeric|Katakana|ExtendNumLet) x ExtendNumLet
    if next == Wb::ExtendNumLet
        && matches!(
            prev,
            Wb::ALetter
                | Wb::HebrewLetter
                | Wb::Numeric
                | Wb::Katakana
                | Wb::ExtendNumLet
        )
    {
        return false;
    }
    // WB13b: ExtendNumLet x (AHLetter|Numeric|Katakana)
    if prev == Wb::ExtendNumLet
        && matches!(
            next,
            Wb::ALetter | Wb::HebrewLetter | Wb::Numeric | Wb::Katakana
        )
    {
        return false;
    }
    // WB15/WB16: Do not break within emoji flag sequences (RI pairing).
    // ri_count tracks consecutive RIs in the resolved stream ending at prev.
    // If ri_count is odd, prev is "unpaired" and pairs with next -> no break.
    if prev == Wb::RegionalIndicator && next == Wb::RegionalIndicator {
        if ri_count % 2 == 1 {
            return false; // odd: forming a pair
        }
        // even: previous RIs are all paired, this starts a new sequence -> break
    }
    // WB999: Any / Any
    true
}
