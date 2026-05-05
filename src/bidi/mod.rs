//! UAX #9: Unicode Bidirectional Algorithm.
//!
//! Resolves mixed LTR/RTL text to visual order and provides logical/visual
//! position mappings.  Implements rules P1-P3, X1-X10, W1-W7, N0-N2, I1-I2,
//! L1-L4 per <https://www.unicode.org/reports/tr9/>.
//!
//! # Public API
//!
//! - [`BidiInfo`]: resolved paragraph direction, embedding levels, and visual
//!   reordering for a single paragraph.
//! - [`resolve`]: run the full algorithm on a paragraph string.
//! - [`resolve_classes`]: run the algorithm on a sequence of explicit
//!   `BidiClass` values (used by the class-level conformance tests).

use crate::data::bidi_class::{
    bidi_class, bracket_pair, is_closing_bracket, opening_bracket_for, BidiClass,
};

// Re-export BidiClass so callers can use bidi::BidiClass.
pub use crate::data::bidi_class::BidiClass as Class;

/// Maximum explicit embedding depth (UAX #9 BD2).
const MAX_DEPTH: u8 = 125;

/// Result of running the bidi algorithm on a single paragraph.
#[derive(Debug, Clone)]
pub struct BidiInfo {
    /// Resolved paragraph embedding level (0 = LTR, 1 = RTL).
    pub paragraph_level: u8,
    /// Resolved embedding level for each input element.
    /// Characters removed by X9 have level `u8::MAX`.
    pub levels: Vec<u8>,
    /// Visual ordering: indices into the original input, left-to-right.
    /// Characters removed by X9 are omitted.
    pub reorder: Vec<usize>,
}

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Resolve a paragraph string.  Returns [`BidiInfo`] with one level per
/// *character* (indexed by character position in the string).
///
/// `paragraph_override`: `None` for auto-detect (P2/P3), `Some(0)` force LTR,
/// `Some(1)` force RTL.
pub fn resolve(s: &str, paragraph_override: Option<u8>) -> BidiInfo {
    let chars: Vec<char> = s.chars().collect();
    let classes: Vec<BidiClass> = chars.iter().map(|&c| bidi_class(c as u32)).collect();
    let cps: Vec<u32> = chars.iter().map(|&c| c as u32).collect();
    resolve_inner(&classes, Some(&cps), paragraph_override)
}

/// Resolve a sequence of explicit [`BidiClass`] values (no actual characters).
/// Used by BidiTest.txt conformance tests which operate on class sequences.
pub fn resolve_classes(classes: &[BidiClass], paragraph_override: Option<u8>) -> BidiInfo {
    resolve_inner(classes, None, paragraph_override)
}

// ---------------------------------------------------------------------------
// Core algorithm
// ---------------------------------------------------------------------------

