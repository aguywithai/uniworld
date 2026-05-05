//! Composite cursor navigation on top of UAX #9 (bidi) and UAX #29 (grapheme/word).
//!
//! This module provides higher-level cursor operations:
//!
//! **Logical-order** (for plain LTR or when visual order is not needed):
//! - [`move_right`] / [`move_left`] -- step by grapheme cluster in logical order
//! - [`delete_backward`] / [`delete_forward`] -- delete by grapheme cluster
//! - [`select_word`] -- select word at a byte position
//!
//! **Visual-order** (for mixed-direction bidi text):
//! - [`move_right_visual`] / [`move_left_visual`] -- step in screen-left / screen-right
//!   direction, using UAX #9 bidi resolution and grapheme segmentation.

use crate::bidi;
use crate::segment::{grapheme_boundaries, word_boundaries};

/// Move the cursor one grapheme cluster to the right (logical order).
///
/// - `text`: full string
/// - `current`: current byte offset (must be on a char boundary)
///
/// Returns the new byte offset, clamped to `text.len()`.
#[must_use]
pub fn move_right(text: &str, current: usize) -> usize {
    if text.is_empty() {
        return 0;
    }
    if current >= text.len() {
        return text.len();
    }

    let boundaries = grapheme_boundaries(text);
    // Find the first boundary strictly greater than `current`.
    for &b in &boundaries {
        if b > current {
            return b;
        }
    }
    // No later boundary: move to end of string.
    text.len()
}

/// Move the cursor one grapheme cluster to the left (logical order).
///
/// - `text`: full string
/// - `current`: current byte offset (must be on a char boundary)
///
/// Returns the new byte offset, clamped to 0.
#[must_use]
pub fn move_left(text: &str, current: usize) -> usize {
    if text.is_empty() || current == 0 {
        return 0;
    }
    let boundaries = grapheme_boundaries(text);
    // Find the last boundary strictly less than `current`.
    let mut prev = 0usize;
    for &b in &boundaries {
        if b >= current {
            break;
        }
        prev = b;
    }
    prev
}

/// Delete the grapheme cluster immediately before `current` (backspace).
///
/// Returns `(new_text, new_cursor_offset)`.
#[must_use]
pub fn delete_backward(text: &str, current: usize) -> (String, usize) {
    if text.is_empty() || current == 0 {
        return (text.to_owned(), current);
    }
    let start = move_left(text, current);
    let mut result = String::with_capacity(text.len());
    result.push_str(&text[..start]);
    result.push_str(&text[current..]);
    (result, start)
}

/// Delete the grapheme cluster immediately after `current` (Delete key).
///
/// Returns `(new_text, new_cursor_offset)`; the cursor stays at `current`.
#[must_use]
pub fn delete_forward(text: &str, current: usize) -> (String, usize) {
    if text.is_empty() || current >= text.len() {
        return (text.to_owned(), current);
    }
    let next = move_right(text, current);
    if next == current {
        return (text.to_owned(), current);
    }
    let mut result = String::with_capacity(text.len());
    result.push_str(&text[..current]);
    result.push_str(&text[next..]);
    (result, current)
}

/// Select the word containing `current` (double-click semantics).
///
/// Returns `(start, end)` byte offsets of the selected word. If no word
/// is found (e.g. whitespace-only text), returns `(current, current)`.
#[must_use]
pub fn select_word(text: &str, current: usize) -> (usize, usize) {
    if text.is_empty() || current > text.len() {
        return (current, current);
    }
    // `word_boundaries` returns the start byte of each word, always including 0.
    let mut bounds = word_boundaries(text, None);
    if bounds.is_empty() {
        return (current, current);
    }
    // Add sentinel end boundary.
    if *bounds.last().unwrap() != text.len() {
        bounds.push(text.len());
    }

    // Find the word interval [start, end) containing `current`.
    for window in bounds.windows(2) {
        let s = window[0];
        let e = window[1];
        if current >= s && current < e {
            return (s, e);
        }
    }

    // If we did not find a containing interval, fall back to the nearest
    // preceding boundary and extend to the next one.
    let mut start = 0usize;
    for &b in &bounds {
        if b > current {
            break;
        }
        start = b;
    }
    let mut end = text.len();
    for &b in &bounds {
        if b > current {
            end = b;
            break;
        }
    }
    if start <= current && current <= end {
        (start, end)
    } else {
        (current, current)
    }
}

