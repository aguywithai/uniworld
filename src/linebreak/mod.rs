//! UAX #14: Unicode Line Breaking Algorithm (Unicode 17.0).
//!
//! Determines permissible line break opportunities in Unicode text.
//! Implements all non-tailorable rules (LB1-LB9, LB10) and tailorable
//! rules (LB11-LB31) per the default specification.
//!
//! SA (Complex Context / Southeast Asian) characters support dictionary-
//! based segmentation for Thai, Lao, Khmer, and Myanmar. Without
//! dictionary support, SA characters are treated as AL (or CM based on
//! General_Category) per the default behavior.

pub mod dictionary;

use crate::data::line_break::{lb, is_east_asian_wide, Lb};

/// Whether a break is mandatory, optional (allowed), or prohibited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakAction {
    /// A mandatory break (hard line break).
    Mandatory,
    /// A break opportunity (line may break here).
    Allowed,
    /// No break permitted at this position.
    Prohibited,
}

/// Find all line break opportunities in `text`.
///
/// Returns a `Vec<BreakAction>` of length `text.len() + 1`. Index 0 is before
/// the first byte; index `text.len()` is after the last byte. Breaks are only
/// meaningful at code point (char) boundaries.
///
/// For most callers, iterate char boundaries and check the corresponding index.
pub fn line_break_opportunities(text: &str) -> Vec<BreakAction> {
    let len = text.len();
    let mut breaks = vec![BreakAction::Prohibited; len + 1];

    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let n = chars.len();
    if n == 0 {
        // LB2: sot x   (never break at start of empty text)
        // LB3: ! eot
        breaks[0] = BreakAction::Mandatory;
        return breaks;
    }

    // Build resolved classes per LB1, LB9, LB10.
    let cps: Vec<u32> = chars.iter().map(|(_, ch)| *ch as u32).collect();
    let classes: Vec<Lb> = cps.iter().map(|&cp| resolve_lb1(cp)).collect();

    // LB2: never break at start of text.
    breaks[chars[0].0] = BreakAction::Prohibited;

    // LB3: always break at end of text.
    breaks[len] = BreakAction::Mandatory;

    if n == 1 {
        return breaks;
    }

    // Build the "resolved" class array after LB9 (combining mark absorption)
    // and LB10 (remaining CM/ZWJ -> AL).
    // LB9: X (CM|ZWJ)* is treated as X, where X is not BK/CR/LF/NL/SP/ZW.
    // We store the "effective" class for each position.
    let mut effective: Vec<Lb> = classes.clone();
    {
        let mut base_idx: Option<usize> = None;
        for i in 0..n {
            let c = classes[i];
            if c == Lb::CM || c == Lb::ZWJ {
                if let Some(bi) = base_idx {
                    let base_c = classes[bi];
                    if !is_hard_break_or_space(base_c) {
                        // This CM/ZWJ inherits the base class for rule matching.
                        effective[i] = effective[bi];
                        continue;
                    }
                }
                // LB10: remaining CM/ZWJ treated as AL.
                effective[i] = Lb::AL;
            } else {
                base_idx = Some(i);
            }
        }
    }

    // Precompute: for each position i, the "non-CM/ZWJ" effective class
    // looking backwards (the class of the base character before CM/ZWJ run).
    // Also precompute forward skip over CM/ZWJ for space-handling rules.

    // Now apply rules LB4-LB31 for each break position between chars[i] and chars[i+1].
    // We track state needed for multi-character lookahead rules.

    // Track ZW state for LB8.
    let mut after_zw_sp = false;

    // (LB30a RI counting is done via backward walk, no state variable needed.)

    for i in 0..n {
        let byte_pos = if i + 1 < n {
            chars[i + 1].0
        } else {
            len
        };

        // The break position is between chars[i] and chars[i+1].
        // If i+1 >= n, this is the position before eot (already set to Mandatory by LB3).
        if i + 1 >= n {
            break;
        }

        let cls_before = classes[i]; // raw class of char at i
        let cls_after = classes[i + 1]; // raw class of char at i+1
        let eff_before = effective[i]; // effective (after LB9) class of char at i
        let eff_after = effective[i + 1]; // effective class of char at i+1

        // LB4: BK !
        if cls_before == Lb::BK {
            breaks[byte_pos] = BreakAction::Mandatory;
            after_zw_sp = false;
            continue;
        }

        // LB5: CR x LF
        if cls_before == Lb::CR && cls_after == Lb::LF {
            breaks[byte_pos] = BreakAction::Prohibited;
            after_zw_sp = false;
            continue;
        }
        // LB5: CR !, LF !, NL !
        if cls_before == Lb::CR || cls_before == Lb::LF || cls_before == Lb::NL {
            breaks[byte_pos] = BreakAction::Mandatory;
            after_zw_sp = false;
            continue;
        }

        // LB6: x (BK|CR|LF|NL)
        if cls_after == Lb::BK
            || cls_after == Lb::CR
            || cls_after == Lb::LF
            || cls_after == Lb::NL
        {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB7: x SP, x ZW
        if cls_after == Lb::SP || cls_after == Lb::ZW {
            breaks[byte_pos] = BreakAction::Prohibited;
            // Track ZW SP* sequence for LB8.
            // after_zw_sp = true means we are inside a ZW SP* sequence.
            if cls_after == Lb::ZW || cls_before == Lb::ZW || after_zw_sp {
                after_zw_sp = true;
            } else {
                after_zw_sp = false;
            }
            continue;
        }

        // LB8: ZW SP* ÷
        if after_zw_sp {
            breaks[byte_pos] = BreakAction::Allowed;
            after_zw_sp = false;
            continue;
        }
        // Also check: if cls_before == ZW (no SP intervening), break after.
        if cls_before == Lb::ZW {
            breaks[byte_pos] = BreakAction::Allowed;
            continue;
        }

        // LB8a: ZWJ x
        if cls_before == Lb::ZWJ {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB9: Do not break a combining character sequence.
        // X (CM|ZWJ)* is treated as X. Don't break between X and CM/ZWJ.
        if (cls_after == Lb::CM || cls_after == Lb::ZWJ)
            && !is_hard_break_or_space(eff_before)
        {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB10: Treat remaining CM/ZWJ as AL (handled in effective array).

        // From here, use effective classes for rule matching.
        // But we need to be careful: CM/ZWJ at the break point (i+1) that
        // absorbed into a base at i should not trigger rules for the raw class.
        // The effective array handles this.

        // For rules that reference the class "before" the break:
        // We need the effective class looking back past CM/ZWJ.
        let eb = eff_before;
        // For rules that reference the class "after" the break:
        let ea = eff_after;

        // LB9 interaction: if cls_after is CM or ZWJ, the effective class
        // of i+1 already reflects the base, so we use eff_after. But if
        // the raw class is CM/ZWJ and the base is a hard break/space, then
        // eff_after is AL (from LB10), which is correct.

        // LB11: x WJ, WJ x
        if ea == Lb::WJ || eb == Lb::WJ {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB12: GL x
        if eb == Lb::GL {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB12a: [^SP BA HY HH] x GL
        // HH is excepted because, like BA and HY, it is a "break after" class.
        if ea == Lb::GL {
            if eb != Lb::SP && eb != Lb::BA && eb != Lb::HY && eb != Lb::HH {
                breaks[byte_pos] = BreakAction::Prohibited;

                continue;
            }
        }

        // LB13: x CL, x CP, x EX, x SY  (IS moved to LB15.3/15.4)
        if ea == Lb::CL
            || ea == Lb::CP
            || ea == Lb::EX
            || ea == Lb::SY
        {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB15.3: SP / IS NU  (allow break before IS when preceded by SP and followed by NU)
        // LB15.4: x IS  (otherwise prohibit break before IS)
        if ea == Lb::IS {
            // Check for LB15.3 exception: SP / IS NU
            if cls_before == Lb::SP && i + 2 < n && effective[i + 2] == Lb::NU {
                breaks[byte_pos] = BreakAction::Allowed;
                continue;
            }
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB14: OP SP* x
        // Need to look back past spaces to find if there's an OP.
        if is_after_op_sp(&effective, &classes, i) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB15a: (sot | BK | CR | LF | NL | OP | QU | GL | SP | ZW)
        //        QU_Pi SP* x
        // Look back to see if there's a QU with Pi property, preceded by
        // appropriate context, with possible SP* between.
        if is_after_qu_pi_sp(&effective, &classes, &cps, i, &chars) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB15b: x QU_Pf (sot | BK | CR | LF | NL | SP | GL | WJ | CL | QU |
        //        CP | EX | IS | SY | ZW | eot)
        // If the char after is QU with Pf property, and the char after *that*
        // is one of the listed classes (or eot), don't break before the QU.
        if ea == Lb::QU && is_qu_pf(cps[i + 1]) {
            if is_followed_by_lb15b_context(&classes, i + 1, n) {
                breaks[byte_pos] = BreakAction::Prohibited;

                continue;
            }
        }

        // LB16: (CL|CP) SP* x NS
        if ea == Lb::NS && is_after_cl_cp_sp(&effective, &classes, i) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB17: B2 SP* x B2
        if ea == Lb::B2 && is_after_b2_sp(&effective, &classes, i) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB18: SP ÷
        if cls_before == Lb::SP {
            breaks[byte_pos] = BreakAction::Allowed;

            continue;
        }

        // LB19: Context-sensitive quotation mark rules (Unicode 17.0).
        // 19.01: x QUmPi  (don't break before QU that is NOT Pi)
        if ea == Lb::QU && !is_qu_pi(cps[i + 1]) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // 19.02: QUmPf x  (don't break after QU that is NOT Pf)
        if eb == Lb::QU && !is_qu_pf(cps[find_base_index(&classes, i)]) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // 19.1: [^EastAsian] x QU
        if ea == Lb::QU && !is_east_asian_wide(cps[find_base_index(&classes, i)]) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // 19.11: x QU ([^EastAsian] | eot)
        if ea == Lb::QU {
            let qu_followed_by_ea = if i + 2 < n {
                is_east_asian_wide(cps[i + 2])
            } else {
                false // eot counts as [^EastAsian]
            };
            if !qu_followed_by_ea {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
        }
        // 19.12: QU x [^EastAsian]
        if eb == Lb::QU && !is_east_asian_wide(cps[i + 1]) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // 19.13: ([^EastAsian] | sot) QU x
        if eb == Lb::QU {
            let qu_base_idx = find_base_index(&classes, i);
            let preceded_by_ea = if qu_base_idx > 0 {
                is_east_asian_wide(cps[qu_base_idx - 1])
            } else {
                false // sot counts as [^EastAsian]
            };
            if !preceded_by_ea {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
        }

        // LB20: ÷ CB, CB ÷
        if eb == Lb::CB || ea == Lb::CB {
            breaks[byte_pos] = BreakAction::Allowed;

            continue;
        }

        // LB20a: (sot|BK|CR|LF|NL|OP|QU|GL|SP|ZW|CB) (HY|HH) x (AL|HL)
        // Word-initial hyphens: don't break after HY/HH when preceded by
        // break-like context and followed by AL/HL.
        // Use find_base_index to look past absorbed CMs to the real base,
        // then check the context before that base.
        if (eb == Lb::HY || eb == Lb::HH)
            && (ea == Lb::AL || ea == Lb::HL)
        {
            let base_idx = find_base_index(&classes, i);
            let prev_class = if base_idx == 0 {
                None // sot
            } else {
                Some(effective[base_idx - 1])
            };
            let is_word_initial = match prev_class {
                None => true, // sot
                Some(c) => matches!(c,
                    Lb::BK | Lb::CR | Lb::LF | Lb::NL | Lb::SP
                    | Lb::ZW | Lb::CB | Lb::GL | Lb::OP | Lb::QU
                ),
            };
            if is_word_initial {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
        }

        // LB21: x BA, x HH, x HY, x NS, BB x
        if ea == Lb::BA || ea == Lb::HH || ea == Lb::HY || ea == Lb::NS {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        if eb == Lb::BB {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB21a: HL (HY|HH) x [^HL]
        // Don't break after HY/HH when preceded by HL, unless followed by HL.
        if (eb == Lb::HY || eb == Lb::HH) && ea != Lb::HL && i >= 1 {
            let prev_eff = effective_class_before(&effective, &classes, i - 1);
            if prev_eff == Lb::HL {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
        }

        // LB21b: SY x HL
        if eb == Lb::SY && ea == Lb::HL {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB22: x IN
        if ea == Lb::IN {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB23: (AL|HL) x NU, NU x (AL|HL)
        if (eb == Lb::AL || eb == Lb::HL) && ea == Lb::NU {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        if eb == Lb::NU && (ea == Lb::AL || ea == Lb::HL) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB23a: PR x (ID|EB|EM), (ID|EB|EM) x PO
        if eb == Lb::PR && (ea == Lb::ID || ea == Lb::EB || ea == Lb::EM) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        if (eb == Lb::ID || eb == Lb::EB || eb == Lb::EM) && ea == Lb::PO {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB24: (PR|PO) x (AL|HL), (AL|HL) x (PR|PO)
        if (eb == Lb::PR || eb == Lb::PO) && (ea == Lb::AL || ea == Lb::HL) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        if (eb == Lb::AL || eb == Lb::HL) && (ea == Lb::PR || ea == Lb::PO) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB25: Numeric context rules.
        // This is the complex numeric expression rule.
        // (PR|PO) x (OP|HY)? NU
        // NU (NU|SY|IS)* x (NU|SY|IS|CL|CP)
        // NU (NU|SY|IS)* (CL|CP)? x (PO|PR)
        if is_lb25_no_break(&effective, &classes, &cps, i, n) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB26: JL x (JL|JV|H2|H3)
        if eb == Lb::JL
            && (ea == Lb::JL || ea == Lb::JV || ea == Lb::H2 || ea == Lb::H3)
        {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // (JV|H2) x (JV|JT)
        if (eb == Lb::JV || eb == Lb::H2) && (ea == Lb::JV || ea == Lb::JT) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // (JT|H3) x JT
        if (eb == Lb::JT || eb == Lb::H3) && ea == Lb::JT {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB27: (JL|JV|JT|H2|H3) x PO
        if matches!(eb, Lb::JL | Lb::JV | Lb::JT | Lb::H2 | Lb::H3) && ea == Lb::PO
        {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // PR x (JL|JV|JT|H2|H3)
        if eb == Lb::PR
            && matches!(ea, Lb::JL | Lb::JV | Lb::JT | Lb::H2 | Lb::H3)
        {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB28: (AL|HL) x (AL|HL)
        if (eb == Lb::AL || eb == Lb::HL) && (ea == Lb::AL || ea == Lb::HL) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB28a: Aksara rules for Brahmic scripts.
        // "Aksara base" = AK | AS | dotted_circle (U+25CC has
        // Indic_Syllabic_Category=Dotted_Circle and participates in aksara rules)
        // When CM absorbed into a base, use the base's code point.
        let eb_base_cp = cps[find_base_index(&classes, i)];
        let ea_base_cp = cps[find_base_index(&classes, i + 1)];
        let eb_aksara = eb == Lb::AK || eb == Lb::AS || is_aksara_base(eb_base_cp);
        let ea_aksara = ea == Lb::AK || ea == Lb::AS || is_aksara_base(ea_base_cp);

        // [28.11] AP x (AK|AS|dotted_circle)
        if eb == Lb::AP && ea_aksara {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // [28.13] (AK|AS|dotted_circle) VI x (AK|AS|dotted_circle)
        // Use find_base_index to look past absorbed CMs to find the actual
        // VI character, then check the aksara before it.
        if ea_aksara && eb == Lb::VI {
            let vi_base = find_base_index(&classes, i);
            if vi_base > 0 {
                let prev = effective_class_before(&effective, &classes, vi_base - 1);
                let prev_cp = cps[find_base_index(&classes, vi_base - 1)];
                if prev == Lb::AK || prev == Lb::AS || is_aksara_base(prev_cp) {
                    breaks[byte_pos] = BreakAction::Prohibited;
                    continue;
                }
            }
        }
        // (AK|AS|dotted_circle) VI x VF
        if ea == Lb::VF && eb == Lb::VI {
            let vi_base = find_base_index(&classes, i);
            if vi_base > 0 {
                let prev = effective_class_before(&effective, &classes, vi_base - 1);
                let prev_cp = cps[find_base_index(&classes, vi_base - 1)];
                if prev == Lb::AK || prev == Lb::AS || is_aksara_base(prev_cp) {
                    breaks[byte_pos] = BreakAction::Prohibited;
                    continue;
                }
            }
        }
        // [28.14] (AK|AS|dotted_circle) x (AK|AS|dotted_circle) VF
        // Look ahead past CMs: if ea is aksara and the next non-CM after ea is VF,
        // prohibit break.
        if eb_aksara && ea_aksara {
            let next_non_cm = find_next_non_cm(&classes, &effective, i + 1, n);
            if next_non_cm < n && effective[next_non_cm] == Lb::VF {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
        }

        // [28.12] (AK|AS|dotted_circle) x (VF|VI)
        if eb_aksara && (ea == Lb::VF || ea == Lb::VI) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB29: IS x (AL|HL)
        if eb == Lb::IS && (ea == Lb::AL || ea == Lb::HL) {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }

        // LB30: (AL|HL|NU) x OP (where OP does not have EAW F/W/H)
        //       CP x (AL|HL|NU) (where CP does not have EAW F/W/H)
        if (eb == Lb::AL || eb == Lb::HL || eb == Lb::NU) && ea == Lb::OP {
            if !is_east_asian_wide(cps[i + 1]) {
                breaks[byte_pos] = BreakAction::Prohibited;

                continue;
            }
        }
        if eb == Lb::CP && (ea == Lb::AL || ea == Lb::HL || ea == Lb::NU) {
            if !is_east_asian_wide(cps[i]) {
                breaks[byte_pos] = BreakAction::Prohibited;

                continue;
            }
        }

        // LB30a: (RI RI)* RI x RI
        // Use backward walk to count consecutive RIs ending at position i (eb).
        // Only count positions where the ORIGINAL class is RI, not CM/ZWJ
        // that absorbed into RI.
        if eb == Lb::RI && ea == Lb::RI {
            let mut ri_before = 0u32;
            let mut j = i as isize;
            while j >= 0 && effective[j as usize] == Lb::RI {
                if classes[j as usize] == Lb::RI {
                    ri_before += 1;
                }
                j -= 1;
            }
            // ri_before includes eb. If odd, eb is unpaired -> pair with ea (no break).
            if ri_before % 2 == 1 {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
            // Even: eb already paired -> fall through to LB31.
        }

        // LB30b: EB x EM
        if eb == Lb::EB && ea == Lb::EM {
            breaks[byte_pos] = BreakAction::Prohibited;
            continue;
        }
        // LB30b: [Extended_Pictographic && Cn] x EM
        // Only unassigned (General_Category=Cn) Extended_Pictographic
        // characters participate in this rule, regardless of their resolved
        // Line_Break class. Unassigned ExtPict chars may have Lb=ID or
        // Lb=XX (resolved to AL).
        if ea == Lb::EM {
            let cp_b = cps[find_base_index(&classes, i)];
            if is_extended_pictographic(cp_b) && is_cn(cp_b) {
                breaks[byte_pos] = BreakAction::Prohibited;
                continue;
            }
        }

        // LB31: Break everywhere else.
        breaks[byte_pos] = BreakAction::Allowed;
    }

    breaks
}

/// Find all line break opportunities in `text`, with dictionary-based
/// segmentation for SA-class scripts (Thai, Lao, Khmer, Myanmar).
///
/// This extends `line_break_opportunities` by post-processing runs of
/// SA characters using dictionary word lookup to refine break positions.
/// Within a dictionary word, breaks are prohibited. Between words (at
/// word boundaries), breaks are allowed.
///
/// Returns the same format as `line_break_opportunities`.
pub fn line_break_opportunities_with_dictionary(text: &str) -> Vec<BreakAction> {
    let mut breaks = line_break_opportunities(text);
    apply_dictionary_breaks(text, &mut breaks);
    breaks
}

/// Post-process break actions to apply dictionary-based segmentation
/// on SA-class script runs (Thai, Lao, Khmer, Myanmar).
///
/// Finds contiguous runs of SA-class characters, segments each run using
/// the appropriate language dictionary, and adjusts break actions:
/// - Within a word: Prohibited
/// - At word boundaries: Allowed (unless already Mandatory)
fn apply_dictionary_breaks(text: &str, breaks: &mut [BreakAction]) {
    use crate::data::line_break::lb;
    use dictionary::{language_for_codepoint, segment_words};

    let chars: Vec<(usize, char)> = text.char_indices().collect();
    if chars.is_empty() {
        return;
    }

    // Find runs of SA characters that share the same language.
    let mut i = 0;
    while i < chars.len() {
        let (_byte_start, ch) = chars[i];
        let cp = ch as u32;
        let raw_class = lb(cp);

        if raw_class != Lb::SA {
            i += 1;
            continue;
        }

        // Determine language for this SA character.
        let lang = match language_for_codepoint(cp) {
            Some(l) => l,
            None => {
                i += 1;
                continue;
            }
        };

        // Collect the full run of SA characters with the same language.
        let run_start = i;
        let mut run_end = i + 1;
        while run_end < chars.len() {
            let next_cp = chars[run_end].1 as u32;
            let next_class = lb(next_cp);
            if next_class != Lb::SA {
                break;
            }
            match language_for_codepoint(next_cp) {
                Some(l) if l == lang => {
                    run_end += 1;
                }
                _ => break,
            }
        }

        // Extract the text slice for this SA run.
        let byte_run_start = chars[run_start].0;
        let byte_run_end = if run_end < chars.len() {
            chars[run_end].0
        } else {
            text.len()
        };
        let run_text = &text[byte_run_start..byte_run_end];

        // Segment using dictionary.
        let word_boundaries = segment_words(run_text, lang);

        // Prohibit breaks within the run by default.
        for j in (run_start + 1)..run_end {
            let byte_pos = chars[j].0;
            if breaks[byte_pos] != BreakAction::Mandatory {
                breaks[byte_pos] = BreakAction::Prohibited;
            }
        }

        // Allow breaks at word boundaries.
        for &boundary_offset in &word_boundaries {
            let abs_byte = byte_run_start + boundary_offset;
            if abs_byte < text.len() && breaks[abs_byte] != BreakAction::Mandatory {
                breaks[abs_byte] = BreakAction::Allowed;
            }
        }

        i = run_end;
    }
}

/// Resolve LB1: map AI, CB, CJ, SA, SG, XX to their resolved classes.
fn resolve_lb1(cp: u32) -> Lb {
    let c = lb(cp);
    match c {
        Lb::AI => {
            // Resolve based on East_Asian_Width context.
            // Default (non-East-Asian context): treat as AL.
            // The test data uses "AI_EastAsian" annotation, meaning in the
            // default algorithm AI resolves to AL unless in East Asian context.
            // For conformance with the default test, resolve AI -> AL.
            Lb::AL
        }
        Lb::SG => Lb::AL,
        Lb::XX => Lb::AL,
        Lb::CJ => {
            // Default: treat as NS (strict mode). The test data uses NS.
            Lb::NS
        }
        Lb::SA => {
            // SA with General_Category Mn or Mc -> CM, otherwise -> AL.
            if is_gc_mn_or_mc(cp) {
                Lb::CM
            } else {
                Lb::AL
            }
        }
        _ => c,
    }
}

/// Check if a code point has General_Category Mn (Nonspacing_Mark) or
/// Mc (Spacing_Mark). Used for SA -> CM resolution in LB1.
fn is_gc_mn_or_mc(cp: u32) -> bool {
    // Use the Unicode General_Category. For now, check via the UnicodeData
    // ranges. We can use a simplified check based on known SA ranges.
    // SA characters are mainly Thai (0E00-0E7F), Lao (0E80-0EFF),
    // Myanmar (1000-109F), Khmer (1780-17FF), Tai Tham (1A20-1AAF),
    // and a few others. Within these, Mn/Mc marks are:
    // Thai: 0E31, 0E34-0E3A, 0E47-0E4E (Mn), 0E33 (Mc? actually Lo+Mn combo)
    // Lao: 0EB1, 0EB4-0EB9, 0EBB-0EBC, 0EC8-0ECD (Mn), 0EB3 (Mc)
    // Myanmar: 102B-102C (Mc), 102D-1030 (Mn), 1031 (Mc), 1032-1037 (Mn),
    //          1038 (Mc), 1039-103A (Mn), 103B-103C (Mc), 103D-103E (Mn),
    //          1056-1057 (Mc), 1058-1059 (Mn), 105E-1060 (Mn), 1062 (Mc),
    //          1067-1068 (Mc), 1071-1074 (Mn), 1082 (Mn), 1083-1084 (Mc),
    //          1085-1086 (Mn), 1087-108C (Mc), 108D (Mn), 108F (Mc),
    //          109A-109C (Mc), 109D (Mn), A9E5 (Mn)
    // Khmer: 17B4-17B5 (Mn), 17B6 (Mc), 17B7-17BD (Mn), 17BE-17C5 (Mc),
    //        17C6 (Mn), 17C7-17C8 (Mc), 17C9-17D3 (Mn), 17DD (Mn)
    // Tai Tham: 1A55 (Mc), 1A56 (Mn), 1A57 (Mc), 1A58-1A5E (Mn),
    //           1A62 (Mn), 1A65-1A6C (Mn), 1A6D-1A72 (Mc), 1A73-1A7C (Mn),
    //           1A7F (Mn)
    // We'll check the common SA ranges directly.
    matches!(cp,
        // Thai Mn
        0x0E31 | 0x0E34..=0x0E3A | 0x0E47..=0x0E4E |
        // Thai Mc
        0x0E33 |
        // Lao Mn
        0x0EB1 | 0x0EB4..=0x0EBC | 0x0EC8..=0x0ECD |
        // Lao Mc
        0x0EB3 |
        // Myanmar Mn
        0x102D..=0x1030 | 0x1032..=0x1037 | 0x1039..=0x103A |
        0x103D..=0x103E | 0x1058..=0x1059 | 0x105E..=0x1060 |
        0x1071..=0x1074 | 0x1082 | 0x1085..=0x1086 | 0x108D | 0x109D |
        0xA9E5 |
        // Myanmar Mc
        0x102B..=0x102C | 0x1031 | 0x1038 | 0x103B..=0x103C |
        0x1056..=0x1057 | 0x1062 | 0x1067..=0x1068 | 0x1083..=0x1084 |
        0x1087..=0x108C | 0x108F | 0x109A..=0x109C |
        // Khmer Mn
        0x17B4..=0x17B5 | 0x17B7..=0x17BD | 0x17C6 | 0x17C9..=0x17D3 |
        0x17DD |
        // Khmer Mc
        0x17B6 | 0x17BE..=0x17C5 | 0x17C7..=0x17C8 |
        // Tai Tham Mn
        0x1A56 | 0x1A58..=0x1A5E | 0x1A62 | 0x1A65..=0x1A6C |
        0x1A73..=0x1A7C | 0x1A7F |
        // Tai Tham Mc
        0x1A55 | 0x1A57 | 0x1A6D..=0x1A72
    )
}

/// Check if a code point is an "aksara base" for LB28a beyond AK/AS.
/// U+25CC (DOTTED CIRCLE) has Indic_Syllabic_Category=Dotted_Circle
/// and participates in aksara rules.
fn is_aksara_base(cp: u32) -> bool {
    cp == 0x25CC
}

/// Find the base character index looking back past CM/ZWJ.
fn find_base_index(classes: &[Lb], idx: usize) -> usize {
    let mut j = idx;
    loop {
        if classes[j] != Lb::CM && classes[j] != Lb::ZWJ {
            return j;
        }
        if j == 0 {
            return 0;
        }
        j -= 1;
    }
}

fn is_hard_break_or_space(c: Lb) -> bool {
    matches!(c, Lb::BK | Lb::CR | Lb::LF | Lb::NL | Lb::SP | Lb::ZW)
}

/// Check if the position i is effectively after OP SP* (for LB14).
fn is_after_op_sp(effective: &[Lb], classes: &[Lb], i: usize) -> bool {
    // Walk back from i through SP to find OP.
    let mut j = i;
    loop {
        if effective[j] == Lb::OP {
            return true;
        }
        if classes[j] != Lb::SP {
            return false;
        }
        if j == 0 {
            return false;
        }
        j -= 1;
    }
}

/// Check if position i is effectively after (CL|CP) SP* (for LB16).
fn is_after_cl_cp_sp(effective: &[Lb], classes: &[Lb], i: usize) -> bool {
    let mut j = i;
    loop {
        if effective[j] == Lb::CL || effective[j] == Lb::CP {
            return true;
        }
        if classes[j] != Lb::SP {
            return false;
        }
        if j == 0 {
            return false;
        }
        j -= 1;
    }
}

/// Check if position i is effectively after B2 SP* (for LB17).
fn is_after_b2_sp(effective: &[Lb], classes: &[Lb], i: usize) -> bool {
    let mut j = i;
    loop {
        if effective[j] == Lb::B2 {
            return true;
        }
        if classes[j] != Lb::SP {
            return false;
        }
        if j == 0 {
            return false;
        }
        j -= 1;
    }
}

/// Check if code point is a Unicode General_Category=Pi (Initial_Punctuation)
/// quotation mark. Used for QU sub-classification (QU_Pi).
fn is_qu_pi(cp: u32) -> bool {
    matches!(
        cp,
        0x00AB  // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
        | 0x2018 // LEFT SINGLE QUOTATION MARK
        | 0x201B // SINGLE HIGH-REVERSED-9 QUOTATION MARK
        | 0x201C // LEFT DOUBLE QUOTATION MARK
        | 0x201F // DOUBLE HIGH-REVERSED-9 QUOTATION MARK
        | 0x2039 // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
        | 0x2E02 // LEFT SUBSTITUTION BRACKET
        | 0x2E04 // LEFT DOTTED SUBSTITUTION BRACKET
        | 0x2E09 // LEFT TRANSPOSITION BRACKET
        | 0x2E0C // LEFT RAISED OMISSION BRACKET
        | 0x2E1C // LEFT LOW PARAPHRASE BRACKET
        | 0x2E20 // LEFT VERTICAL BAR WITH QUILL
    )
}

/// Check if code point is a Unicode General_Category=Pf (Final_Punctuation)
/// quotation mark. Used for QU sub-classification (QU_Pf).
fn is_qu_pf(cp: u32) -> bool {
    matches!(
        cp,
        0x00BB  // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
        | 0x2019 // RIGHT SINGLE QUOTATION MARK
        | 0x201D // RIGHT DOUBLE QUOTATION MARK
        | 0x203A // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
        | 0x2E03 // RIGHT SUBSTITUTION BRACKET
        | 0x2E05 // RIGHT DOTTED SUBSTITUTION BRACKET
        | 0x2E0A // RIGHT TRANSPOSITION BRACKET
        | 0x2E0D // RIGHT RAISED OMISSION BRACKET
        | 0x2E1D // RIGHT LOW PARAPHRASE BRACKET
        | 0x2E21 // RIGHT VERTICAL BAR WITH QUILL
    )
}

/// LB15a: Check if position i is effectively after QU_Pi SP* preceded by
/// appropriate context (sot or BK/CR/LF/NL/OP/QU/GL/SP/ZW).
fn is_after_qu_pi_sp(
    effective: &[Lb],
    classes: &[Lb],
    cps: &[u32],
    i: usize,
    _chars: &[(usize, char)],
) -> bool {
    // Walk back through SP to find QU_Pi.
    let mut j = i;
    loop {
        let ej = effective[j];
        if ej == Lb::QU {
            // When CM absorbed into QU, cps[j] is the CM code point.
            // Use find_base_index to get the actual QU character.
            let base_idx = find_base_index(&classes, j);
            if is_qu_pi(cps[base_idx]) {
                // Found QU_Pi. Check context before it.
                if base_idx == 0 {
                    return true; // sot
                }
                let prev = effective[base_idx - 1];
                return matches!(
                    prev,
                    Lb::BK
                        | Lb::CR
                        | Lb::LF
                        | Lb::NL
                        | Lb::OP
                        | Lb::QU
                        | Lb::GL
                        | Lb::SP
                        | Lb::ZW
                );
            }
            return false;
        }
        if classes[j] != Lb::SP {
            return false;
        }
        if j == 0 {
            return false;
        }
        j -= 1;
    }
}

/// LB15b: Check if the character after position qu_idx has the right context.
fn is_followed_by_lb15b_context(classes: &[Lb], qu_idx: usize, n: usize) -> bool {
    if qu_idx + 1 >= n {
        return true; // eot
    }
    let next = classes[qu_idx + 1];
    matches!(
        next,
        Lb::SP
            | Lb::GL
            | Lb::WJ
            | Lb::CL
            | Lb::QU
            | Lb::CP
            | Lb::EX
            | Lb::IS
            | Lb::SY
            | Lb::BK
            | Lb::CR
            | Lb::LF
            | Lb::NL
            | Lb::ZW
    )
}

/// Get effective class looking backwards past CM/ZWJ absorption.
fn effective_class_before(effective: &[Lb], classes: &[Lb], idx: usize) -> Lb {
    // Walk back to find the base character (skip CM/ZWJ that are absorbed).
    let mut j = idx;
    loop {
        if classes[j] != Lb::CM && classes[j] != Lb::ZWJ {
            return effective[j];
        }
        if j == 0 {
            return Lb::AL; // LB10 fallback
        }
        j -= 1;
    }
}

/// Find the index of the next non-CM/ZWJ character after position `start`.
/// Skips CMs/ZWJs that absorbed into the character at `start`.
/// Returns `n` if no non-CM/ZWJ is found.
fn find_next_non_cm(
    classes: &[Lb],
    _effective: &[Lb],
    start: usize,
    n: usize,
) -> usize {
    let mut k = start + 1;
    while k < n && (classes[k] == Lb::CM || classes[k] == Lb::ZWJ) {
        k += 1;
    }
    k
}

/// LB25: Complex numeric context rules.
/// Check if the break between i and i+1 should be prohibited due to numeric context.
fn is_lb25_no_break(
    effective: &[Lb],
    classes: &[Lb],
    _cps: &[u32],
    i: usize,
    n: usize,
) -> bool {
    let eb = effective[i];
    let ea = effective[i + 1];

    // Rule forms from UAX #14 LB25:
    // (PR|PO) x (OP|HY)? NU
    // -- if before is PR or PO and after starts a numeric expression
    if (eb == Lb::PR || eb == Lb::PO)
        && (ea == Lb::NU || ea == Lb::OP || ea == Lb::HY)
    {
        // Check: if ea is OP or HY, there must be NU following.
        if ea == Lb::NU {
            return true;
        }
        // ea is OP or HY: look ahead for NU
        if i + 2 < n && effective[i + 2] == Lb::NU {
            return true;
        }
        // OP SP* NU pattern
        if ea == Lb::OP {
            let mut k = i + 2;
            while k < n && classes[k] == Lb::SP {
                k += 1;
            }
            if k < n && effective[k] == Lb::NU {
                return true;
            }
        }
    }

    // NU (NU|SY|IS)* x (NU|SY|IS|CL|CP)
    if matches!(ea, Lb::NU | Lb::SY | Lb::IS | Lb::CL | Lb::CP) {
        // Walk back: check if there's a NU followed by (NU|SY|IS)* ending at i.
        if is_in_numeric_sequence(effective, i) {
            return true;
        }
    }

    // NU (NU|SY|IS)* (CL|CP)? x (PO|PR)
    if ea == Lb::PO || ea == Lb::PR {
        // Walk back: check for NU sequence optionally ending with CL|CP.
        let mut j = i;
        if effective[j] == Lb::CL || effective[j] == Lb::CP {
            if j == 0 {
                return false;
            }
            j -= 1;
        }
        if is_in_numeric_sequence(effective, j) {
            return true;
        }
    }

    // (OP|HY) x NU -- unconditional: don't break between HY/OP and NU.
    // UAX #14 LB25 sub-rules [25.13]: (OP|HY) x NU
    if (eb == Lb::HY || eb == Lb::OP) && ea == Lb::NU {
        return true;
    }

    // IS x NU -- unconditional: don't break between IS and NU.
    // UAX #14 LB25 sub-rule [25.14]: IS x NU
    if eb == Lb::IS && ea == Lb::NU {
        return true;
    }

    false
}

/// Check if position i is part of a NU (NU|SY|IS)* sequence.
fn is_in_numeric_sequence(effective: &[Lb], i: usize) -> bool {
    let mut j = i;
    loop {
        let c = effective[j];
        if c == Lb::NU {
            return true;
        }
        if c != Lb::SY && c != Lb::IS {
            return false;
        }
        if j == 0 {
            return false;
        }
        j -= 1;
    }
}

/// Check if a code point has General_Category=Cn (unassigned).
/// Used for LB30b second clause: [Extended_Pictographic && Cn] x EM.
/// Based on LineBreak.txt entries marked `# Cn` for Unicode 17.0,
/// intersected with Extended_Pictographic ranges.
fn is_cn(cp: u32) -> bool {
    matches!(cp,
        // Unassigned in Mahjong/Domino/Playing Cards block (1F000-1F0FF)
        0x1F02C..=0x1F02F |
        0x1F094..=0x1F09F |
        0x1F0AF..=0x1F0B0 |
        0x1F0C0 |
        0x1F0D0 |
        0x1F0F6..=0x1F0FF |
        // Unassigned in Enclosed Ideographic Supplement / Misc Symbols (1F100-1F2FF)
        0x1F1AE..=0x1F1E5 |
        0x1F203..=0x1F20F |
        0x1F23C..=0x1F23F |
        0x1F249..=0x1F24F |
        0x1F252..=0x1F25F |
        0x1F266..=0x1F2FF |
        // Unassigned in Transport & Map Symbols (1F680-1F6FF)
        0x1F6D9..=0x1F6DB |
        0x1F6ED..=0x1F6EF |
        0x1F6FD..=0x1F6FF |
        // Unassigned in Geometric Shapes Extended (1F780-1F7FF)
        0x1F7DA..=0x1F7DF |
        0x1F7EC..=0x1F7EF |
        0x1F7F1..=0x1F7FF |
        // Unassigned in Supplemental Arrows-C (1F800-1F8FF)
        0x1F80C..=0x1F80F |
        0x1F848..=0x1F84F |
        0x1F85A..=0x1F85F |
        0x1F888..=0x1F88F |
        0x1F8AE..=0x1F8FF |
        // Unassigned in Supplemental Symbols (1F900-1F9FF) -- none currently
        // Unassigned in Symbols and Pictographs Extended-A (1FA00-1FA6F)
        0x1FA58..=0x1FA5F |
        0x1FA6E..=0x1FA6F |
        // Unassigned in Symbols Extended-A (1FA70-1FAFF)
        0x1FA7D..=0x1FA7F |
        0x1FA8B..=0x1FA8D |
        0x1FAC7 |
        0x1FAC9..=0x1FACC |
        0x1FADD..=0x1FADE |
        0x1FAEB..=0x1FAEE |
        0x1FAF9..=0x1FAFF |
        // Large unassigned Extended_Pictographic block (1FC00-1FFFD)
        0x1FC00..=0x1FFFD
    )
}

/// Check if a code point is Extended_Pictographic.
fn is_extended_pictographic(cp: u32) -> bool {
    // Simplified check for common Extended_Pictographic ranges.
    // This covers the main emoji ranges that are assigned ID but have
    // the Extended_Pictographic property.
    matches!(cp,
        0x00A9 | 0x00AE |
        0x203C | 0x2049 |
        0x2122 | 0x2139 |
        0x2194..=0x2199 |
        0x21A9..=0x21AA |
        0x231A..=0x231B |
        0x2328 |
        0x23CF |
        0x23E9..=0x23F3 |
        0x23F8..=0x23FA |
        0x24C2 |
        0x25AA..=0x25AB |
        0x25B6 | 0x25C0 |
        0x25FB..=0x25FE |
        0x2600..=0x2604 |
        0x260E | 0x2611 | 0x2614..=0x2615 |
        0x2618 | 0x261D | 0x2620 |
        0x2622..=0x2623 | 0x2626 | 0x262A | 0x262E..=0x262F |
        0x2638..=0x263A | 0x2640 | 0x2642 |
        0x2648..=0x2653 |
        0x265F..=0x2660 | 0x2663 | 0x2665..=0x2666 | 0x2668 |
        0x267B | 0x267E..=0x267F |
        0x2692..=0x2697 | 0x2699 | 0x269B..=0x269C |
        0x26A0..=0x26A1 | 0x26A7 |
        0x26AA..=0x26AB |
        0x26B0..=0x26B1 |
        0x26BD..=0x26BE |
        0x26C4..=0x26C5 | 0x26C8 |
        0x26CE..=0x26CF | 0x26D1 | 0x26D3..=0x26D4 |
        0x26E9..=0x26EA |
        0x26F0..=0x26F5 | 0x26F7..=0x26FA | 0x26FD |
        0x2702 | 0x2705 | 0x2708..=0x270D | 0x270F |
        0x2712 | 0x2714 | 0x2716 | 0x271D | 0x2721 |
        0x2728 |
        0x2733..=0x2734 | 0x2744 | 0x2747 | 0x274C | 0x274E |
        0x2753..=0x2755 | 0x2757 |
        0x2763..=0x2764 |
        0x2795..=0x2797 | 0x27A1 | 0x27B0 | 0x27BF |
        0x2934..=0x2935 |
        0x2B05..=0x2B07 | 0x2B1B..=0x2B1C | 0x2B50 | 0x2B55 |
        0x3030 | 0x303D | 0x3297 | 0x3299 |
        0x1F000..=0x1F0FF |
        0x1F10D..=0x1F10F |
        0x1F12F |
        0x1F170..=0x1F171 | 0x1F17E..=0x1F17F |
        0x1F18E |
        0x1F191..=0x1F19A |
        0x1F1AD |
        0x1F1E6..=0x1F1FF |
        0x1F201..=0x1F202 | 0x1F21A | 0x1F22F |
        0x1F232..=0x1F23A | 0x1F250..=0x1F251 |
        0x1F300..=0x1F9FF |
        0x1FA00..=0x1FA6F |
        0x1FA70..=0x1FAFF |
        0x1FC00..=0x1FFFD
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let b = line_break_opportunities("");
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], BreakAction::Mandatory); // eot
    }

    #[test]
    fn test_simple_ascii() {
        // "Hello World" -- break after space
        let text = "Hello World";
        let b = line_break_opportunities(text);
        // Break should be allowed after the space (byte 5->6 boundary).
        // Space is at index 5. Break after space is at byte 6.
        assert_eq!(b[6], BreakAction::Allowed);
    }

    #[test]
    fn test_mandatory_break() {
        let text = "A\nB";
        let b = line_break_opportunities(text);
        // LF at index 1. Break after LF is mandatory at byte 2.
        assert_eq!(b[2], BreakAction::Mandatory);
    }

    #[test]
    fn test_crlf() {
        let text = "A\r\nB";
        let b = line_break_opportunities(text);
        // No break between CR and LF (byte 2).
        assert_eq!(b[2], BreakAction::Prohibited);
        // Mandatory break after LF (byte 3).
        assert_eq!(b[3], BreakAction::Mandatory);
    }
}