fn resolve_inner(
    original_classes: &[BidiClass],
    cps: Option<&[u32]>,
    paragraph_override: Option<u8>,
) -> BidiInfo {
    let n = original_classes.len();
    if n == 0 {
        return BidiInfo {
            paragraph_level: paragraph_override.unwrap_or(0),
            levels: vec![],
            reorder: vec![],
        };
    }

    // --- P2/P3: determine paragraph embedding level ---
    let para_level = match paragraph_override {
        Some(lev) => lev,
        None => determine_paragraph_level(original_classes),
    };

    // Working copy of classes (modified by X and subsequent rules).
    let mut classes: Vec<BidiClass> = original_classes.to_vec();
    let mut levels: Vec<u8> = vec![para_level; n];

    // --- X1-X8: explicit embeddings, overrides, isolates ---
    apply_explicit_levels(&mut classes, &mut levels, para_level);

    // --- X9: remove BN, embedding/override controls ---
    // Must use original_classes because X6 overrides may have changed BN to L/R.
    let removed = apply_x9(original_classes, &mut classes, &mut levels);

    // --- X10: run W, N, I rules on each isolating run sequence ---
    let sequences = build_isolating_run_sequences(
        original_classes,
        &classes,
        &levels,
        &removed,
        para_level,
    );

    for seq in &sequences {
        let sos = seq.sos;
        let eos = seq.eos;
        let indices = &seq.indices;
        let embed_level = seq.level;
        let embed_dir = if embed_level % 2 == 0 {
            BidiClass::L
        } else {
            BidiClass::R
        };

        // Extract working class slice for this sequence.
        let mut sc: Vec<BidiClass> = indices.iter().map(|&i| classes[i]).collect();

        // Record which positions are NSM *before* W rules change them.
        // N0 needs this to apply bracket type to following NSMs.
        let pre_w_nsm: Vec<bool> = sc.iter().map(|&c| c == BidiClass::NSM).collect();

        // --- W1-W7: weak type resolution ---
        resolve_weak_types(&mut sc, sos);

        // --- N0: bracket pair resolution ---
        if let Some(code_points) = cps {
            resolve_bracket_pairs(
                &mut sc, indices, code_points, &levels, embed_dir, sos,
                &pre_w_nsm,
            );
        }

        // --- N1-N2: neutral type resolution ---
        resolve_neutral_types(&mut sc, sos, eos, embed_dir);

        // --- I1-I2: implicit level resolution ---
        resolve_implicit_levels(&sc, indices, &mut levels);

        // Write resolved classes back.
        for (j, &i) in indices.iter().enumerate() {
            classes[i] = sc[j];
        }
    }

    // --- L1: reset levels of certain characters ---
    apply_l1(original_classes, &mut levels, para_level);

    // Restore X9-removed characters: they must retain level u8::MAX.
    for i in 0..n {
        if removed[i] {
            levels[i] = u8::MAX;
        }
    }

    // --- L2: reorder ---
    let reorder = compute_reorder(&levels);

    BidiInfo {
        paragraph_level: para_level,
        levels,
        reorder,
    }
}

// ---------------------------------------------------------------------------
// P2/P3: paragraph direction
// ---------------------------------------------------------------------------

fn determine_paragraph_level(classes: &[BidiClass]) -> u8 {
    // P2: find first strong character, skipping matched isolate pairs.
    let mut isolate_depth: u32 = 0;
    for &bc in classes {
        match bc {
            BidiClass::LRI | BidiClass::RLI | BidiClass::FSI => {
                isolate_depth += 1;
            }
            BidiClass::PDI => {
                if isolate_depth > 0 {
                    isolate_depth -= 1;
                }
            }
            _ => {
                if isolate_depth == 0 {
                    match bc {
                        BidiClass::L => return 0,
                        BidiClass::R | BidiClass::AL => return 1,
                        _ => {}
                    }
                }
            }
        }
    }
    0 // P3: default LTR
}

