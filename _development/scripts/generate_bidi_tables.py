"""
Generate Rust bidi lookup tables from UnicodeData.txt and BidiBrackets.txt.

Produces src/data/bidi_class.rs with:
- Bidi_Class enum
- bidi_class() lookup function using range tables
- Bracket pair tables and lookup functions for BD16

Usage:
    python _development/scripts/generate_bidi_tables.py
"""

import os
import re
from collections import defaultdict


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
UCD_DIR = os.path.join(SCRIPT_DIR, "..", "data", "ucd")
OUT_PATH = os.path.join(SCRIPT_DIR, "..", "..", "src", "data", "bidi_class.rs")

# The 23 Bidi_Class values
BIDI_CLASSES = [
    "L", "R", "AL",
    "EN", "ES", "ET", "AN", "CS",
    "NSM", "BN",
    "B", "S", "WS", "ON",
    "LRE", "RLE", "LRO", "RLO", "PDF",
    "LRI", "RLI", "FSI", "PDI",
]


def parse_unicode_data(path):
    """Parse UnicodeData.txt and return dict: bidi_class -> list of code points."""
    class_to_cps = defaultdict(list)
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            fields = line.split(";")
            cp = int(fields[0], 16)
            name = fields[1]
            bidi_class = fields[4]
            class_to_cps[bidi_class].append(cp)
    return class_to_cps


def compress_to_ranges(cps):
    """Convert sorted list of code points to (start, end) ranges."""
    if not cps:
        return []
    cps = sorted(set(cps))
    ranges = []
    start = cps[0]
    end = cps[0]
    for cp in cps[1:]:
        if cp == end + 1:
            end = cp
        else:
            ranges.append((start, end))
            start = cp
            end = cp
    ranges.append((start, end))
    return ranges


def parse_bidi_brackets(path):
    """Parse BidiBrackets.txt. Returns list of (cp, paired_cp, type) where type is 'o' or 'c'."""
    pairs = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            # Remove inline comment
            if "#" in line:
                line = line[:line.index("#")].strip()
            fields = [f.strip() for f in line.split(";")]
            cp = int(fields[0], 16)
            paired = int(fields[1], 16)
            bpt = fields[2]  # 'o' or 'c'
            pairs.append((cp, paired, bpt))
    return pairs


