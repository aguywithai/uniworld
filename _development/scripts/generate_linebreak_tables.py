"""
Generate Rust Line_Break property lookup tables from LineBreak.txt and EastAsianWidth.txt.

Produces src/data/line_break.rs with:
- LineBreak enum (all LB classes)
- lb() lookup function using range tables
- East_Asian_Width helpers for AI/CJ/XX resolution

Usage:
    python _development/scripts/generate_linebreak_tables.py
"""

import os
from collections import defaultdict

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
UCD_DIR = os.path.join(SCRIPT_DIR, "..", "data", "ucd")
OUT_PATH = os.path.join(SCRIPT_DIR, "..", "..", "src", "data", "line_break.rs")

# All Line_Break classes from UAX #14
LB_CLASSES = [
    "BK", "CM", "CR", "GL", "LF", "NL", "SP", "WJ", "ZW", "ZWJ",
    "AI", "AK", "AL", "AP", "AS", "B2", "BA", "BB", "CB", "CJ",
    "CL", "CP", "EB", "EM", "EX", "H2", "H3", "HH", "HL", "HY",
    "ID", "IN", "IS", "JL", "JT", "JV", "NS", "NU", "OP", "PO",
    "PR", "QU", "RI", "SA", "SG", "SY", "VF", "VI", "XX",
]


def parse_linebreak(path):
    """Parse LineBreak.txt -> dict of class -> list of (start, end) ranges."""
    class_ranges = defaultdict(list)
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "#" in line:
                line = line[:line.index("#")].strip()
            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 2:
                continue
            cp_range = parts[0]
            lb_class = parts[1]
            if ".." in cp_range:
                start, end = cp_range.split("..")
                start = int(start, 16)
                end = int(end, 16)
            else:
                start = int(cp_range, 16)
                end = start
            class_ranges[lb_class].append((start, end))
    return class_ranges


def parse_east_asian_width(path):
    """Parse EastAsianWidth.txt -> set of code points with width W or F."""
    wide_cps = set()
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "#" in line:
                line = line[:line.index("#")].strip()
            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 2:
                continue
            eaw = parts[1]
            if eaw in ("W", "F"):
                cp_range = parts[0]
                if ".." in cp_range:
                    start, end = cp_range.split("..")
                    for cp in range(int(start, 16), int(end, 16) + 1):
                        wide_cps.add(cp)
                else:
                    wide_cps.add(int(cp_range, 16))
    return wide_cps


def compress_ranges(ranges):
    """Merge and sort ranges."""
    if not ranges:
        return []
    merged = []
    ranges = sorted(ranges)
    start, end = ranges[0]
    for s, e in ranges[1:]:
        if s <= end + 1:
            end = max(end, e)
        else:
            merged.append((start, end))
            start, end = s, e
    merged.append((start, end))
    return merged