// ---------------------------------------------------------------------------
// X1-X8: explicit embeddings/overrides/isolates
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct DirectionalStatus {
    level: u8,
    override_status: Override,
    isolate_status: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Override {
    Neutral,
    LTR,
    RTL,
}

fn apply_explicit_levels(classes: &mut [BidiClass], levels: &mut [u8], para_level: u8) {
    let n = classes.len();
    let mut stack: Vec<DirectionalStatus> = Vec::with_capacity(MAX_DEPTH as usize + 2);
    stack.push(DirectionalStatus {
        level: para_level,
        override_status: Override::Neutral,
        isolate_status: false,
    });
    let mut overflow_isolate_count: u32 = 0;
    let mut overflow_embedding_count: u32 = 0;
    let mut valid_isolate_count: u32 = 0;

    for i in 0..n {
        let bc = classes[i];
        let current = *stack.last().unwrap();

        match bc {
            // X2: RLE
            BidiClass::RLE => {
                let new_level = (current.level + 1) | 1;
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::Neutral,
                        isolate_status: false,
                    });
                } else if overflow_isolate_count == 0 {
                    overflow_embedding_count += 1;
                }
            }
            // X3: LRE
            BidiClass::LRE => {
                let new_level = (current.level + 2) & !1;
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::Neutral,
                        isolate_status: false,
                    });
                } else if overflow_isolate_count == 0 {
                    overflow_embedding_count += 1;
                }
            }
            // X4: RLO
            BidiClass::RLO => {
                let new_level = (current.level + 1) | 1;
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::RTL,
                        isolate_status: false,
                    });
                } else if overflow_isolate_count == 0 {
                    overflow_embedding_count += 1;
                }
            }
            // X5: LRO
            BidiClass::LRO => {
                let new_level = (current.level + 2) & !1;
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::LTR,
                        isolate_status: false,
                    });
                } else if overflow_isolate_count == 0 {
                    overflow_embedding_count += 1;
                }
            }
            // X5a: RLI
            BidiClass::RLI => {
                levels[i] = current.level;
                apply_override(classes, i, current.override_status);
                let new_level = (current.level + 1) | 1;
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    valid_isolate_count += 1;
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::Neutral,
                        isolate_status: true,
                    });
                } else {
                    overflow_isolate_count += 1;
                }
            }
            // X5b: LRI
            BidiClass::LRI => {
                levels[i] = current.level;
                apply_override(classes, i, current.override_status);
                let new_level = (current.level + 2) & !1;
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    valid_isolate_count += 1;
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::Neutral,
                        isolate_status: true,
                    });
                } else {
                    overflow_isolate_count += 1;
                }
            }
            // X5c: FSI
            BidiClass::FSI => {
                let inner_level = determine_paragraph_level_for_fsi(classes, i + 1);
                levels[i] = current.level;
                apply_override(classes, i, current.override_status);
                let new_level = if inner_level == 1 {
                    (current.level + 1) | 1 // as RLI
                } else {
                    (current.level + 2) & !1 // as LRI
                };
                if new_level <= MAX_DEPTH
                    && overflow_isolate_count == 0
                    && overflow_embedding_count == 0
                {
                    valid_isolate_count += 1;
                    stack.push(DirectionalStatus {
                        level: new_level,
                        override_status: Override::Neutral,
                        isolate_status: true,
                    });
                } else {
                    overflow_isolate_count += 1;
                }
            }
            // X6a: PDI
            BidiClass::PDI => {
                if overflow_isolate_count > 0 {
                    overflow_isolate_count -= 1;
                } else if valid_isolate_count > 0 {
                    overflow_embedding_count = 0;
                    while stack.len() > 1 {
                        if stack.last().unwrap().isolate_status {
                            stack.pop();
                            break;
                        }
                        stack.pop();
                    }
                    valid_isolate_count -= 1;
                }
                let cur = *stack.last().unwrap();
                levels[i] = cur.level;
                apply_override(classes, i, cur.override_status);
            }
            // X7: PDF
            BidiClass::PDF => {
                if overflow_isolate_count > 0 {
                    // do nothing
                } else if overflow_embedding_count > 0 {
                    overflow_embedding_count -= 1;
                } else if stack.len() >= 2 && !stack.last().unwrap().isolate_status {
                    stack.pop();
                }
            }
            // X8: B (paragraph separator)
            BidiClass::B => {
                levels[i] = para_level;
            }
            // X6: all other characters
            _ => {
                levels[i] = current.level;
                apply_override(classes, i, current.override_status);
            }
        }
    }
}

fn apply_override(classes: &mut [BidiClass], i: usize, ovr: Override) {
    match ovr {
        Override::LTR => classes[i] = BidiClass::L,
        Override::RTL => classes[i] = BidiClass::R,
        Override::Neutral => {}
    }
}

/// Determine paragraph level for FSI: P2/P3 applied to text following the FSI
/// until its matching PDI.
fn determine_paragraph_level_for_fsi(classes: &[BidiClass], start: usize) -> u8 {
    let mut depth: u32 = 1;
    for i in start..classes.len() {
        match classes[i] {
            BidiClass::LRI | BidiClass::RLI | BidiClass::FSI => depth += 1,
            BidiClass::PDI => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {
                if depth == 1 {
                    match classes[i] {
                        BidiClass::L => return 0,
                        BidiClass::R | BidiClass::AL => return 1,
                        _ => {}
                    }
                }
            }
        }
    }
    0
}

// ---------------------------------------------------------------------------
// X9: remove embedding/override/BN controls
// ---------------------------------------------------------------------------