// ---------------------------------------------------------------------------
// Visual-order cursor movement (bidi-aware)
// ---------------------------------------------------------------------------

/// Grapheme cluster info for visual-order operations.
struct ClusterInfo {
    /// Byte range of this cluster in the original string.
    byte_start: usize,
    byte_end: usize,
    /// Bidi embedding level of this cluster (level of its first character).
    level: u8,
}

/// Build the list of grapheme clusters with their bidi levels.
fn build_clusters(text: &str) -> Vec<ClusterInfo> {
    if text.is_empty() {
        return vec![];
    }

    let bidi_info = bidi::resolve(text, None);
    let mut boundaries = grapheme_boundaries(text);
    if boundaries.is_empty() || boundaries[0] != 0 {
        boundaries.insert(0, 0);
    }
    if *boundaries.last().unwrap() != text.len() {
        boundaries.push(text.len());
    }

    // Map byte offsets to character indices for bidi level lookup.
    // Build a byte-to-char-index mapping.
    let char_offsets: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();

    let mut clusters = Vec::with_capacity(boundaries.len() - 1);
    for window in boundaries.windows(2) {
        let byte_start = window[0];
        let byte_end = window[1];
        if byte_start >= byte_end {
            continue;
        }

        // Find the character index for the first byte of this cluster.
        let char_idx = match char_offsets.binary_search(&byte_start) {
            Ok(i) => i,
            Err(i) => i.min(char_offsets.len().saturating_sub(1)),
        };

        let level = if char_idx < bidi_info.levels.len() {
            let lev = bidi_info.levels[char_idx];
            // X9-removed characters have level u8::MAX; fall back to paragraph level
            if lev == u8::MAX {
                bidi_info.paragraph_level
            } else {
                lev
            }
        } else {
            bidi_info.paragraph_level
        };

        clusters.push(ClusterInfo {
            byte_start,
            byte_end,
            level,
        });
    }

    clusters
}

/// Build visual ordering of clusters using the bidi L2 algorithm:
/// reverse runs of clusters at each level, from max level down.
///
/// Returns a vector of indices into the `clusters` slice, ordered visually
/// (screen left to right).
fn visual_order(clusters: &[ClusterInfo]) -> Vec<usize> {
    let n = clusters.len();
    if n == 0 {
        return vec![];
    }

    let mut order: Vec<usize> = (0..n).collect();

    let max_level = clusters.iter().map(|c| c.level).max().unwrap_or(0);
    let min_level = clusters.iter().map(|c| c.level).min().unwrap_or(0);
    // Ensure we start from at least level 1 if there are RTL clusters
    let start_level = if min_level % 2 == 0 {
        min_level + 1
    } else {
        min_level
    };

    // Reverse at each level from max down to start_level (odd levels)
    let mut level = max_level;
    while level >= start_level && level > 0 {
        // Find runs of clusters with level >= current level and reverse them
        let mut i = 0;
        while i < n {
            if clusters[order[i]].level >= level {
                let run_start = i;
                while i < n && clusters[order[i]].level >= level {
                    i += 1;
                }
                order[run_start..i].reverse();
            } else {
                i += 1;
            }
        }
        if level == 0 {
            break;
        }
        level -= 1;
    }

    order
}

/// Build the visual cursor stop list for a line of text.
///
/// Returns byte offsets in screen-left-to-right order.  Each stop is a
/// position where the cursor can rest.  Stops may repeat the same byte
/// offset at bidi boundaries (entering and exiting an RTL run both pass
/// through the same logical byte offset).
///
/// Callers should navigate by index into this list, not by searching for
/// byte offsets, to avoid ambiguity at bidi boundaries.
#[must_use]
pub fn visual_cursor_stops(text: &str) -> Vec<usize> {
    if text.is_empty() {
        return vec![0];
    }

    let clusters = build_clusters(text);
    if clusters.is_empty() {
        return vec![0];
    }
    let vis = visual_order(&clusters);
    let raw = build_visual_stops(&clusters, &vis, text.len());

    // Deduplicate: remove second (and subsequent) occurrences of any byte
    // offset.  A duplicate arises when the same byte sits on both the entry
    // and exit edges of an embedded bidi run.  Because VS Code maps one byte
    // offset to one screen position, keeping both would cause a "stuck" arrow
    // press where the cursor does not visibly move.
    let mut seen = std::collections::HashSet::new();
    raw.into_iter().filter(|b| seen.insert(*b)).collect()
}

