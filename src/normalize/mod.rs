//! UAX #15: Unicode Normalization Forms (NFC, NFD, NFKC, NFKD).
//!
//! Implements all four normalization forms per Unicode Standard Annex #15.
//!
//! - **NFD**: Canonical Decomposition
//! - **NFC**: Canonical Decomposition + Canonical Composition
//! - **NFKD**: Compatibility Decomposition
//! - **NFKC**: Compatibility Decomposition + Canonical Composition
//!
//! Algorithm:
//!   1. Decompose: replace each character with its decomposition (recursively).
//!   2. Sort: reorder combining marks by Canonical Combining Class (CCC), stable sort.
//!   3. Compose (NFC/NFKC only): combine adjacent composable pairs.
//!
//! Data tables are auto-generated from UnicodeData.txt by
//! `_development/scripts/generate_normalization_tables.py`.

use crate::data::normalization::{
    canonical_composition, canonical_decomposition, ccc, compatibility_decomposition,
};

// Hangul constants (Unicode Standard, Chapter 3)
const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;
const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = V_COUNT * T_COUNT; // 588
const S_COUNT: u32 = L_COUNT * N_COUNT; // 11172

/// Normalization form NFD (Canonical Decomposition).
///
/// Recursively decomposes characters using canonical decomposition mappings,
/// then reorders combining marks by Canonical Combining Class (CCC).
#[must_use]
pub fn nfd(s: &str) -> String {
    let decomposed = decompose(s, false);
    String::from_iter(decomposed.iter().map(|&cp| unsafe {
        char::from_u32_unchecked(cp)
    }))
}

/// Normalization form NFC (Canonical Decomposition + Canonical Composition).
///
/// Applies NFD, then combines adjacent composable character pairs.
#[must_use]
pub fn nfc(s: &str) -> String {
    let decomposed = decompose(s, false);
    let composed = compose(&decomposed);
    String::from_iter(composed.iter().map(|&cp| unsafe {
        char::from_u32_unchecked(cp)
    }))
}

/// Normalization form NFKD (Compatibility Decomposition).
///
/// Recursively decomposes characters using compatibility decomposition mappings,
/// then reorders combining marks by CCC.
#[must_use]
pub fn nfkd(s: &str) -> String {
    let decomposed = decompose(s, true);
    String::from_iter(decomposed.iter().map(|&cp| unsafe {
        char::from_u32_unchecked(cp)
    }))
}

/// Normalization form NFKC (Compatibility Decomposition + Canonical Composition).
///
/// Applies NFKD, then combines adjacent composable character pairs.
#[must_use]
pub fn nfkc(s: &str) -> String {
    let decomposed = decompose(s, true);
    let composed = compose(&decomposed);
    String::from_iter(composed.iter().map(|&cp| unsafe {
        char::from_u32_unchecked(cp)
    }))
}

/// Decompose a string into code points, applying the appropriate decomposition
/// (canonical or compatibility) recursively, then sorting by CCC.
fn decompose(s: &str, compat: bool) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::with_capacity(s.len());

    for ch in s.chars() {
        decompose_char(ch as u32, compat, &mut result);
    }

    // Canonical ordering: stable sort consecutive combining marks by CCC.
    // Only reorder within sequences of non-zero CCC (starters are fixed points).
    canonical_order(&mut result);

    result
}

/// Recursively decompose a single code point, appending results to `out`.
fn decompose_char(cp: u32, compat: bool, out: &mut Vec<u32>) {
    // Hangul algorithmic decomposition (Unicode Chapter 3)
    if cp >= S_BASE && cp < S_BASE + S_COUNT {
        let s_index = cp - S_BASE;
        let l = L_BASE + s_index / N_COUNT;
        let v = V_BASE + (s_index % N_COUNT) / T_COUNT;
        let t = T_BASE + s_index % T_COUNT;
        out.push(l);
        out.push(v);
        if t != T_BASE {
            out.push(t);
        }
        return;
    }

    let decomp = if compat {
        compatibility_decomposition(cp)
    } else {
        canonical_decomposition(cp)
    };

    match decomp {
        Some(mapping) => {
            // Recursively decompose each code point in the mapping
            for &dcp in mapping {
                decompose_char(dcp, compat, out);
            }
        }
        None => {
            // No further decomposition; emit the code point as-is
            out.push(cp);
        }
    }
}

