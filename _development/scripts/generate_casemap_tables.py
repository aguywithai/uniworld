"""
Generate Rust case mapping lookup tables from UCD data files.

Reads:
  _development/data/ucd/UnicodeData.txt       (simple case mappings: fields 12-14)
  _development/data/ucd/SpecialCasing.txt     (full/multi-char case mappings)
  _development/data/ucd/CaseFolding.txt       (case folding for case-insensitive matching)

Writes:
  src/data/casemap.rs

Run from repo root:
  python _development/scripts/generate_casemap_tables.py
"""

import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
UCD_DIR = REPO_ROOT / "_development" / "data" / "ucd"
OUTPUT = REPO_ROOT / "src" / "data" / "casemap.rs"


def parse_simple_case_mappings(path):
    """Parse UnicodeData.txt for simple case mappings (fields 12-14).

    Returns:
        lower: dict cp -> lower_cp  (Simple_Lowercase_Mapping)
        upper: dict cp -> upper_cp  (Simple_Uppercase_Mapping)
        title: dict cp -> title_cp  (Simple_Titlecase_Mapping)
    """
    lower = {}
    upper = {}
    title = {}

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            fields = line.split(";")
            if len(fields) < 15:
                continue

            cp = int(fields[0], 16)
            upper_field = fields[12].strip()
            lower_field = fields[13].strip()
            title_field = fields[14].strip()

            if upper_field:
                upper[cp] = int(upper_field, 16)
            if lower_field:
                lower[cp] = int(lower_field, 16)
            if title_field:
                tc = int(title_field, 16)
                # Only store titlecase if it differs from uppercase
                uc = upper.get(cp)
                if tc != uc:
                    title[cp] = tc

    return lower, upper, title


def parse_special_casing(path):
    """Parse SpecialCasing.txt for full (multi-char) case mappings.

    Returns:
        unconditional: list of (cp, lower_cps, title_cps, upper_cps) --
                       only entries with no condition
        conditional: list of (cp, lower_cps, title_cps, upper_cps, condition_str) --
                     entries with language/context conditions
    """
    unconditional = []
    conditional = []

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            # Strip inline comment
            if "#" in line:
                line = line[:line.index("#")].strip()

            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 4:
                continue

            cp = int(parts[0], 16)
            lower_cps = [int(x, 16) for x in parts[1].split()] if parts[1] else [cp]
            title_cps = [int(x, 16) for x in parts[2].split()] if parts[2] else [cp]
            upper_cps = [int(x, 16) for x in parts[3].split()] if parts[3] else [cp]

            # Check for condition (5th field)
            condition = parts[4].strip() if len(parts) > 4 and parts[4].strip() else None

            if condition:
                conditional.append((cp, lower_cps, title_cps, upper_cps, condition))
            else:
                unconditional.append((cp, lower_cps, title_cps, upper_cps))

    return unconditional, conditional


def parse_case_folding(path):
    """Parse CaseFolding.txt.

    Returns:
        simple_fold: dict cp -> fold_cp  (status C or S, single char)
        full_fold: dict cp -> [fold_cps]  (status C or F, possibly multi-char)
        turkic_fold: dict cp -> fold_cp  (status T, Turkish locale)
    """
    simple_fold = {}
    full_fold = {}
    turkic_fold = {}

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue

            parts = [p.strip() for p in line.split(";")]
            if len(parts) < 3:
                continue

            cp = int(parts[0], 16)
            status = parts[1].strip()
            mapping = [int(x, 16) for x in parts[2].split()]

            if status == "C":
                # Common: used by both simple and full
                simple_fold[cp] = mapping[0]
                full_fold[cp] = mapping
            elif status == "S":
                # Simple only: single char, used when C is not available
                simple_fold[cp] = mapping[0]
            elif status == "F":
                # Full only: multi-char, used instead of C for full folding
                full_fold[cp] = mapping
            elif status == "T":
                # Turkic: special for tr/az locales
                turkic_fold[cp] = mapping[0]

    return simple_fold, full_fold, turkic_fold