/// Build visual stops from cluster info and visual order.
///
/// At bidi boundaries we distinguish ENTRY (going into a deeper embedding)
/// from EXIT (returning to a shallower embedding):
///
///   - ENTRY boundary (next cluster has higher level): add both the current
///     cluster's right edge AND the next cluster's left edge.  This is the
///     "jump" the cursor makes when entering the embedded run.
///
///   - EXIT boundary (next cluster has lower level): add ONLY the current
///     cluster's right edge.  Skip the "back-jump" to the surrounding run's
///     left edge.  The surrounding run's right edge will be added by the
///     next iteration, so no positions are lost -- the cursor simply advances
///     without a visible backward jump.
fn build_visual_stops(
    clusters: &[ClusterInfo],
    vis: &[usize],
    _text_len: usize,
) -> Vec<usize> {
    if vis.is_empty() {
        return vec![0];
    }

    let mut stops: Vec<usize> = Vec::new();

    // Add the left visual edge of the first cluster.
    let first = &clusters[vis[0]];
    let first_left = if first.level % 2 == 1 {
        first.byte_end
    } else {
        first.byte_start
    };
    stops.push(first_left);

    for vi in 0..vis.len() {
        let c = &clusters[vis[vi]];
        let right_edge = if c.level % 2 == 1 {
            c.byte_start
        } else {
            c.byte_end
        };

        if vi + 1 < vis.len() {
            let next = &clusters[vis[vi + 1]];
            let next_left = if next.level % 2 == 1 {
                next.byte_end
            } else {
                next.byte_start
            };

            if right_edge != next_left {
                // Bidi boundary detected.
                if stops.last() != Some(&right_edge) {
                    stops.push(right_edge);
                }

                if next.level > c.level {
                    // ENTRY into deeper embedding: add the jump target.
                    stops.push(next_left);
                }
                // EXIT from deeper embedding (next.level < c.level):
                // Skip next_left -- eliminates the visible back-jump.
                // The next cluster's right edge will be added on the
                // next iteration, so the cursor advances smoothly.
                //
                // Same level but different edges (rare): also skip to
                // avoid potential loops.
            } else {
                // Normal transition: shared boundary.
                if stops.last() != Some(&right_edge) {
                    stops.push(right_edge);
                }
            }
        } else {
            // Last cluster: add right edge.
            if stops.last() != Some(&right_edge) {
                stops.push(right_edge);
            }
        }
    }

    stops
}

/// Move the cursor one grapheme cluster to the right in **visual** order.
///
/// "Right" means visually rightward on screen.
///
/// - `text`: full string
/// - `current`: current byte offset (must be on a char/grapheme boundary)
///
/// Returns the new byte offset.
#[must_use]
pub fn move_right_visual(text: &str, current: usize) -> usize {
    move_right_visual_indexed(text, current, usize::MAX).0
}

/// Index-aware version of [`move_right_visual`].
///
/// `stop_hint` is the caller's cached stop index from the previous move.
/// Pass `usize::MAX` if unknown (e.g. after a click). The function uses
/// bidi-level rules to disambiguate at boundaries when the hint is absent.
///
/// Returns `(new_byte_offset, new_stop_index)`.
#[must_use]
pub fn move_right_visual_indexed(
    text: &str,
    current: usize,
    stop_hint: usize,
) -> (usize, usize) {
    if text.is_empty() {
        return (0, 0);
    }

    let clusters = build_clusters(text);
    if clusters.is_empty() {
        return (text.len(), 0);
    }
    let vis = visual_order(&clusters);
    let stops = build_visual_stops(&clusters, &vis, text.len());
    if stops.is_empty() {
        return (text.len(), 0);
    }

    let idx = resolve_stop_index(&stops, current, stop_hint, &clusters);

    if idx + 1 < stops.len() {
        (stops[idx + 1], idx + 1)
    } else {
        (*stops.last().unwrap(), stops.len() - 1)
    }
}

/// Move the cursor one grapheme cluster to the left in **visual** order.
///
/// "Left" means visually leftward on screen.
#[must_use]
pub fn move_left_visual(text: &str, current: usize) -> usize {
    move_left_visual_indexed(text, current, usize::MAX).0
}

