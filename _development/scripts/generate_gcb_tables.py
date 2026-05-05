"""
Generate Rust Grapheme_Cluster_Break lookup tables from GraphemeBreakProperty.txt.

Reads:
  _development/data/ucd/GraphemeBreakProperty.txt

Writes:
  src/data/grapheme_break.rs

This replaces the hand-crafted GCB data with authoritative Unicode data,
fixing data gaps (especially SpacingMark for Indic scripts).

Preserves the existing is_extended_pictographic() function for GB11.

Run from repo root:
  python _development/scripts/generate_gcb_tables.py
"""

import os
import sys
from pathlib import Path
from collections import defaultdict

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
UCD_DIR = REPO_ROOT / "_development" / "data" / "ucd"
OUTPUT = REPO_ROOT / "src" / "data" / "grapheme_break.rs"


# GCB property values we care about (matching our Gcb enum)
GCB_VALUES = [
    "CR", "LF", "Control", "Extend", "ZWJ", "Regional_Indicator",
    "Prepend", "SpacingMark", "L", "V", "T", "LV", "LVT",
    "InCB_Consonant",  # mapped to IndicLetter
    "InCB_Linker",     # mapped to ConjunctLinker
    # NOTE: InCB_Extend is NOT included - it's a separate property that
    # should not override GCB values. InCB_Extend characters already have
    # their correct GCB value (usually Extend) from GraphemeBreakProperty.txt.
]

# Map file property names to our Rust enum variant names
NAME_MAP = {
    "CR": "Cr",
    "LF": "Lf",
    "Control": "Control",
    "Extend": "Extend",
    "ZWJ": "Zwj",
    "Regional_Indicator": "RegionalIndicator",
    "Prepend": "Prepend",
    "SpacingMark": "SpacingMark",
    "L": "L",
    "V": "V",
    "T": "T",
    "LV": "Lv",
    "LVT": "Lvt",
    "InCB_Consonant": "IndicLetter",
    "InCB_Linker": "ConjunctLinker",
}


def parse_grapheme_break_property(path):
    """Parse GraphemeBreakProperty.txt into ranges.

    Returns:
        dict: property_name -> list of (start_cp, end_cp) ranges
    """
    props = defaultdict(list)

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            # Strip inline comment
            if "#" in line:
                line = line[:line.index("#")].strip()

            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 2:
                continue

            cp_range = parts[0].strip()
            prop_name = parts[1].strip()

            if ".." in cp_range:
                start, end = cp_range.split("..")
                start_cp = int(start, 16)
                end_cp = int(end, 16)
            else:
                start_cp = int(cp_range, 16)
                end_cp = start_cp

            props[prop_name].append((start_cp, end_cp))

    return props


def merge_ranges(ranges):
    """Merge adjacent/overlapping ranges and sort."""
    if not ranges:
        return []
    ranges = sorted(ranges)
    merged = [ranges[0]]
    for start, end in ranges[1:]:
        if start <= merged[-1][1] + 1:
            merged[-1] = (merged[-1][0], max(merged[-1][1], end))
        else:
            merged.append((start, end))
    return merged