def generate_rust(simple_lower, simple_upper, simple_title,
                  special_unconditional, special_conditional,
                  simple_fold, full_fold, turkic_fold):
    """Generate Rust source file with case mapping tables."""

    lines = []
    lines.append("//! Auto-generated Unicode case mapping tables.")
    lines.append("//! Do not edit manually.")
    lines.append("//! Generated from UnicodeData.txt, SpecialCasing.txt, CaseFolding.txt (Unicode 17.0).")
    lines.append("//!")
    lines.append("//! Run: python _development/scripts/generate_casemap_tables.py")
    lines.append("")

    # --- Simple lowercase table ---
    sl = sorted(simple_lower.items())
    lines.append(f"/// Simple lowercase mappings: (source_cp, lowercase_cp).")
    lines.append(f"/// {len(sl)} entries, sorted by source for binary search.")
    lines.append(f"pub const SIMPLE_LOWERCASE: &[(u32, u32)] = &[")
    for cp, lc in sl:
        lines.append(f"    (0x{cp:04X}, 0x{lc:04X}),")
    lines.append("];")
    lines.append("")

    # --- Simple uppercase table ---
    su = sorted(simple_upper.items())
    lines.append(f"/// Simple uppercase mappings: (source_cp, uppercase_cp).")
    lines.append(f"/// {len(su)} entries, sorted by source for binary search.")
    lines.append(f"pub const SIMPLE_UPPERCASE: &[(u32, u32)] = &[")
    for cp, uc in su:
        lines.append(f"    (0x{cp:04X}, 0x{uc:04X}),")
    lines.append("];")
    lines.append("")

    # --- Simple titlecase table (only where title != upper) ---
    st = sorted(simple_title.items())
    lines.append(f"/// Simple titlecase mappings where titlecase differs from uppercase.")
    lines.append(f"/// {len(st)} entries, sorted by source for binary search.")
    lines.append(f"pub const SIMPLE_TITLECASE: &[(u32, u32)] = &[")
    for cp, tc in st:
        lines.append(f"    (0x{cp:04X}, 0x{tc:04X}),")
    lines.append("];")
    lines.append("")

    # --- Full uppercase mappings from SpecialCasing (unconditional, multi-char) ---
    # Only include entries where the full mapping differs from simple
    full_upper_entries = []
    for cp, _lower_cps, _title_cps, upper_cps in special_unconditional:
        if len(upper_cps) > 1 or (len(upper_cps) == 1 and upper_cps[0] != simple_upper.get(cp, cp)):
            full_upper_entries.append((cp, upper_cps))
    full_upper_entries.sort(key=lambda x: x[0])

    fu_data = []
    fu_index = []
    offset = 0
    for cp, cps in full_upper_entries:
        fu_index.append((cp, offset, len(cps)))
        fu_data.extend(cps)
        offset += len(cps)

    lines.append(f"/// Full uppercase index: (source_cp, data_offset, length).")
    lines.append(f"/// {len(fu_index)} entries from SpecialCasing.txt (unconditional).")
    lines.append(f"pub const FULL_UPPERCASE_INDEX: &[(u32, u16, u8)] = &[")
    for cp, off, ln in fu_index:
        lines.append(f"    (0x{cp:04X}, {off}, {ln}),")
    lines.append("];")
    lines.append("")

    lines.append(f"/// Full uppercase data (flattened code points).")
    lines.append(f"pub const FULL_UPPERCASE_DATA: &[u32] = &[")
    chunk_size = 12
    for i in range(0, len(fu_data), chunk_size):
        chunk = fu_data[i:i + chunk_size]
        vals = ", ".join(f"0x{v:04X}" for v in chunk)
        lines.append(f"    {vals},")
    lines.append("];")
    lines.append("")

    # --- Full titlecase mappings from SpecialCasing ---
    full_title_entries = []
    for cp, _lower_cps, title_cps, _upper_cps in special_unconditional:
        if len(title_cps) > 1 or (len(title_cps) == 1 and title_cps[0] != simple_title.get(cp, simple_upper.get(cp, cp))):
            full_title_entries.append((cp, title_cps))
    full_title_entries.sort(key=lambda x: x[0])

    ft_data = []
    ft_index = []
    offset = 0
    for cp, cps in full_title_entries:
        ft_index.append((cp, offset, len(cps)))
        ft_data.extend(cps)
        offset += len(cps)

    lines.append(f"/// Full titlecase index: (source_cp, data_offset, length).")
    lines.append(f"/// {len(ft_index)} entries from SpecialCasing.txt (unconditional).")
    lines.append(f"pub const FULL_TITLECASE_INDEX: &[(u32, u16, u8)] = &[")
    for cp, off, ln in ft_index:
        lines.append(f"    (0x{cp:04X}, {off}, {ln}),")
    lines.append("];")
    lines.append("")

    lines.append(f"/// Full titlecase data (flattened code points).")
    lines.append(f"pub const FULL_TITLECASE_DATA: &[u32] = &[")
    for i in range(0, len(ft_data), chunk_size):
        chunk = ft_data[i:i + chunk_size]
        vals = ", ".join(f"0x{v:04X}" for v in chunk)
        lines.append(f"    {vals},")
    lines.append("];")
    lines.append("")

    # --- Full lowercase mappings from SpecialCasing ---
    full_lower_entries = []
    for cp, lower_cps, _title_cps, _upper_cps in special_unconditional:
        if len(lower_cps) > 1 or (len(lower_cps) == 1 and lower_cps[0] != simple_lower.get(cp, cp)):
            full_lower_entries.append((cp, lower_cps))
    full_lower_entries.sort(key=lambda x: x[0])

    fl_data = []
    fl_index = []
    offset = 0
    for cp, cps in full_lower_entries:
        fl_index.append((cp, offset, len(cps)))
        fl_data.extend(cps)
        offset += len(cps)

    lines.append(f"/// Full lowercase index: (source_cp, data_offset, length).")
    lines.append(f"/// {len(fl_index)} entries from SpecialCasing.txt (unconditional).")
    lines.append(f"pub const FULL_LOWERCASE_INDEX: &[(u32, u16, u8)] = &[")
    for cp, off, ln in fl_index:
        lines.append(f"    (0x{cp:04X}, {off}, {ln}),")
    lines.append("];")
    lines.append("")

    lines.append(f"/// Full lowercase data (flattened code points).")
    lines.append(f"pub const FULL_LOWERCASE_DATA: &[u32] = &[")
    for i in range(0, len(fl_data), chunk_size):
        chunk = fl_data[i:i + chunk_size]
        vals = ", ".join(f"0x{v:04X}" for v in chunk)
        lines.append(f"    {vals},")
    lines.append("];")
    lines.append("")

    # --- Simple case folding table (C + S) ---
    sf = sorted(simple_fold.items())
    lines.append(f"/// Simple case folding: (source_cp, folded_cp).")
    lines.append(f"/// {len(sf)} entries (status C + S), sorted for binary search.")
    lines.append(f"pub const SIMPLE_CASE_FOLD: &[(u32, u32)] = &[")
    for cp, fc in sf:
        lines.append(f"    (0x{cp:04X}, 0x{fc:04X}),")
    lines.append("];")
    lines.append("")

    # --- Full case folding (C + F) - only multi-char entries ---
    full_fold_multi = {cp: cps for cp, cps in full_fold.items() if len(cps) > 1}
    ff_sorted = sorted(full_fold_multi.items())
    ff_data = []
    ff_index = []
    offset = 0
    for cp, cps in ff_sorted:
        ff_index.append((cp, offset, len(cps)))
        ff_data.extend(cps)
        offset += len(cps)

    lines.append(f"/// Full case folding index (multi-char entries only): (source_cp, data_offset, length).")
    lines.append(f"/// {len(ff_index)} entries (status F), sorted for binary search.")
    lines.append(f"pub const FULL_CASE_FOLD_INDEX: &[(u32, u16, u8)] = &[")
    for cp, off, ln in ff_index:
        lines.append(f"    (0x{cp:04X}, {off}, {ln}),")
    lines.append("];")
    lines.append("")

    lines.append(f"/// Full case folding data (flattened code points).")
    lines.append(f"pub const FULL_CASE_FOLD_DATA: &[u32] = &[")
    for i in range(0, len(ff_data), chunk_size):
        chunk = ff_data[i:i + chunk_size]
        vals = ", ".join(f"0x{v:04X}" for v in chunk)
        lines.append(f"    {vals},")
    lines.append("];")
    lines.append("")

    # --- Turkic case folding ---
    tf = sorted(turkic_fold.items())
    lines.append(f"/// Turkic case folding overrides: (source_cp, folded_cp).")
    lines.append(f"/// {len(tf)} entries (status T), for tr/az locales.")
    lines.append(f"pub const TURKIC_CASE_FOLD: &[(u32, u32)] = &[")
    for cp, fc in tf:
        lines.append(f"    (0x{cp:04X}, 0x{fc:04X}),")
    lines.append("];")
    lines.append("")

    # --- Soft_Dotted property (for Lithuanian and Turkish handling) ---
    # Characters with Soft_Dotted=True: i, j, and a few others
    # We embed a short list based on Unicode data
    soft_dotted = [
        0x0069, 0x006A,  # Latin i, j
        0x012F,  # Latin i with ogonek
        0x0249,  # Latin j with stroke
        0x0268,  # Latin i with stroke
        0x029D,  # Latin j with crossed tail
        0x02B2,  # Modifier letter small j
        0x03F3,  # Greek letter yot
        0x0456,  # Cyrillic i
        0x0458,  # Cyrillic je
        0x1D62,  # Latin subscript i
        0x1D96,  # Latin small letter i with retroflex hook
        0x1DA4,  # Modifier letter small i with stroke
        0x1DA8,  # Modifier letter small j with crossed tail
        0x1E2D,  # Latin small letter i with tilde below
        0x1ECB,  # Latin small letter i with dot below
        0x2071,  # Superscript i
        0x2148, 0x2149,  # Double-struck i, j
    ]
    lines.append(f"/// Soft_Dotted characters (for Turkish/Lithuanian case handling).")
    lines.append(f"/// Sorted for binary search.")
    lines.append(f"pub const SOFT_DOTTED: &[u32] = &[")
    for cp in sorted(soft_dotted):
        lines.append(f"    0x{cp:04X},")
    lines.append("];")
    lines.append("")

    # --- Lookup functions ---
    lines.append("/// Look up simple lowercase mapping. Returns None if no mapping.")
    lines.append("#[must_use]")
    lines.append("pub fn simple_lowercase(cp: u32) -> Option<u32> {")
    lines.append("    match SIMPLE_LOWERCASE.binary_search_by_key(&cp, |&(c, _)| c) {")
    lines.append("        Ok(i) => Some(SIMPLE_LOWERCASE[i].1),")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up simple uppercase mapping. Returns None if no mapping.")
    lines.append("#[must_use]")
    lines.append("pub fn simple_uppercase(cp: u32) -> Option<u32> {")
    lines.append("    match SIMPLE_UPPERCASE.binary_search_by_key(&cp, |&(c, _)| c) {")
    lines.append("        Ok(i) => Some(SIMPLE_UPPERCASE[i].1),")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up simple titlecase mapping. Falls back to uppercase if no specific titlecase.")
    lines.append("#[must_use]")
    lines.append("pub fn simple_titlecase(cp: u32) -> Option<u32> {")
    lines.append("    match SIMPLE_TITLECASE.binary_search_by_key(&cp, |&(c, _)| c) {")
    lines.append("        Ok(i) => Some(SIMPLE_TITLECASE[i].1),")
    lines.append("        Err(_) => simple_uppercase(cp),")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up full uppercase mapping (multi-char). Returns None if no full mapping.")
    lines.append("#[must_use]")
    lines.append("pub fn full_uppercase(cp: u32) -> Option<&'static [u32]> {")
    lines.append("    match FULL_UPPERCASE_INDEX.binary_search_by_key(&cp, |&(c, _, _)| c) {")
    lines.append("        Ok(i) => {")
    lines.append("            let (_, off, len) = FULL_UPPERCASE_INDEX[i];")
    lines.append("            Some(&FULL_UPPERCASE_DATA[off as usize..(off as usize + len as usize)])")
    lines.append("        }")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up full titlecase mapping (multi-char). Returns None if no full mapping.")
    lines.append("#[must_use]")
    lines.append("pub fn full_titlecase(cp: u32) -> Option<&'static [u32]> {")
    lines.append("    match FULL_TITLECASE_INDEX.binary_search_by_key(&cp, |&(c, _, _)| c) {")
    lines.append("        Ok(i) => {")
    lines.append("            let (_, off, len) = FULL_TITLECASE_INDEX[i];")
    lines.append("            Some(&FULL_TITLECASE_DATA[off as usize..(off as usize + len as usize)])")
    lines.append("        }")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up full lowercase mapping (multi-char). Returns None if no full mapping.")
    lines.append("#[must_use]")
    lines.append("pub fn full_lowercase(cp: u32) -> Option<&'static [u32]> {")
    lines.append("    match FULL_LOWERCASE_INDEX.binary_search_by_key(&cp, |&(c, _, _)| c) {")
    lines.append("        Ok(i) => {")
    lines.append("            let (_, off, len) = FULL_LOWERCASE_INDEX[i];")
    lines.append("            Some(&FULL_LOWERCASE_DATA[off as usize..(off as usize + len as usize)])")
    lines.append("        }")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up simple case folding. Returns None if character folds to itself.")
    lines.append("#[must_use]")
    lines.append("pub fn simple_case_fold(cp: u32) -> Option<u32> {")
    lines.append("    match SIMPLE_CASE_FOLD.binary_search_by_key(&cp, |&(c, _)| c) {")
    lines.append("        Ok(i) => Some(SIMPLE_CASE_FOLD[i].1),")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up full case folding (multi-char). Returns None if no multi-char folding.")
    lines.append("#[must_use]")
    lines.append("pub fn full_case_fold(cp: u32) -> Option<&'static [u32]> {")
    lines.append("    match FULL_CASE_FOLD_INDEX.binary_search_by_key(&cp, |&(c, _, _)| c) {")
    lines.append("        Ok(i) => {")
    lines.append("            let (_, off, len) = FULL_CASE_FOLD_INDEX[i];")
    lines.append("            Some(&FULL_CASE_FOLD_DATA[off as usize..(off as usize + len as usize)])")
    lines.append("        }")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Look up Turkic case folding override. Returns None if no Turkic override.")
    lines.append("#[must_use]")
    lines.append("pub fn turkic_case_fold(cp: u32) -> Option<u32> {")
    lines.append("    match TURKIC_CASE_FOLD.binary_search_by_key(&cp, |&(c, _)| c) {")
    lines.append("        Ok(i) => Some(TURKIC_CASE_FOLD[i].1),")
    lines.append("        Err(_) => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    lines.append("/// Check if a code point has the Soft_Dotted property.")
    lines.append("#[must_use]")
    lines.append("pub fn is_soft_dotted(cp: u32) -> bool {")
    lines.append("    SOFT_DOTTED.binary_search(&cp).is_ok()")
    lines.append("}")
    lines.append("")

    return "\n".join(lines)


def main():
    unicode_data_path = UCD_DIR / "UnicodeData.txt"
    special_casing_path = UCD_DIR / "SpecialCasing.txt"
    case_folding_path = UCD_DIR / "CaseFolding.txt"

    for p, name in [(unicode_data_path, "UnicodeData.txt"),
                    (special_casing_path, "SpecialCasing.txt"),
                    (case_folding_path, "CaseFolding.txt")]:
        if not p.exists():
            print(f"ERROR: {p} not found. Run download_ucd_tests.ps1 first.")
            sys.exit(1)

    print("Parsing UnicodeData.txt for simple case mappings ...")
    simple_lower, simple_upper, simple_title = parse_simple_case_mappings(unicode_data_path)
    print(f"  Simple lowercase: {len(simple_lower)} entries")
    print(f"  Simple uppercase: {len(simple_upper)} entries")
    print(f"  Simple titlecase (differs from upper): {len(simple_title)} entries")

    print("Parsing SpecialCasing.txt ...")
    unconditional, conditional = parse_special_casing(special_casing_path)
    print(f"  Unconditional full mappings: {len(unconditional)} entries")
    print(f"  Conditional mappings: {len(conditional)} entries (handled in code)")

    print("Parsing CaseFolding.txt ...")
    simple_fold, full_fold, turkic_fold = parse_case_folding(case_folding_path)
    full_multi = {cp: cps for cp, cps in full_fold.items() if len(cps) > 1}
    print(f"  Simple case fold (C+S): {len(simple_fold)} entries")
    print(f"  Full case fold multi-char (F): {len(full_multi)} entries")
    print(f"  Turkic overrides (T): {len(turkic_fold)} entries")

    print("Generating Rust source ...")
    rust_src = generate_rust(
        simple_lower, simple_upper, simple_title,
        unconditional, conditional,
        simple_fold, full_fold, turkic_fold,
    )

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(rust_src)
    print(f"  Written to {OUTPUT}")

    size_kb = os.path.getsize(OUTPUT) / 1024
    print(f"  File size: {size_kb:.1f} KB")
    print("Done.")


if __name__ == "__main__":
    main()