fn apply_x9(
    original_classes: &[BidiClass],
    classes: &mut [BidiClass],
    levels: &mut [u8],
) -> Vec<bool> {
    let mut removed = vec![false; classes.len()];
    for i in 0..classes.len() {
        // Check original class, not the possibly-overridden class.
        match original_classes[i] {
            BidiClass::RLE
            | BidiClass::LRE
            | BidiClass::RLO
            | BidiClass::LRO
            | BidiClass::PDF
            | BidiClass::BN => {
                removed[i] = true;
                levels[i] = u8::MAX;
                classes[i] = BidiClass::BN;
            }
            _ => {}
        }
    }
    removed
}

// ---------------------------------------------------------------------------
// X10: isolating run sequences
// ---------------------------------------------------------------------------

struct IsolatingRunSequence {
    indices: Vec<usize>,
    level: u8,
    sos: BidiClass,
    eos: BidiClass,
}

fn build_isolating_run_sequences(
    original_classes: &[BidiClass],
    classes: &[BidiClass],
    levels: &[u8],
    removed: &[bool],
    para_level: u8,
) -> Vec<IsolatingRunSequence> {
    let n = classes.len();
    if n == 0 {
        return vec![];
    }

    // Build level runs: each is a maximal contiguous block of non-removed
    // characters at the same embedding level.
    let mut runs: Vec<Vec<usize>> = Vec::new();
    let mut current_run: Vec<usize> = Vec::new();
    let mut current_level: u8 = u8::MAX;

    for i in 0..n {
        if removed[i] {
            continue;
        }
        if levels[i] != current_level {
            if !current_run.is_empty() {
                runs.push(current_run.clone());
                current_run.clear();
            }
            current_level = levels[i];
        }
        current_run.push(i);
    }
    if !current_run.is_empty() {
        runs.push(current_run);
    }

    if runs.is_empty() {
        return vec![];
    }

    // Match isolate initiators to PDIs using original_classes.
    // This is needed to chain level runs into isolating run sequences.
    // initiator_to_pdi[i] = Some(j) means original_classes[i] is an isolate
    // initiator and its matching PDI is at index j.
    let mut initiator_to_pdi: Vec<Option<usize>> = vec![None; n];
    let mut pdi_to_initiator: Vec<Option<usize>> = vec![None; n];
    {
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            match original_classes[i] {
                BidiClass::LRI | BidiClass::RLI | BidiClass::FSI => {
                    stack.push(i);
                }
                BidiClass::PDI => {
                    if let Some(init) = stack.pop() {
                        initiator_to_pdi[init] = Some(i);
                        pdi_to_initiator[i] = Some(init);
                    }
                }
                _ => {}
            }
        }
    }

    // Build a map: for each run, which run index does it belong to?
    let mut char_to_run: Vec<usize> = vec![0; n];
    for (ri, run) in runs.iter().enumerate() {
        for &idx in run {
            char_to_run[idx] = ri;
        }
    }

    // Chain runs into isolating run sequences:
    // - A run whose first character is a PDI matching an isolate initiator is
    //   a continuation of the sequence containing that initiator's run.
    // - A run that ends with an isolate initiator continues in the run
    //   starting with the matching PDI.
    let num_runs = runs.len();
    let mut run_used: Vec<bool> = vec![false; num_runs];
    let mut sequences: Vec<IsolatingRunSequence> = Vec::new();

    for start_ri in 0..num_runs {
        if run_used[start_ri] {
            continue;
        }
        // Check if this run starts with a PDI that matches an initiator.
        // If so, it's a continuation and shouldn't start a new sequence.
        let first_char = runs[start_ri][0];
        if pdi_to_initiator[first_char].is_some() {
            // This run continues a previous run's sequence -- skip here;
            // it will be picked up when processing the initiator's run.
            continue;
        }

        // Build the chain of runs for this sequence.
        let mut chain: Vec<usize> = Vec::new();
        let mut ri = start_ri;
        loop {
            chain.push(ri);
            run_used[ri] = true;

            // Check if the last character of this run is an isolate initiator.
            let last_char = *runs[ri].last().unwrap();
            if let Some(pdi_idx) = initiator_to_pdi[last_char] {
                // The next run in the sequence is the one containing the PDI.
                if !removed[pdi_idx] {
                    let next_ri = char_to_run[pdi_idx];
                    if !run_used[next_ri] {
                        ri = next_ri;
                        continue;
                    }
                }
            }
            break;
        }

        // Gather all indices from the chained runs.
        let mut indices: Vec<usize> = Vec::new();
        for &cri in &chain {
            indices.extend_from_slice(&runs[cri]);
        }

        if indices.is_empty() {
            continue;
        }

        let run_level = levels[indices[0]];

        // sos: max(run_level, level of char before first char of first run in
        // the sequence, or para_level).
        let first_idx = indices[0];
        let prev_level = find_preceding_level(first_idx, levels, removed, para_level);
        let sos_level = run_level.max(prev_level);
        let sos = if sos_level % 2 == 0 {
            BidiClass::L
        } else {
            BidiClass::R
        };

        // eos: max(run_level, level of "following" character).
        // If last char is an isolate initiator:
        //   - With matching PDI: use the PDI's level.
        //   - Without matching PDI: use para_level (content inside the isolate
        //     is not "following" in the same embedding context).
        // Otherwise: level of the next non-removed character, or para_level.
        let last_idx = *indices.last().unwrap();
        let last_is_isolate_init =
            matches!(original_classes[last_idx], BidiClass::LRI | BidiClass::RLI | BidiClass::FSI);
        let next_level = if last_is_isolate_init {
            if let Some(pdi_idx) = initiator_to_pdi[last_idx] {
                levels[pdi_idx]
            } else {
                para_level
            }
        } else {
            find_following_level(last_idx, levels, removed, para_level)
        };
        let eos_level = run_level.max(if next_level == u8::MAX {
            para_level
        } else {
            next_level
        });
        let eos = if eos_level % 2 == 0 {
            BidiClass::L
        } else {
            BidiClass::R
        };

        sequences.push(IsolatingRunSequence {
            indices,
            level: run_level,
            sos,
            eos,
        });
    }

    // Mark any remaining unchained runs (PDI-starting runs whose initiators
    // weren't found or were removed) as their own sequences.
    for ri in 0..num_runs {
        if run_used[ri] {
            continue;
        }
        run_used[ri] = true;
        let indices = runs[ri].clone();
        if indices.is_empty() {
            continue;
        }
        let run_level = levels[indices[0]];
        let first_idx = indices[0];
        let last_idx = *indices.last().unwrap();
        let prev_level = find_preceding_level(first_idx, levels, removed, para_level);
        let next_level = find_following_level(last_idx, levels, removed, para_level);
        let sos_level = run_level.max(prev_level);
        let eos_level = run_level.max(if next_level == u8::MAX {
            para_level
        } else {
            next_level
        });
        let sos = if sos_level % 2 == 0 {
            BidiClass::L
        } else {
            BidiClass::R
        };
        let eos = if eos_level % 2 == 0 {
            BidiClass::L
        } else {
            BidiClass::R
        };
        sequences.push(IsolatingRunSequence {
            indices,
            level: run_level,
            sos,
            eos,
        });
    }

    sequences
}