def generate_rust(props, extpict_ranges=None):
    """Generate Rust source file with GCB lookup tables."""

    lines = []
    lines.append("//! Auto-generated Grapheme_Cluster_Break property tables.")
    lines.append("//! Do not edit manually.")
    lines.append("//! Generated from GraphemeBreakProperty.txt (Unicode 17.0).")
    lines.append("//!")
    lines.append("//! Run: python _development/scripts/generate_gcb_tables.py")
    lines.append("")

    # --- Enum ---
    lines.append("/// Grapheme_Cluster_Break property values.")
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    lines.append("pub enum Gcb {")
    lines.append("    /// Other (default for unassigned code points)")
    lines.append("    Other,")
    lines.append("    /// CR (carriage return)")
    lines.append("    Cr,")
    lines.append("    /// LF (line feed)")
    lines.append("    Lf,")
    lines.append("    /// Control / Format")
    lines.append("    Control,")
    lines.append("    /// Extend (combining marks, modifiers)")
    lines.append("    Extend,")
    lines.append("    /// ZWJ (zero width joiner)")
    lines.append("    Zwj,")
    lines.append("    /// Regional_Indicator (flag sequences)")
    lines.append("    RegionalIndicator,")
    lines.append("    /// Prepend (certain Indic, Arabic formatting)")
    lines.append("    Prepend,")
    lines.append("    /// SpacingMark (spacing combining marks)")
    lines.append("    SpacingMark,")
    lines.append("    /// L (Hangul leading jamo)")
    lines.append("    L,")
    lines.append("    /// V (Hangul vowel jamo)")
    lines.append("    V,")
    lines.append("    /// T (Hangul trailing jamo)")
    lines.append("    T,")
    lines.append("    /// LV (Hangul LV syllable)")
    lines.append("    Lv,")
    lines.append("    /// LVT (Hangul LVT syllable)")
    lines.append("    Lvt,")
    lines.append("    /// Extended_Pictographic (emoji base for GB11)")
    lines.append("    ExtendedPictographic,")
    lines.append("    /// Indic consonant letter (for conjunct cluster rules)")
    lines.append("    IndicLetter,")
    lines.append("    /// Conjunct linker (Indic virama / invisible stacker)")
    lines.append("    ConjunctLinker,")
    lines.append("}")
    lines.append("")

    # --- Range table ---
    # Build a per-code-point map first, then compress to ranges.
    # This ensures InCB_Consonant/InCB_Linker override any conflicting GCB values.
    #
    # Step 1: Populate from GCB properties (from GraphemeBreakProperty.txt)
    # Step 2: Override with InCB_Consonant (-> IndicLetter) and InCB_Linker (-> ConjunctLinker)
    # Step 3: Compress back to ranges

    # Determine which property names are InCB overrides
    incb_overrides = {"InCB_Consonant", "InCB_Linker"}

    # First pass: collect all GCB ranges (non-InCB)
    cp_map = {}  # code_point -> rust_name
    for prop_name, ranges in props.items():
        rust_name = NAME_MAP.get(prop_name)
        if rust_name is None:
            continue
        if prop_name in incb_overrides:
            continue  # Skip InCB overrides for now
        for start, end in ranges:
            for cp in range(start, end + 1):
                cp_map[cp] = rust_name

    # Second pass: apply InCB overrides (these take precedence)
    for prop_name in incb_overrides:
        if prop_name not in props:
            continue
        rust_name = NAME_MAP.get(prop_name)
        if rust_name is None:
            continue
        for start, end in props[prop_name]:
            for cp in range(start, end + 1):
                cp_map[cp] = rust_name

    # Compress back to ranges
    all_ranges = []
    if cp_map:
        sorted_cps = sorted(cp_map.items())
        run_start = sorted_cps[0][0]
        run_end = sorted_cps[0][0]
        run_val = sorted_cps[0][1]
        for cp, val in sorted_cps[1:]:
            if val == run_val and cp == run_end + 1:
                run_end = cp
            else:
                all_ranges.append((run_start, run_end, run_val))
                run_start = cp
                run_end = cp
                run_val = val
        all_ranges.append((run_start, run_end, run_val))

    lines.append(f"/// GCB range table: (start_cp, end_cp, Gcb). {len(all_ranges)} entries.")
    lines.append(f"/// Sorted by start_cp for binary search.")
    lines.append(f"const GCB_RANGES: &[(u32, u32, Gcb)] = &[")
    for start, end, variant in all_ranges:
        lines.append(f"    (0x{start:04X}, 0x{end:04X}, Gcb::{variant}),")
    lines.append("];")
    lines.append("")

    # --- Lookup function ---
    lines.append("/// Look up Grapheme_Cluster_Break property for a character.")
    lines.append("#[must_use]")
    lines.append("pub fn gcb(ch: char) -> Gcb {")
    lines.append("    let cp = ch as u32;")
    lines.append("    // Check Extended_Pictographic first (not in GCB_RANGES)")
    lines.append("    if is_extended_pictographic(cp) {")
    lines.append("        return Gcb::ExtendedPictographic;")
    lines.append("    }")
    lines.append("    // Binary search on range table")
    lines.append("    match GCB_RANGES.binary_search_by(|&(start, end, _)| {")
    lines.append("        if cp < start {")
    lines.append("            std::cmp::Ordering::Greater")
    lines.append("        } else if cp > end {")
    lines.append("            std::cmp::Ordering::Less")
    lines.append("        } else {")
    lines.append("            std::cmp::Ordering::Equal")
    lines.append("        }")
    lines.append("    }) {")
    lines.append("        Ok(i) => GCB_RANGES[i].2,")
    lines.append("        Err(_) => Gcb::Other,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    # --- Extended Pictographic (from emoji-data.txt) ---
    lines.append("/// Extended_Pictographic (UTS #51) for GB11: emoji that can follow ZWJ.")
    lines.append("///")
    lines.append("/// Source: Unicode emoji-data.txt Extended_Pictographic property.")
    lines.append("#[must_use]")
    lines.append("pub fn is_extended_pictographic(u: u32) -> bool {")

    if extpict_ranges:
        # Generate from authoritative emoji-data.txt
        lines.append(f"    // {len(extpict_ranges)} ranges from emoji-data.txt (Unicode 17.0)")
        lines.append("    matches!(u,")
        for i, (start, end) in enumerate(extpict_ranges):
            prefix = "        " if i == 0 else "        | "
            if start == end:
                lines.append(f"{prefix}0x{start:04X}")
            else:
                lines.append(f"{prefix}0x{start:04X}..=0x{end:04X}")
        lines.append("    )")
    else:
        # Fallback: very broad approximate ranges (legacy)
        lines.append("    // Approximate ranges (authoritative data not available)")
        lines.append("    (0x1F000..=0x1FFFD).contains(&u)")

    lines.append("}")
    lines.append("")

    return "\n".join(lines)


def parse_incb_properties(path):
    """Parse DerivedCoreProperties.txt for InCB (Indic_Conjunct_Break) entries.

    Returns:
        dict: 'InCB_Consonant' -> list of ranges, 'InCB_Linker' -> list of ranges
    """
    props = defaultdict(list)

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            # Strip inline comment
            if "#" in line:
                line = line[:line.index("#")].strip()

            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 3:
                continue

            cp_range = parts[0].strip()
            prop_name = parts[1].strip()
            prop_value = parts[2].strip()

            if prop_name != "InCB":
                continue

            if ".." in cp_range:
                start, end = cp_range.split("..")
                start_cp = int(start, 16)
                end_cp = int(end, 16)
            else:
                start_cp = int(cp_range, 16)
                end_cp = start_cp

            key = f"InCB_{prop_value}"
            props[key].append((start_cp, end_cp))

    return props


def parse_extended_pictographic(path):
    """Parse emoji-data.txt for Extended_Pictographic ranges.

    Returns:
        list of (start_cp, end_cp) ranges
    """
    ranges = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            # Strip inline comment
            if "#" in line:
                line = line[:line.index("#")].strip()

            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 2:
                continue

            prop = parts[1].strip()
            if prop != "Extended_Pictographic":
                continue

            cp_range = parts[0].strip()
            if ".." in cp_range:
                start, end = cp_range.split("..")
                ranges.append((int(start, 16), int(end, 16)))
            else:
                cp = int(cp_range, 16)
                ranges.append((cp, cp))

    return merge_ranges(ranges)


def main():
    gbp_path = UCD_DIR / "GraphemeBreakProperty.txt"
    dcp_path = UCD_DIR / "DerivedCoreProperties.txt"
    emoji_path = UCD_DIR / "emoji-data.txt"

    if not gbp_path.exists():
        print(f"ERROR: {gbp_path} not found.")
        sys.exit(1)

    print("Parsing GraphemeBreakProperty.txt ...")
    props = parse_grapheme_break_property(gbp_path)
    for name in sorted(props.keys()):
        total_cps = sum(e - s + 1 for s, e in props[name])
        print(f"  {name}: {len(props[name])} ranges, {total_cps} code points")

    # Parse InCB from DerivedCoreProperties.txt if available
    if dcp_path.exists():
        print("Parsing DerivedCoreProperties.txt for InCB ...")
        incb_props = parse_incb_properties(dcp_path)
        for name in sorted(incb_props.keys()):
            total_cps = sum(e - s + 1 for s, e in incb_props[name])
            print(f"  {name}: {len(incb_props[name])} ranges, {total_cps} code points")
            props[name] = incb_props[name]
    else:
        print("WARNING: DerivedCoreProperties.txt not found; InCB data not included.")

    # Parse Extended_Pictographic from emoji-data.txt
    extpict_ranges = []
    if emoji_path.exists():
        print("Parsing emoji-data.txt for Extended_Pictographic ...")
        extpict_ranges = parse_extended_pictographic(emoji_path)
        total_cps = sum(e - s + 1 for s, e in extpict_ranges)
        print(f"  Extended_Pictographic: {len(extpict_ranges)} ranges, {total_cps} code points")
    else:
        print("WARNING: emoji-data.txt not found; using approximate ExtPict ranges.")

    print("Generating Rust source ...")
    rust_src = generate_rust(props, extpict_ranges)

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(rust_src)
    print(f"  Written to {OUTPUT}")

    size_kb = os.path.getsize(OUTPUT) / 1024
    print(f"  File size: {size_kb:.1f} KB")
    print("Done.")


if __name__ == "__main__":
    main()