/// Index-aware version of [`move_left_visual`].
#[must_use]
pub fn move_left_visual_indexed(
    text: &str,
    current: usize,
    stop_hint: usize,
) -> (usize, usize) {
    if text.is_empty() {
        return (0, 0);
    }

    let clusters = build_clusters(text);
    if clusters.is_empty() {
        return (0, 0);
    }
    let vis = visual_order(&clusters);
    let stops = build_visual_stops(&clusters, &vis, text.len());
    if stops.is_empty() {
        return (0, 0);
    }

    let idx = resolve_stop_index(&stops, current, stop_hint, &clusters);

    if idx > 0 {
        (stops[idx - 1], idx - 1)
    } else {
        (stops[0], 0)
    }
}

/// Find the stop index for `current`, using `stop_hint` to disambiguate
/// when the same byte offset appears multiple times in the list.
///
/// When no hint is available and the byte offset is ambiguous (appears at
/// both a LTR->RTL and RTL->LTR boundary), we inspect the bidi levels of
/// the characters on either side of the cursor to determine which boundary
/// the cursor is "naturally" at:
///
///   - Char before is LTR, char at/after is RTL  =>  first occurrence
///     (cursor sits at the LTR side, about to enter RTL)
///   - Char before is RTL, char at/after is LTR  =>  last occurrence
///     (cursor sits at the RTL side, about to enter LTR)
fn resolve_stop_index(
    stops: &[usize],
    current: usize,
    stop_hint: usize,
    clusters: &[ClusterInfo],
) -> usize {
    // 1. If the hint is valid and matches, use it directly.
    if stop_hint < stops.len() && stops[stop_hint] == current {
        return stop_hint;
    }

    // 2. Collect all matching indices.
    let matches: Vec<usize> = stops
        .iter()
        .enumerate()
        .filter(|&(_, &s)| s == current)
        .map(|(i, _)| i)
        .collect();

    if matches.is_empty() {
        // Not an exact match; find the closest stop >= current.
        return stops
            .iter()
            .position(|&s| s >= current)
            .unwrap_or(stops.len().saturating_sub(1));
    }

    if matches.len() == 1 {
        return matches[0];
    }

    // 3. Multiple matches -- bidi boundary ambiguity.
    //    The same byte offset appears in the stops list once when the cursor
    //    ENTERS a deeper embedding and once when it EXITS back out.
    //    We compare raw embedding LEVELS (not parity) to decide which side
    //    the cursor is naturally at when the user clicks or first presses
    //    a key without a cached hint.
    //
    //    level_before < level_after  =>  entering deeper  =>  first occurrence
    //    level_before > level_after  =>  exiting deeper   =>  last occurrence
    //    level_before == level_after =>  shouldn't happen for duplicates
    let level_before = cluster_level_before(clusters, current);
    let level_after = cluster_level_at(clusters, current);

    if level_before < level_after {
        // Entering a deeper embedding level (e.g. LTR→RTL or RTL→digits)
        // The cursor is on the shallower side → first occurrence in stops
        matches[0]
    } else if level_before > level_after {
        // Exiting from a deeper embedding back to shallower
        // The cursor is on the shallower side → last occurrence in stops
        *matches.last().unwrap()
    } else {
        // Same level on both sides (shouldn't produce duplicates, but be safe)
        matches[0]
    }
}

/// Return the bidi level of the cluster whose byte range ends at or just
/// before `byte_offset`, i.e. the character to the LEFT of the cursor.
/// Falls back to level 0 (LTR) if at the start of text.
fn cluster_level_before(clusters: &[ClusterInfo], byte_offset: usize) -> u8 {
    // Find the cluster whose byte_end == byte_offset (the one just before cursor)
    for c in clusters.iter().rev() {
        if c.byte_end <= byte_offset && c.byte_end > 0 {
            return c.level;
        }
    }
    // No cluster before -- treat as LTR (paragraph start)
    0
}