fn find_preceding_level(idx: usize, levels: &[u8], removed: &[bool], para_level: u8) -> u8 {
    if idx == 0 {
        return para_level;
    }
    for i in (0..idx).rev() {
        if !removed[i] && levels[i] != u8::MAX {
            return levels[i];
        }
    }
    para_level
}

fn find_following_level(idx: usize, levels: &[u8], removed: &[bool], para_level: u8) -> u8 {
    for i in (idx + 1)..levels.len() {
        if !removed[i] && levels[i] != u8::MAX {
            return levels[i];
        }
    }
    para_level
}

// ---------------------------------------------------------------------------
// W1-W7: weak type resolution
// ---------------------------------------------------------------------------

fn resolve_weak_types(sc: &mut [BidiClass], sos: BidiClass) {
    let n = sc.len();
    if n == 0 {
        return;
    }

    // W1: NSM gets the type of the preceding character (or sos).
    // Isolate initiators and PDI are treated as ON for this rule.
    {
        let mut prev_type = sos;
        for i in 0..n {
            if sc[i] == BidiClass::NSM {
                sc[i] = match prev_type {
                    BidiClass::LRI | BidiClass::RLI | BidiClass::FSI | BidiClass::PDI => {
                        BidiClass::ON
                    }
                    _ => prev_type,
                };
            }
            prev_type = sc[i];
        }
    }

    // W2: EN preceded by AL (past non-strong types) becomes AN.
    {
        let mut last_strong = sos;
        for i in 0..n {
            match sc[i] {
                BidiClass::L | BidiClass::R | BidiClass::AL => last_strong = sc[i],
                BidiClass::EN => {
                    if last_strong == BidiClass::AL {
                        sc[i] = BidiClass::AN;
                    }
                }
                _ => {}
            }
        }
    }

    // W3: AL -> R.
    for i in 0..n {
        if sc[i] == BidiClass::AL {
            sc[i] = BidiClass::R;
        }
    }

    // W4: Single ES between EN+EN -> EN; single CS between EN+EN -> EN;
    //     single CS between AN+AN -> AN.
    for i in 1..n.saturating_sub(1) {
        if sc[i] == BidiClass::ES
            && sc[i - 1] == BidiClass::EN
            && sc[i + 1] == BidiClass::EN
        {
            sc[i] = BidiClass::EN;
        } else if sc[i] == BidiClass::CS {
            if sc[i - 1] == BidiClass::EN && sc[i + 1] == BidiClass::EN {
                sc[i] = BidiClass::EN;
            } else if sc[i - 1] == BidiClass::AN && sc[i + 1] == BidiClass::AN {
                sc[i] = BidiClass::AN;
            }
        }
    }

    // W5: Sequence of ETs adjacent to EN -> EN.
    {
        let mut prev_en = false;
        for i in 0..n {
            match sc[i] {
                BidiClass::EN => prev_en = true,
                BidiClass::ET if prev_en => sc[i] = BidiClass::EN,
                _ => prev_en = false,
            }
        }
        let mut next_en = false;
        for i in (0..n).rev() {
            match sc[i] {
                BidiClass::EN => next_en = true,
                BidiClass::ET if next_en => sc[i] = BidiClass::EN,
                _ => next_en = false,
            }
        }
    }

    // W6: Remaining ES, ET, CS -> ON.
    for i in 0..n {
        if matches!(sc[i], BidiClass::ES | BidiClass::ET | BidiClass::CS) {
            sc[i] = BidiClass::ON;
        }
    }

    // W7: EN preceded by L (past non-L/R types) becomes L.
    {
        let mut last_strong = sos;
        for i in 0..n {
            match sc[i] {
                BidiClass::L | BidiClass::R => last_strong = sc[i],
                BidiClass::EN => {
                    if last_strong == BidiClass::L {
                        sc[i] = BidiClass::L;
                    }
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// N0: bracket pair resolution (BD16)
// ---------------------------------------------------------------------------

fn resolve_bracket_pairs(
    sc: &mut [BidiClass],
    indices: &[usize],
    cps: &[u32],
    _levels: &[u8],
    embed_dir: BidiClass,
    sos: BidiClass,
    pre_w_nsm: &[bool],
) {
    let n = sc.len();
    if n == 0 {
        return;
    }

    // BD16: find bracket pairs.
    // Per spec: "the Bidi_Paired_Bracket_Type property is considered only
    // for characters whose *current* Bidi_Class is ON."
    // Also: bracket pairs consider canonical equivalents (e.g. U+2329 = U+3008).
    let mut open_stack: Vec<(usize, u32)> = Vec::new(); // (idx in sc, canonical cp)
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for j in 0..n {
        if sc[j] != BidiClass::ON {
            continue;
        }
        // Canonicalize the code point for bracket matching.
        let raw_cp = cps[indices[j]];
        let cp = canonical_bracket_cp(raw_cp);

        if bracket_pair(cp).is_some() {
            // Opening bracket.
            if open_stack.len() >= 63 {
                // BD16: "otherwise, stop processing." Abort bracket pairing.
                break;
            }
            open_stack.push((j, cp));
        } else if is_closing_bracket(cp) {
            if let Some(open_cp) = opening_bracket_for(cp) {
                let mut found = None;
                for k in (0..open_stack.len()).rev() {
                    if open_stack[k].1 == open_cp {
                        found = Some(k);
                        break;
                    }
                }
                if let Some(k) = found {
                    let open_idx = open_stack[k].0;
                    pairs.push((open_idx, j));
                    open_stack.truncate(k);
                }
            }
        }
    }

    // Sort pairs by opening index.
    pairs.sort_by_key(|&(o, _)| o);

    let opposite = if embed_dir == BidiClass::L {
        BidiClass::R
    } else {
        BidiClass::L
    };

    for &(open_idx, close_idx) in &pairs {
        // Classify strong types enclosed within the bracket pair.
        let mut found_same = false;
        let mut found_opposite = false;

        for k in (open_idx + 1)..close_idx {
            match strong_type_for_n0(sc[k]) {
                Some(d) if d == embed_dir => found_same = true,
                Some(_) => found_opposite = true,
                None => {}
            }
        }

        let bracket_dir = if found_same {
            // N0b: strong type matching embedding direction found -> embed_dir.
            embed_dir
        } else if found_opposite {
            // N0c: only opposite found -> check context before opening bracket.
            let mut context_dir = sos;
            for k in (0..open_idx).rev() {
                if let Some(d) = strong_type_for_n0(sc[k]) {
                    context_dir = d;
                    break;
                }
            }
            if context_dir == opposite {
                opposite
            } else {
                embed_dir
            }
        } else {
            // N0d: no strong types inside -> leave brackets unchanged.
            continue;
        };

        sc[open_idx] = bracket_dir;
        sc[close_idx] = bracket_dir;

        // Any characters that were originally NSM (before W1) immediately
        // following the opening or closing bracket take the bracket's type.
        let mut k = open_idx + 1;
        while k < n && pre_w_nsm[k] {
            sc[k] = bracket_dir;
            k += 1;
        }
        k = close_idx + 1;
        while k < n && pre_w_nsm[k] {
            sc[k] = bracket_dir;
            k += 1;
        }
    }
}

/// For BD16 bracket pairing, canonicalize code points that have a single-
/// character canonical decomposition to a bracket.  This handles the
/// U+2329/U+232A -> U+3008/U+3009 equivalence required by the spec.
fn canonical_bracket_cp(cp: u32) -> u32 {
    use crate::data::normalization::canonical_decomposition;
    if let Some(decomp) = canonical_decomposition(cp) {
        if decomp.len() == 1 {
            let d = decomp[0];
            // Only use the decomposed form if it's itself a bracket.
            if bracket_pair(d).is_some() || is_closing_bracket(d) {
                return d;
            }
        }
    }
    cp
}

fn strong_type_for_n0(bc: BidiClass) -> Option<BidiClass> {
    match bc {
        BidiClass::L => Some(BidiClass::L),
        BidiClass::R | BidiClass::AL | BidiClass::AN | BidiClass::EN => Some(BidiClass::R),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// N1-N2: neutral type resolution
// ---------------------------------------------------------------------------

fn resolve_neutral_types(
    sc: &mut [BidiClass],
    sos: BidiClass,
    eos: BidiClass,
    embed_dir: BidiClass,
) {
    let n = sc.len();
    if n == 0 {
        return;
    }

    fn is_ni(bc: BidiClass) -> bool {
        matches!(
            bc,
            BidiClass::B
                | BidiClass::S
                | BidiClass::WS
                | BidiClass::ON
                | BidiClass::FSI
                | BidiClass::LRI
                | BidiClass::RLI
                | BidiClass::PDI
        )
    }

    fn to_strong(bc: BidiClass) -> BidiClass {
        match bc {
            BidiClass::L => BidiClass::L,
            BidiClass::R | BidiClass::AN | BidiClass::EN => BidiClass::R,
            _ => bc,
        }
    }

    let mut i = 0;
    while i < n {
        if is_ni(sc[i]) {
            let start = i;
            while i < n && is_ni(sc[i]) {
                i += 1;
            }

            // N1: type before and after the run.
            let before = if start > 0 {
                to_strong(sc[start - 1])
            } else {
                to_strong(sos)
            };
            let after = if i < n {
                to_strong(sc[i])
            } else {
                to_strong(eos)
            };

            let resolved = if before == after
                && (before == BidiClass::L || before == BidiClass::R)
            {
                before // N1
            } else {
                embed_dir // N2
            };

            for j in start..i {
                sc[j] = resolved;
            }
        } else {
            i += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// I1-I2: implicit level resolution
// ---------------------------------------------------------------------------

fn resolve_implicit_levels(sc: &[BidiClass], indices: &[usize], levels: &mut [u8]) {
    for (j, &i) in indices.iter().enumerate() {
        let level = levels[i];
        if level == u8::MAX {
            continue;
        }
        if level % 2 == 0 {
            // I1: even level
            match sc[j] {
                BidiClass::R => levels[i] = level + 1,
                BidiClass::AN | BidiClass::EN => levels[i] = level + 2,
                _ => {}
            }
        } else {
            // I2: odd level
            match sc[j] {
                BidiClass::L | BidiClass::EN | BidiClass::AN => levels[i] = level + 1,
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// L1: reset levels
// ---------------------------------------------------------------------------

fn apply_l1(original_classes: &[BidiClass], levels: &mut [u8], para_level: u8) {
    let n = original_classes.len();
    let mut reset = true; // start from end of line
    for i in (0..n).rev() {
        match original_classes[i] {
            BidiClass::B | BidiClass::S => {
                levels[i] = para_level;
                reset = true;
            }
            BidiClass::WS
            | BidiClass::FSI
            | BidiClass::LRI
            | BidiClass::RLI
            | BidiClass::PDI => {
                if reset {
                    levels[i] = para_level;
                }
            }
            BidiClass::BN
            | BidiClass::RLE
            | BidiClass::LRE
            | BidiClass::RLO
            | BidiClass::LRO
            | BidiClass::PDF => {
                if reset {
                    levels[i] = para_level;
                }
            }
            _ => {
                reset = false;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// L2: reorder
// ---------------------------------------------------------------------------

fn compute_reorder(levels: &[u8]) -> Vec<usize> {
    let n = levels.len();
    if n == 0 {
        return vec![];
    }

    // Collect non-removed characters.
    let mut items: Vec<(usize, u8)> = Vec::new();
    for i in 0..n {
        if levels[i] != u8::MAX {
            items.push((i, levels[i]));
        }
    }
    if items.is_empty() {
        return vec![];
    }

    let max_level = items.iter().map(|&(_, l)| l).max().unwrap_or(0);
    let min_odd = {
        let m = items.iter().map(|&(_, l)| l).min().unwrap_or(0);
        if m % 2 == 1 {
            m
        } else {
            m + 1
        }
    };

    let mut order: Vec<usize> = items.iter().map(|&(i, _)| i).collect();
    let item_levels: Vec<u8> = items.iter().map(|&(_, l)| l).collect();

    for level in (min_odd..=max_level).rev() {
        let mut i = 0;
        while i < order.len() {
            if item_levels[i] >= level {
                let start = i;
                while i < order.len() && item_levels[i] >= level {
                    i += 1;
                }
                order[start..i].reverse();
            } else {
                i += 1;
            }
        }
    }

    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_ltr() {
        let info = resolve("hello", None);
        assert_eq!(info.paragraph_level, 0);
        assert_eq!(info.levels, vec![0, 0, 0, 0, 0]);
        assert_eq!(info.reorder, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_simple_rtl() {
        let info = resolve("\u{05D0}\u{05D1}\u{05D2}", None);
        assert_eq!(info.paragraph_level, 1);
        assert_eq!(info.levels, vec![1, 1, 1]);
        assert_eq!(info.reorder, vec![2, 1, 0]);
    }

    #[test]
    fn test_mixed_ltr_rtl() {
        // "abc" followed by 3 Hebrew chars
        let info = resolve("abc\u{05D0}\u{05D1}\u{05D2}", None);
        assert_eq!(info.paragraph_level, 0);
        assert_eq!(info.levels, vec![0, 0, 0, 1, 1, 1]);
        assert_eq!(info.reorder, vec![0, 1, 2, 5, 4, 3]);
    }

    #[test]
    fn test_empty() {
        let info = resolve("", None);
        assert_eq!(info.paragraph_level, 0);
        assert!(info.levels.is_empty());
        assert!(info.reorder.is_empty());
    }
}
