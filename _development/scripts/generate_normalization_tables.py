"""
Generate Rust normalization lookup tables from UnicodeData.txt.

Reads:
  _development/data/ucd/UnicodeData.txt
  _development/data/ucd/CompositionExclusions.txt

Writes:
  src/data/normalization.rs

Run from repo root:
  python _development/scripts/generate_normalization_tables.py
"""

import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
UCD_DIR = REPO_ROOT / "_development" / "data" / "ucd"
OUTPUT = REPO_ROOT / "src" / "data" / "normalization.rs"


def parse_unicode_data(path):
    """Parse UnicodeData.txt for CCC and decomposition mappings."""
    ccc_map = {}  # code_point -> ccc (non-zero only)
    canonical_decomp = {}  # code_point -> [code_points]
    compat_decomp = {}  # code_point -> [code_points] (includes canonical)

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            fields = line.split(";")
            if len(fields) < 6:
                continue

            cp = int(fields[0], 16)
            ccc = int(fields[3])
            decomp_field = fields[5].strip()

            if ccc != 0:
                ccc_map[cp] = ccc

            if decomp_field:
                # Check for compatibility tag
                if decomp_field.startswith("<"):
                    # Compatibility decomposition (has tag like <compat>, <font>, etc.)
                    tag_end = decomp_field.index(">")
                    decomp_str = decomp_field[tag_end + 1:].strip()
                    decomp_cps = [int(x, 16) for x in decomp_str.split()]
                    compat_decomp[cp] = decomp_cps
                else:
                    # Canonical decomposition (no tag)
                    decomp_cps = [int(x, 16) for x in decomp_field.split()]
                    canonical_decomp[cp] = decomp_cps
                    compat_decomp[cp] = decomp_cps

    return ccc_map, canonical_decomp, compat_decomp


def parse_composition_exclusions(path):
    """Parse CompositionExclusions.txt for explicit exclusion list."""
    exclusions = set()
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            # Lines like: "0958    #  DEVANAGARI LETTER QA"
            cp_str = line.split("#")[0].strip()
            if cp_str:
                exclusions.add(int(cp_str, 16))
    return exclusions


def build_composition_table(canonical_decomp, ccc_map, exclusions):
    """
    Build canonical composition table from decompositions.

    A canonical decomposition X -> A B creates composition entry (A, B) -> X
    UNLESS X is in the Full_Composition_Exclusion set:
    - Explicitly listed in CompositionExclusions.txt
    - Singleton decomposition (X -> A, length 1)
    - Non-starter decomposition (X has non-zero CCC, or A has non-zero CCC)
    """
    compositions = {}  # (first, second) -> composed

    for cp, decomp in canonical_decomp.items():
        # Only 2-character decompositions
        if len(decomp) != 2:
            continue

        # Check exclusions
        if cp in exclusions:
            continue

        # Non-starter exclusion: if the character itself has non-zero CCC
        if ccc_map.get(cp, 0) != 0:
            continue

        # Non-starter exclusion: if first character of decomposition has non-zero CCC
        if ccc_map.get(decomp[0], 0) != 0:
            continue

        first, second = decomp[0], decomp[1]
        compositions[(first, second)] = cp

    return compositions


