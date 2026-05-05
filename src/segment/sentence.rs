//! UAX #29: Sentence boundary detection.
//!
//! Used for sentence-level operations, text-to-speech segmentation, and
//! abbreviation-aware sentence splitting.
//!
//! Implements default sentence boundary rules (SB1-SB998); locale tailoring
//! reserved for future.
//!
//! Architecture:
//!   1. Pre-collect characters, their SB properties, and byte offsets.
//!   2. For each adjacent pair, apply SB3-SB4 on RAW properties.
//!   3. SB5 (Extend/Format transparency): do not break before transparent chars.
//!   4. For SB6-SB998: track ATerm/STerm context state and apply resolved rules.

use crate::data::sentence_break::{sb, Sb};

/// Context state for tracking ATerm/STerm sequences (SB6-SB11).
#[derive(Clone, Copy, PartialEq)]
enum SbCtx {
    /// Not in any sentence-terminator context.
    None,
    /// After SATerm (STerm|ATerm), possibly followed by Close*.
    /// `is_aterm`: true if the trigger was ATerm, false if STerm.
    /// `after_close`: true if Close characters have been seen after the SATerm.
    /// SB6/SB7 only apply when after_close is false (ATerm is immediate resolved prev).
    SATerm { is_aterm: bool, after_close: bool },
    /// After SATerm Close* Sp+.
    SATermSp { is_aterm: bool },
}

/// Returns true if this Sb value is transparent per SB5 (Extend or Format).
fn is_sb_transparent(s: Sb) -> bool {
    matches!(s, Sb::Extend | Sb::Format)
}

/// SB8 lookahead: starting from position `start`, scan forward in the resolved
/// (non-transparent) stream to see if the first "significant" character
/// (OLetter/Upper/Lower/ParaSep/SATerm) is Lower.
///
/// Characters that are NOT significant (Numeric, Close, SContinue, Sp, Other)
/// are skipped. Transparent characters (Extend/Format) are also skipped.
///
/// Returns true if Lower is found before any other significant character.
fn sb8_lookahead_lower(sbs: &[Sb], start: usize) -> bool {
    for s in sbs.iter().skip(start) {
        if is_sb_transparent(*s) {
            continue;
        }
        match s {
            Sb::Lower => return true,
            Sb::OLetter | Sb::Upper | Sb::Sep | Sb::Cr | Sb::Lf | Sb::STerm | Sb::ATerm => {
                return false;
            }
            _ => continue, // Numeric, Close, SContinue, Sp, Other
        }
    }
    false
}