/// Apply canonical ordering to a sequence of code points.
///
/// Reorders combining marks (non-zero CCC) by their CCC values using
/// a stable sort. Starters (CCC=0) act as boundaries and are not moved.
fn canonical_order(cps: &mut [u32]) {
    let len = cps.len();
    if len < 2 {
        return;
    }

    // Bubble sort combining marks by CCC (stable).
    // This is efficient because combining mark sequences are typically short.
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..len - 1 {
            let ccc_a = ccc(cps[i]);
            let ccc_b = ccc(cps[i + 1]);
            // Swap if both are non-zero CCC and left > right
            if ccc_a > ccc_b && ccc_b != 0 {
                cps.swap(i, i + 1);
                changed = true;
            }
        }
    }
}

/// Canonical composition: combine adjacent composable pairs.
///
/// Implements the Canonical Composition Algorithm from UAX #15 Section 1.3:
/// 1. Scan left to right for starters (CCC=0).
/// 2. For each starter, attempt to compose it with following characters.
/// 3. A character is "blocked" from composing with the starter if there's
///    an intervening character with CCC >= the candidate's CCC (and CCC != 0).
fn compose(cps: &[u32]) -> Vec<u32> {
    if cps.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<u32> = Vec::with_capacity(cps.len());
    let len = cps.len();

    // Work buffer: copy all code points, then compose in place
    let buf: Vec<u32> = cps.to_vec();
    // Flags for "consumed" (composed into something else)
    let mut consumed = vec![false; len];

    let mut i = 0;
    while i < len {
        if consumed[i] {
            i += 1;
            continue;
        }

        let starter_ccc = ccc(buf[i]);
        if starter_ccc != 0 {
            // Not a starter; just emit
            result.push(buf[i]);
            i += 1;
            continue;
        }

        // This is a starter. Try to compose with following characters.
        let mut starter = buf[i];
        let mut last_ccc: u8 = 0;

        let mut j = i + 1;
        while j < len {
            if consumed[j] {
                j += 1;
                continue;
            }

            let c = buf[j];
            let c_ccc = ccc(c);

            // Check if c is blocked from starter:
            // c is blocked if there's a character between starter and c
            // with CCC >= c_ccc (and c_ccc != 0), or if c_ccc == 0 and it's not
            // adjacent to the starter (there's a non-consumed char between them).
            let blocked = if c_ccc == 0 {
                // Another starter: blocked unless it's immediately after the current starter
                // (checked by seeing if last_ccc > 0, meaning something intervenes)
                last_ccc > 0 || (j != i + 1 && has_non_consumed_between(i, j, &consumed))
            } else {
                // Non-starter: blocked if last_ccc >= c_ccc
                last_ccc >= c_ccc
            };

            if !blocked {
                // Try Hangul Jamo composition first, then table lookup
                if let Some(composite) = hangul_compose(starter, c)
                    .or_else(|| canonical_composition(starter, c))
                {
                    // Compose!
                    starter = composite;
                    consumed[j] = true;
                    // Don't update last_ccc since this character was consumed
                    j += 1;
                    continue;
                }
            }

            // If c_ccc == 0, it's a new starter boundary; stop trying to compose
            if c_ccc == 0 {
                break;
            }

            last_ccc = c_ccc;
            j += 1;
        }

        result.push(starter);
        i += 1;
    }

    result
}

/// Hangul Jamo algorithmic composition (Unicode Chapter 3).
///
/// Composes:
/// - L + V -> LV syllable
/// - LV + T -> LVT syllable
fn hangul_compose(first: u32, second: u32) -> Option<u32> {
    // L + V -> LV
    if first >= L_BASE && first < L_BASE + L_COUNT
        && second >= V_BASE && second < V_BASE + V_COUNT
    {
        let l_index = first - L_BASE;
        let v_index = second - V_BASE;
        let lv = S_BASE + (l_index * N_COUNT) + (v_index * T_COUNT);
        return Some(lv);
    }

    // LV + T -> LVT
    if first >= S_BASE && first < S_BASE + S_COUNT {
        let s_index = first - S_BASE;
        if s_index % T_COUNT == 0 {
            // first is an LV syllable
            if second > T_BASE && second < T_BASE + T_COUNT {
                let lvt = first + (second - T_BASE);
                return Some(lvt);
            }
        }
    }

    None
}

/// Check if there's any non-consumed character between positions `start` and `end` (exclusive).
fn has_non_consumed_between(start: usize, end: usize, consumed: &[bool]) -> bool {
    for k in (start + 1)..end {
        if !consumed[k] {
            return true;
        }
    }
    false
}