def generate_rust(ccc_map, canonical_decomp, compat_decomp, compositions):
    """Generate Rust source file with normalization tables."""

    lines = []
    lines.append("//! Auto-generated Unicode normalization tables.")
    lines.append("//! Do not edit manually.")
    lines.append("//! Generated from UnicodeData.txt and CompositionExclusions.txt (Unicode 17.0).")
    lines.append("//!")
    lines.append("//! Run: python _development/scripts/generate_normalization_tables.py")
    lines.append("")

    # --- CCC table ---
    ccc_sorted = sorted(ccc_map.items())
    lines.append("/// Canonical Combining Class entries: (code_point, ccc).")
    lines.append("/// Sorted by code_point for binary search.")
    lines.append(f"pub const CCC_TABLE: &[(u32, u8)] = &[")
    for cp, ccc in ccc_sorted:
        lines.append(f"    (0x{cp:04X}, {ccc}),")
    lines.append("];")
    lines.append("")

    # --- Canonical decomposition ---
    # Flatten decomposition data into a single array, with index/length pairs
    canon_sorted = sorted(canonical_decomp.items())
    canon_data = []
    canon_index = []  # (code_point, offset, length)
    offset = 0
    for cp, decomp in canon_sorted:
        canon_index.append((cp, offset, len(decomp)))
        canon_data.extend(decomp)
        offset += len(decomp)

    lines.append("/// Canonical decomposition index: (code_point, data_offset, length).")
    lines.append("/// Sorted by code_point for binary search.")
    lines.append(f"pub const CANONICAL_DECOMP_INDEX: &[(u32, u16, u8)] = &[")
    for cp, off, ln in canon_index:
        lines.append(f"    (0x{cp:04X}, {off}, {ln}),")
    lines.append("];")
    lines.append("")

    lines.append("/// Canonical decomposition data (flattened code points).")
    lines.append(f"pub const CANONICAL_DECOMP_DATA: &[u32] = &[")
    chunk_size = 12
    for i in range(0, len(canon_data), chunk_size):
        chunk = canon_data[i:i+chunk_size]
        vals = ", ".join(f"0x{v:04X}" for v in chunk)
        lines.append(f"    {vals},")
    lines.append("];")
    lines.append("")

    # --- Compatibility decomposition (only entries NOT in canonical) ---
    compat_only = {cp: decomp for cp, decomp in compat_decomp.items()
                   if cp not in canonical_decomp}
    compat_sorted = sorted(compat_only.items())
    compat_data = []
    compat_index = []
    offset = 0
    for cp, decomp in compat_sorted:
        compat_index.append((cp, offset, len(decomp)))
        compat_data.extend(decomp)
        offset += len(decomp)

    lines.append("/// Compatibility-only decomposition index (entries not in canonical).")
    lines.append("/// Sorted by code_point for binary search.")
    lines.append(f"pub const COMPAT_DECOMP_INDEX: &[(u32, u16, u8)] = &[")
    for cp, off, ln in compat_index:
        lines.append(f"    (0x{cp:04X}, {off}, {ln}),")
    lines.append("];")
    lines.append("")

    lines.append("/// Compatibility-only decomposition data (flattened code points).")
    lines.append(f"pub const COMPAT_DECOMP_DATA: &[u32] = &[")
    for i in range(0, len(compat_data), chunk_size):
        chunk = compat_data[i:i+chunk_size]
        vals = ", ".join(f"0x{v:04X}" for v in chunk)
        lines.append(f"    {vals},")
    lines.append("];")
    lines.append("")

    # --- Composition table ---
    comp_sorted = sorted(compositions.items())
    lines.append("/// Canonical composition table: (first, second, composed).")
    lines.append("/// Sorted by (first, second) for binary search.")
    lines.append(f"pub const COMPOSITION_TABLE: &[(u32, u32, u32)] = &[")
    for (first, second), composed in comp_sorted:
        lines.append(f"    (0x{first:04X}, 0x{second:04X}, 0x{composed:04X}),")
    lines.append("];")
    lines.append("")

    # --- Lookup functions ---
    lines.append("/// Look up Canonical Combining Class for a code point. Returns 0 if not found.")
    lines.append("#[must_use]")
    lines.append("pub fn ccc(cp: u32) -> u8 {")
    lines.append("    match CCC_TABLE.binary_search_by_key(&cp, |&(c, _)| c) {")
    lines.append("        Ok(i) => CCC_TABLE[i].1,")
    lines.append("        Err(_) => 0,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up canonical decomposition for a code point.")
    lines.append("/// Returns None if the character has no canonical decomposition.")
    lines.append("#[must_use]")
    lines.append("pub fn canonical_decomposition(cp: u32) -> Option<&'static [u32]> {")
    lines.append("    match CANONICAL_DECOMP_INDEX.binary_search_by_key(&cp, |&(c, _, _)| c) {")
    lines.append("        Ok(i) => {")
    lines.append("            let (_, off, len) = CANONICAL_DECOMP_INDEX[i];")
    lines.append("            Some(&CANONICAL_DECOMP_DATA[off as usize..(off as usize + len as usize)])")
    lines.append("        }")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up compatibility decomposition for a code point.")
    lines.append("/// Returns the canonical decomposition if one exists, otherwise the")
    lines.append("/// compatibility-only decomposition. Returns None if no decomposition.")
    lines.append("#[must_use]")
    lines.append("pub fn compatibility_decomposition(cp: u32) -> Option<&'static [u32]> {")
    lines.append("    // Check canonical first")
    lines.append("    if let Some(d) = canonical_decomposition(cp) {")
    lines.append("        return Some(d);")
    lines.append("    }")
    lines.append("    // Check compatibility-only")
    lines.append("    match COMPAT_DECOMP_INDEX.binary_search_by_key(&cp, |&(c, _, _)| c) {")
    lines.append("        Ok(i) => {")
    lines.append("            let (_, off, len) = COMPAT_DECOMP_INDEX[i];")
    lines.append("            Some(&COMPAT_DECOMP_DATA[off as usize..(off as usize + len as usize)])")
    lines.append("        }")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up canonical composition: (first, second) -> composed.")
    lines.append("/// Returns None if no composition exists for this pair.")
    lines.append("#[must_use]")
    lines.append("pub fn canonical_composition(first: u32, second: u32) -> Option<u32> {")
    lines.append("    match COMPOSITION_TABLE.binary_search_by_key(&(first, second), |&(a, b, _)| (a, b)) {")
    lines.append("        Ok(i) => Some(COMPOSITION_TABLE[i].2),")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    return "\n".join(lines)


def main():
    unicode_data_path = UCD_DIR / "UnicodeData.txt"
    exclusions_path = UCD_DIR / "CompositionExclusions.txt"

    if not unicode_data_path.exists():
        print(f"ERROR: {unicode_data_path} not found. Run download_ucd_tests.ps1 first.")
        sys.exit(1)
    if not exclusions_path.exists():
        print(f"ERROR: {exclusions_path} not found. Run download_ucd_tests.ps1 first.")
        sys.exit(1)

    print("Parsing UnicodeData.txt ...")
    ccc_map, canonical_decomp, compat_decomp = parse_unicode_data(unicode_data_path)
    print(f"  CCC entries: {len(ccc_map)}")
    print(f"  Canonical decompositions: {len(canonical_decomp)}")
    print(f"  Compatibility decompositions: {len(compat_decomp)}")

    print("Parsing CompositionExclusions.txt ...")
    exclusions = parse_composition_exclusions(exclusions_path)
    print(f"  Explicit exclusions: {len(exclusions)}")

    print("Building composition table ...")
    compositions = build_composition_table(canonical_decomp, ccc_map, exclusions)
    print(f"  Composition pairs: {len(compositions)}")

    print("Generating Rust source ...")
    rust_src = generate_rust(ccc_map, canonical_decomp, compat_decomp, compositions)

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(rust_src)
    print(f"  Written to {OUTPUT}")

    size_kb = os.path.getsize(OUTPUT) / 1024
    print(f"  File size: {size_kb:.1f} KB")
    print("Done.")


if __name__ == "__main__":
    main()