/// Returns the byte offsets of sentence boundaries in `s` (start of each sentence).
///
/// Boundaries are the start of each sentence; the first boundary is always 0.
/// Implements UAX #29 default sentence boundary rules (SB1-SB998).
/// `locale` reserved for future tailoring.
#[must_use]
pub fn sentence_boundaries(s: &str, _locale: Option<&str>) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return vec![0];
    }

    // Pre-compute SB property and byte offset for each character
    let sbs: Vec<Sb> = chars.iter().map(|&c| sb(c)).collect();
    let offsets: Vec<usize> = {
        let mut v = Vec::with_capacity(n);
        let mut off = 0usize;
        for &c in &chars {
            v.push(off);
            off += c.len_utf8();
        }
        v
    };

    let mut out = vec![0]; // SB1: sot boundary
    let mut ctx = SbCtx::None;
    let mut before_saterm: Option<Sb> = Option::None;
    let mut last_resolved: Option<Sb> = if !is_sb_transparent(sbs[0]) {
        Some(sbs[0])
    } else {
        Option::None
    };

    // Initialize ctx for first character
    if !is_sb_transparent(sbs[0]) {
        match sbs[0] {
            Sb::ATerm => {
                before_saterm = Option::None;
                ctx = SbCtx::SATerm {
                    is_aterm: true,
                    after_close: false,
                };
            }
            Sb::STerm => {
                before_saterm = Option::None;
                ctx = SbCtx::SATerm {
                    is_aterm: false,
                    after_close: false,
                };
            }
            _ => {}
        }
    }

    for i in 1..n {
        let prev_raw = sbs[i - 1];
        let next_raw = sbs[i];

        // SB3: CR x LF
        if prev_raw == Sb::Cr && next_raw == Sb::Lf {
            continue; // no break, ctx unchanged
        }

        // SB4: (Sep|CR|LF) /
        if matches!(prev_raw, Sb::Sep | Sb::Cr | Sb::Lf) {
            out.push(offsets[i]);
            ctx = SbCtx::None;
            // Update tracking for the new sentence
            if is_sb_transparent(next_raw) {
                last_resolved = Option::None;
            } else {
                last_resolved = Some(next_raw);
                match next_raw {
                    Sb::ATerm => {
                        before_saterm = Option::None;
                        ctx = SbCtx::SATerm {
                            is_aterm: true,
                            after_close: false,
                        };
                    }
                    Sb::STerm => {
                        before_saterm = Option::None;
                        ctx = SbCtx::SATerm {
                            is_aterm: false,
                            after_close: false,
                        };
                    }
                    _ => {}
                }
            }
            continue;
        }

        // SB5: x (Extend|Format)
        if is_sb_transparent(next_raw) {
            continue; // no break, ctx and last_resolved unchanged
        }

        // --- next_raw is non-transparent. Apply SB6-SB998 based on ctx. ---
        let next = next_raw;

        let should_break = match ctx {
            SbCtx::SATerm {
                is_aterm,
                after_close,
            } => {
                if is_aterm && !after_close && next == Sb::Numeric {
                    // SB6: ATerm x Numeric (e.g. "3.4") - ATerm must be immediate prev
                    false
                } else if is_aterm
                    && !after_close
                    && next == Sb::Upper
                    && matches!(before_saterm, Some(Sb::Upper) | Some(Sb::Lower))
                {
                    // SB7: (Upper|Lower) ATerm x Upper (e.g. "U.S.") - ATerm must be immediate
                    false
                } else if is_aterm && sb8_lookahead_lower(&sbs, i) {
                    // SB8: ATerm Close* Sp* x ([^OLetter Upper Lower ParaSep SATerm])* Lower
                    // (ATerm context in Close* phase)
                    false
                } else if matches!(next, Sb::SContinue | Sb::STerm | Sb::ATerm) {
                    // SB8a: SATerm Close* Sp* x (SContinue|STerm|ATerm)
                    false
                } else if matches!(next, Sb::Close | Sb::Sp | Sb::Sep | Sb::Cr | Sb::Lf) {
                    // SB9: SATerm Close* x (Close|Sp|ParaSep)
                    false
                } else {
                    // SB11: SATerm Close* Sp* ParaSep? /
                    true
                }
            }
            SbCtx::SATermSp { is_aterm } => {
                if is_aterm && sb8_lookahead_lower(&sbs, i) {
                    // SB8: ATerm Close* Sp* x ([^...])*  Lower
                    false
                } else if matches!(next, Sb::SContinue | Sb::STerm | Sb::ATerm) {
                    // SB8a
                    false
                } else if matches!(next, Sb::Sp | Sb::Sep | Sb::Cr | Sb::Lf) {
                    // SB10: SATerm Close* Sp* x (Sp|ParaSep)
                    false
                } else {
                    // SB11
                    true
                }
            }
            SbCtx::None => {
                // SB998: Any x Any (default no break)
                false
            }
        };

        if should_break {
            out.push(offsets[i]);
        }

        // Update ctx for the next position
        match next {
            Sb::ATerm => {
                before_saterm = last_resolved;
                ctx = SbCtx::SATerm {
                    is_aterm: true,
                    after_close: false,
                };
            }
            Sb::STerm => {
                before_saterm = last_resolved;
                ctx = SbCtx::SATerm {
                    is_aterm: false,
                    after_close: false,
                };
            }
            Sb::Close => {
                if !should_break && matches!(ctx, SbCtx::SATerm { .. }) {
                    // Close continues the Close* sequence; mark after_close
                    if let SbCtx::SATerm { is_aterm, .. } = ctx {
                        ctx = SbCtx::SATerm {
                            is_aterm,
                            after_close: true,
                        };
                    }
                } else {
                    ctx = SbCtx::None;
                }
            }
            Sb::Sp => match ctx {
                SbCtx::SATerm { is_aterm, .. } if !should_break => {
                    ctx = SbCtx::SATermSp { is_aterm };
                }
                SbCtx::SATermSp { .. } if !should_break => {
                    // Continue Sp* accumulation
                }
                _ => {
                    ctx = SbCtx::None;
                }
            },
            _ => {
                // Any other character resets the SATerm context
                ctx = SbCtx::None;
            }
        }

        last_resolved = Some(next);
    }

    out
}