def generate_rust(class_to_cps, bracket_pairs):
    """Generate the Rust source file."""
    lines = []
    lines.append("//! Auto-generated Bidi_Class and bracket pair tables.")
    lines.append("//! Do not edit manually.")
    lines.append("//! Generated from UnicodeData.txt and BidiBrackets.txt (Unicode 17.0).")
    lines.append("")
    lines.append("/// Bidi_Class property values (UAX #9).")
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    lines.append("#[allow(non_camel_case_types)]")
    lines.append("pub enum BidiClass {")
    for bc in BIDI_CLASSES:
        lines.append(f"    {bc},")
    lines.append("}")
    lines.append("")

    # Build range table for each class.  L is the default and has the most
    # code points, so we store everything *except* L in the table, and
    # return L as default.
    #
    # Actually for a clean approach: store all non-L classes as range tables.
    # We'll build a single sorted array of (start, end, class).

    all_ranges = []
    for bc in BIDI_CLASSES:
        if bc == "L":
            continue  # default
        cps = class_to_cps.get(bc, [])
        ranges = compress_to_ranges(cps)
        for (s, e) in ranges:
            all_ranges.append((s, e, bc))

    # Sort by start
    all_ranges.sort(key=lambda x: x[0])

    lines.append(f"/// Range table: (start, end, BidiClass). {len(all_ranges)} entries.")
    lines.append("/// Characters not in this table default to BidiClass::L.")
    lines.append("const BIDI_CLASS_RANGES: &[(u32, u32, BidiClass)] = &[")
    for (s, e, bc) in all_ranges:
        if s == e:
            lines.append(f"    (0x{s:04X}, 0x{e:04X}, BidiClass::{bc}),")
        else:
            lines.append(f"    (0x{s:04X}, 0x{e:04X}, BidiClass::{bc}),")
    lines.append("];")
    lines.append("")

    # Lookup function with binary search
    lines.append("/// Return the Bidi_Class for a code point.")
    lines.append("pub fn bidi_class(cp: u32) -> BidiClass {")
    lines.append("    // Binary search on BIDI_CLASS_RANGES")
    lines.append("    let mut lo: usize = 0;")
    lines.append("    let mut hi: usize = BIDI_CLASS_RANGES.len();")
    lines.append("    while lo < hi {")
    lines.append("        let mid = lo + (hi - lo) / 2;")
    lines.append("        let (start, end, _) = BIDI_CLASS_RANGES[mid];")
    lines.append("        if cp < start {")
    lines.append("            hi = mid;")
    lines.append("        } else if cp > end {")
    lines.append("            lo = mid + 1;")
    lines.append("        } else {")
    lines.append("            return BIDI_CLASS_RANGES[mid].2;")
    lines.append("        }")
    lines.append("    }")
    lines.append("    BidiClass::L")
    lines.append("}")
    lines.append("")

    # Bracket pairs
    open_brackets = [(cp, paired) for (cp, paired, bpt) in bracket_pairs if bpt == "o"]
    # Sort by code point for binary search
    open_brackets.sort(key=lambda x: x[0])

    lines.append(f"/// Opening bracket -> closing bracket mapping ({len(open_brackets)} pairs).")
    lines.append("const BRACKET_PAIRS: &[(u32, u32)] = &[")
    for (cp, paired) in open_brackets:
        lines.append(f"    (0x{cp:04X}, 0x{paired:04X}),")
    lines.append("];")
    lines.append("")

    # Build a set of closing brackets for quick lookup
    close_set = sorted(set(cp for (cp, _, bpt) in bracket_pairs if bpt == "c"))

    lines.append("/// Return the matching closing bracket for an opening bracket, if any.")
    lines.append("pub fn bracket_pair(cp: u32) -> Option<u32> {")
    lines.append("    let mut lo: usize = 0;")
    lines.append("    let mut hi: usize = BRACKET_PAIRS.len();")
    lines.append("    while lo < hi {")
    lines.append("        let mid = lo + (hi - lo) / 2;")
    lines.append("        let (open, _) = BRACKET_PAIRS[mid];")
    lines.append("        if cp < open {")
    lines.append("            hi = mid;")
    lines.append("        } else if cp > open {")
    lines.append("            lo = mid + 1;")
    lines.append("        } else {")
    lines.append("            return Some(BRACKET_PAIRS[mid].1);")
    lines.append("        }")
    lines.append("    }")
    lines.append("    None")
    lines.append("}")
    lines.append("")

    # Closing bracket check
    lines.append("/// Closing bracket range table for quick check.")
    close_ranges = compress_to_ranges(close_set)
    lines.append(f"const CLOSING_BRACKETS: &[(u32, u32)] = &[")
    for (s, e) in close_ranges:
        lines.append(f"    (0x{s:04X}, 0x{e:04X}),")
    lines.append("];")
    lines.append("")

    lines.append("/// Check if a code point is a closing bracket.")
    lines.append("pub fn is_closing_bracket(cp: u32) -> bool {")
    lines.append("    let mut lo: usize = 0;")
    lines.append("    let mut hi: usize = CLOSING_BRACKETS.len();")
    lines.append("    while lo < hi {")
    lines.append("        let mid = lo + (hi - lo) / 2;")
    lines.append("        let (start, end) = CLOSING_BRACKETS[mid];")
    lines.append("        if cp < start {")
    lines.append("            hi = mid;")
    lines.append("        } else if cp > end {")
    lines.append("            lo = mid + 1;")
    lines.append("        } else {")
    lines.append("            return true;")
    lines.append("        }")
    lines.append("    }")
    lines.append("    false")
    lines.append("}")
    lines.append("")

    # For N0: given a closing bracket, find its opening bracket
    close_to_open = [(cp, paired) for (cp, paired, bpt) in bracket_pairs if bpt == "c"]
    close_to_open.sort(key=lambda x: x[0])
    lines.append(f"/// Closing bracket -> opening bracket mapping ({len(close_to_open)} pairs).")
    lines.append("const CLOSE_TO_OPEN: &[(u32, u32)] = &[")
    for (cp, paired) in close_to_open:
        lines.append(f"    (0x{cp:04X}, 0x{paired:04X}),")
    lines.append("];")
    lines.append("")
    lines.append("/// Return the matching opening bracket for a closing bracket, if any.")
    lines.append("pub fn opening_bracket_for(cp: u32) -> Option<u32> {")
    lines.append("    let mut lo: usize = 0;")
    lines.append("    let mut hi: usize = CLOSE_TO_OPEN.len();")
    lines.append("    while lo < hi {")
    lines.append("        let mid = lo + (hi - lo) / 2;")
    lines.append("        let (close, _) = CLOSE_TO_OPEN[mid];")
    lines.append("        if cp < close {")
    lines.append("            hi = mid;")
    lines.append("        } else if cp > close {")
    lines.append("            lo = mid + 1;")
    lines.append("        } else {")
    lines.append("            return Some(CLOSE_TO_OPEN[mid].1);")
    lines.append("        }")
    lines.append("    }")
    lines.append("    None")
    lines.append("}")
    lines.append("")

    return "\n".join(lines) + "\n"


def main():
    udata_path = os.path.join(UCD_DIR, "UnicodeData.txt")
    brackets_path = os.path.join(UCD_DIR, "BidiBrackets.txt")

    print(f"Parsing {udata_path} ...")
    class_to_cps = parse_unicode_data(udata_path)
    total = sum(len(v) for v in class_to_cps.values())
    print(f"  {total} code points across {len(class_to_cps)} bidi classes")

    print(f"Parsing {brackets_path} ...")
    bracket_pairs = parse_bidi_brackets(brackets_path)
    print(f"  {len(bracket_pairs)} bracket entries")

    rust_src = generate_rust(class_to_cps, bracket_pairs)

    os.makedirs(os.path.dirname(OUT_PATH), exist_ok=True)
    with open(OUT_PATH, "w", encoding="utf-8", newline="\n") as f:
        f.write(rust_src)
    print(f"Wrote {OUT_PATH}")
    print(f"  Range table: {sum(1 for _ in rust_src.split(chr(10)) if 'BidiClass::' in _ and '(' in _)} entries")


if __name__ == "__main__":
    main()