def generate_rust(class_ranges, wide_cps):
    """Generate Rust source."""
    lines = []
    lines.append("//! Auto-generated Line_Break property tables.")
    lines.append("//! Do not edit manually.")
    lines.append("//! Generated from LineBreak.txt and EastAsianWidth.txt (Unicode 17.0).")
    lines.append("")
    lines.append("/// Line_Break property values (UAX #14).")
    lines.append("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
    lines.append("#[allow(non_camel_case_types)]")
    lines.append("pub enum Lb {")
    for cls in LB_CLASSES:
        lines.append(f"    {cls},")
    lines.append("}")
    lines.append("")

    # Build a single sorted range table of (start, end, class).
    # XX is the default for anything not listed.
    all_ranges = []
    for cls in LB_CLASSES:
        if cls == "XX":
            continue  # default
        ranges = class_ranges.get(cls, [])
        ranges = compress_ranges(ranges)
        for (s, e) in ranges:
            all_ranges.append((s, e, cls))

    all_ranges.sort(key=lambda x: x[0])

    lines.append(f"/// Range table: {len(all_ranges)} entries. Default is Lb::XX.")
    lines.append("const LB_RANGES: &[(u32, u32, Lb)] = &[")
    for (s, e, cls) in all_ranges:
        lines.append(f"    (0x{s:04X}, 0x{e:04X}, Lb::{cls}),")
    lines.append("];")
    lines.append("")

    # Lookup function
    lines.append("/// Return the Line_Break class for a code point.")
    lines.append("/// Handles default ranges for CJK (ID), currency (PR), etc.")
    lines.append("pub fn lb(cp: u32) -> Lb {")
    lines.append("    // Binary search")
    lines.append("    let mut lo: usize = 0;")
    lines.append("    let mut hi: usize = LB_RANGES.len();")
    lines.append("    while lo < hi {")
    lines.append("        let mid = lo + (hi - lo) / 2;")
    lines.append("        let (start, end, _) = LB_RANGES[mid];")
    lines.append("        if cp < start {")
    lines.append("            hi = mid;")
    lines.append("        } else if cp > end {")
    lines.append("            lo = mid + 1;")
    lines.append("        } else {")
    lines.append("            return LB_RANGES[mid].2;")
    lines.append("        }")
    lines.append("    }")
    lines.append("    // Default ranges from LineBreak.txt header:")
    lines.append("    // CJK blocks default to ID")
    lines.append("    if (0x3400..=0x4DBF).contains(&cp)")
    lines.append("        || (0x4E00..=0x9FFF).contains(&cp)")
    lines.append("        || (0xF900..=0xFAFF).contains(&cp)")
    lines.append("        || (0x20000..=0x2FFFD).contains(&cp)")
    lines.append("        || (0x30000..=0x3FFFD).contains(&cp)")
    lines.append("        || (0x1F000..=0x1F7FF).contains(&cp)")
    lines.append("        || (0x1F900..=0x1FAFF).contains(&cp)")
    lines.append("        || (0x1FC00..=0x1FFFD).contains(&cp)")
    lines.append("    {")
    lines.append("        return Lb::ID;")
    lines.append("    }")
    lines.append("    // Currency Symbols default to PR")
    lines.append("    if (0x20A0..=0x20CF).contains(&cp) {")
    lines.append("        return Lb::PR;")
    lines.append("    }")
    lines.append("    Lb::XX")
    lines.append("}")
    lines.append("")

    # East Asian Width helper: is this code point wide (W or F)?
    wide_ranges = compress_ranges([(cp, cp) for cp in wide_cps])
    lines.append(f"/// East_Asian_Width W or F range table ({len(wide_ranges)} entries).")
    lines.append("const EAW_WIDE_RANGES: &[(u32, u32)] = &[")
    for (s, e) in wide_ranges:
        lines.append(f"    (0x{s:04X}, 0x{e:04X}),")
    lines.append("];")
    lines.append("")

    lines.append("/// Return true if the code point has East_Asian_Width W or F.")
    lines.append("pub fn is_east_asian_wide(cp: u32) -> bool {")
    lines.append("    let mut lo: usize = 0;")
    lines.append("    let mut hi: usize = EAW_WIDE_RANGES.len();")
    lines.append("    while lo < hi {")
    lines.append("        let mid = lo + (hi - lo) / 2;")
    lines.append("        let (start, end) = EAW_WIDE_RANGES[mid];")
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

    return "\n".join(lines) + "\n"


def main():
    lb_path = os.path.join(UCD_DIR, "LineBreak.txt")
    eaw_path = os.path.join(UCD_DIR, "EastAsianWidth.txt")

    print(f"Parsing {lb_path} ...")
    class_ranges = parse_linebreak(lb_path)
    total_ranges = sum(len(v) for v in class_ranges.values())
    print(f"  {total_ranges} ranges across {len(class_ranges)} classes")

    print(f"Parsing {eaw_path} ...")
    wide_cps = parse_east_asian_width(eaw_path)
    print(f"  {len(wide_cps)} wide (W/F) code points")

    rust_src = generate_rust(class_ranges, wide_cps)

    os.makedirs(os.path.dirname(OUT_PATH), exist_ok=True)
    with open(OUT_PATH, "w", encoding="utf-8", newline="\n") as f:
        f.write(rust_src)
    print(f"Wrote {OUT_PATH}")


if __name__ == "__main__":
    main()