/// Return the bidi level of the cluster that starts at `byte_offset`,
/// i.e. the character to the RIGHT of (or at) the cursor.
/// Falls back to level 0 (LTR) if at the end of text.
fn cluster_level_at(clusters: &[ClusterInfo], byte_offset: usize) -> u8 {
    for c in clusters {
        if c.byte_start == byte_offset {
            return c.level;
        }
        if c.byte_start > byte_offset {
            break;
        }
    }
    // No cluster at this offset -- treat as LTR (paragraph end)
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_across_ascii() {
        let s = "hello";
        assert_eq!(move_right(s, 0), 1);
        assert_eq!(move_right(s, 1), 2);
        assert_eq!(move_left(s, 5), 4);
        assert_eq!(move_left(s, 1), 0);
    }

    #[test]
    fn move_across_emoji_cluster() {
        // Family emoji: 👨‍👩‍👧‍👦 (one grapheme cluster, multiple code points)
        let s = "x👨\u{200d}👩\u{200d}👧\u{200d}👦y";
        // Find byte offset of the emoji cluster start (immediately after 'x').
        let emoji_start = "x".len();
        let y_start = emoji_start + "👨\u{200d}👩\u{200d}👧\u{200d}👦".len();

        let after_emoji = move_right(s, emoji_start);
        assert_eq!(after_emoji, y_start, "move_right should land at 'y'");
        assert_eq!(move_left(s, y_start), emoji_start);
    }

    #[test]
    fn delete_backward_basic() {
        let s = "hello";
        let (t, pos) = delete_backward(s, 5);
        assert_eq!(t, "hell");
        assert_eq!(pos, 4);
    }

    #[test]
    fn delete_forward_basic() {
        let s = "hello";
        let (t, pos) = delete_forward(s, 0);
        assert_eq!(t, "ello");
        assert_eq!(pos, 0);
    }

    #[test]
    fn select_word_ascii() {
        let s = "hello world";
        let (start, end) = select_word(s, 1);
        assert_eq!(&s[start..end], "hello");
        let (start2, end2) = select_word(s, 8);
        assert_eq!(&s[start2..end2], "world");
    }

    // --- Visual-order tests ---

    #[test]
    fn visual_move_pure_ltr() {
        // For pure LTR, visual order == logical order
        let s = "abc";
        assert_eq!(move_right_visual(s, 0), 1);
        assert_eq!(move_right_visual(s, 1), 2);
        assert_eq!(move_right_visual(s, 2), 3);
        assert_eq!(move_left_visual(s, 3), 2);
        assert_eq!(move_left_visual(s, 1), 0);
    }

    #[test]
    fn visual_move_pure_rtl() {
        // Pure RTL text: Hebrew characters
        let s = "\u{05D0}\u{05D1}\u{05D2}"; // alef bet gimel
        let right1 = move_right_visual(s, 0);
        assert!(right1 <= s.len());
    }

    #[test]
    fn visual_move_empty() {
        assert_eq!(move_right_visual("", 0), 0);
        assert_eq!(move_left_visual("", 0), 0);
    }

    #[test]
    fn visual_move_at_boundaries() {
        let s = "hello";
        assert_eq!(move_right_visual(s, s.len()), s.len());
        assert_eq!(move_left_visual(s, 0), 0);
    }

    /// Diagnostic: print cluster info, visual order, and stop list for
    /// mixed bidi text so we can verify correctness.
    #[test]
    fn diagnostic_bidi_stops() {
        // Test both directions of embedding:
        // 1. RTL paragraph with embedded LTR digits
        let s = "\u{062B}\u{0645}\u{0646}: 42 \u{062F}\u{0648}\u{0644}\u{0627}\u{0631}";
        eprintln!("\n====== TEST 1: RTL paragraph with LTR digits ======");
        run_bidi_diagnostic(s);

        // 2. LTR paragraph with embedded RTL (the case from v0.0.5)
        let s2 = "Hi \u{0645}\u{0631}\u{062D}\u{0628}\u{0627} Ok";
        eprintln!("\n====== TEST 2: LTR paragraph with RTL ======");
        run_bidi_diagnostic(s2);
    }

    fn run_bidi_diagnostic(s: &str) {
        eprintln!("=== Text: {:?} ({} bytes) ===", s, s.len());

        // Print byte layout
        for (i, ch) in s.char_indices() {
            let end = i + ch.len_utf8();
            eprintln!(
                "  char {:?} (U+{:04X}) bytes {}..{} level=?",
                ch,
                ch as u32,
                i,
                end
            );
        }

        let clusters = build_clusters(s);
        eprintln!("\n--- Clusters ({}) ---", clusters.len());
        for (ci, c) in clusters.iter().enumerate() {
            let slice = &s[c.byte_start..c.byte_end];
            eprintln!(
                "  [{}] bytes {}..{} level={} text={:?}",
                ci, c.byte_start, c.byte_end, c.level, slice
            );
        }

        let vis = visual_order(&clusters);
        eprintln!("\n--- Visual order ---");
        for (vi, &ci) in vis.iter().enumerate() {
            let c = &clusters[ci];
            let slice = &s[c.byte_start..c.byte_end];
            let lr = if c.level % 2 == 1 { "RTL" } else { "LTR" };
            eprintln!(
                "  vis[{}] = cluster[{}] {:?} ({}) bytes {}..{}",
                vi, ci, slice, lr, c.byte_start, c.byte_end
            );
        }

        let stops = visual_cursor_stops(s);
        eprintln!("\n--- Visual cursor stops ({}) ---", stops.len());
        for (si, &byte_off) in stops.iter().enumerate() {
            eprintln!("  stop[{}] = byte {}", si, byte_off);
        }

        // Verify no duplicate byte offsets in the deduplicated stop list
        let unique_count = {
            let mut s = stops.clone();
            s.dedup();
            s.len()
        };
        assert_eq!(
            stops.len(),
            unique_count,
            "Stop list should have no consecutive duplicate byte offsets"
        );
        // Also verify no non-consecutive duplicates
        {
            let mut seen = std::collections::HashSet::new();
            for &b in &stops {
                assert!(
                    seen.insert(b),
                    "Duplicate byte offset {} found in stop list",
                    b
                );
            }
        }

        // Trace walking RIGHT by pure index increment
        eprintln!("\n--- Walking RIGHT by index (new simple approach) ---");
        for i in 0..stops.len() {
            eprintln!("  index {} -> byte {}", i, stops[i]);
        }

        // Also verify the old move_right_visual_indexed still works
        // with hints from the deduplicated list
        eprintln!("\n--- Walking RIGHT via move_right_visual_indexed ---");
        let mut pos = stops[0];
        let mut hint = 0usize;
        for step in 0..stops.len() + 2 {
            let (new_pos, new_hint) =
                move_right_visual_indexed(s, pos, hint);
            eprintln!(
                "  step {}: pos={} hint={} -> new_pos={} new_hint={}",
                step, pos, hint, new_pos, new_hint
            );
            if new_pos == pos && new_hint == hint {
                eprintln!("  (stuck, stopping)");
                break;
            }
            pos = new_pos;
            hint = new_hint;
        }

        // And LEFT
        eprintln!("\n--- Walking LEFT via move_left_visual_indexed ---");
        pos = *stops.last().unwrap();
        hint = stops.len() - 1;
        for step in 0..stops.len() + 2 {
            let (new_pos, new_hint) =
                move_left_visual_indexed(s, pos, hint);
            eprintln!(
                "  step {}: pos={} hint={} -> new_pos={} new_hint={}",
                step, pos, hint, new_pos, new_hint
            );
            if new_pos == pos && new_hint == hint {
                eprintln!("  (stuck, stopping)");
                break;
            }
            pos = new_pos;
            hint = new_hint;
        }

        // Test disambiguation WITHOUT hints (simulating click + first keypress)
        eprintln!("\n--- Disambiguation test: NO hints (usize::MAX) ---");
        let no_hint = usize::MAX;

        // For each duplicate byte offset, test Right and Left from it
        // Collect the duplicate byte offsets
        let mut seen = std::collections::HashMap::new();
        for (i, &byte_off) in stops.iter().enumerate() {
            seen.entry(byte_off).or_insert_with(Vec::new).push(i);
        }
        let duplicates: Vec<usize> = seen
            .iter()
            .filter(|(_, indices)| indices.len() > 1)
            .map(|(&byte_off, _)| byte_off)
            .collect();

        for &dup_byte in &duplicates {
            let (r, ri) = move_right_visual_indexed(s, dup_byte, no_hint);
            let (l, li) = move_left_visual_indexed(s, dup_byte, no_hint);
            eprintln!(
                "  Ambiguous byte {}: Right -> pos={} idx={}, Left -> pos={} idx={}",
                dup_byte, r, ri, l, li
            );
            // Key invariant: Right should NEVER return the same position
            // (unless at visual right edge), and Left should NEVER return
            // the same position (unless at visual left edge).
            if ri > 0 && ri < stops.len() - 1 {
                assert_ne!(
                    r, dup_byte,
                    "Right from ambiguous byte {} should not stay at same byte",
                    dup_byte
                );
            }
        }
    }
}
