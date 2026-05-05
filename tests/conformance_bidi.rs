//! UAX #9 bidi conformance tests using BidiCharacterTest.txt and BidiTest.txt.

use uniworld::bidi::{self, Class};

// ---------------------------------------------------------------------------
// BidiCharacterTest.txt: character-level tests
// ---------------------------------------------------------------------------

/// Parse a hex code point sequence like "05D0 05D1 0028 05D2"
fn parse_cps(field: &str) -> Vec<u32> {
    field
        .split_whitespace()
        .map(|tok| u32::from_str_radix(tok, 16).unwrap())
        .collect()
}

/// Parse a level list like "1 1 0 1 1 0 0 0 0 0 0 0 0 0"
/// 'x' means removed (u8::MAX).
fn parse_levels(field: &str) -> Vec<u8> {
    field
        .split_whitespace()
        .map(|tok| {
            if tok == "x" {
                u8::MAX
            } else {
                tok.parse::<u8>().unwrap()
            }
        })
        .collect()
}

/// Parse an ordering list like "1 0 2 4 3 5 6 7 8 9 10 11 12 13"
fn parse_order(field: &str) -> Vec<usize> {
    let trimmed = field.trim();
    if trimmed.is_empty() {
        return vec![];
    }
    trimmed
        .split_whitespace()
        .map(|tok| tok.parse::<usize>().unwrap())
        .collect()
}

#[test]
fn bidi_character_test() {
    let data =
        std::fs::read_to_string("_development/data/ucd/BidiCharacterTest.txt").unwrap();

    let mut total = 0;
    let mut failed = 0;
    let mut first_failures: Vec<String> = Vec::new();

    for (line_no, line) in data.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = line.split(';').collect();
        if fields.len() < 5 {
            continue;
        }

        let cps = parse_cps(fields[0].trim());
        let para_dir: u8 = fields[1].trim().parse().unwrap();
        let expected_para_level: u8 = fields[2].trim().parse().unwrap();
        let expected_levels = parse_levels(fields[3].trim());
        let expected_order = parse_order(fields[4].trim());

        // Build string from code points.
        let s: String = cps
            .iter()
            .filter_map(|&cp| char::from_u32(cp))
            .collect();

        let para_override = match para_dir {
            0 => Some(0u8), // LTR
            1 => Some(1u8), // RTL
            2 => None,      // auto
            _ => continue,
        };

        let info = bidi::resolve(&s, para_override);
        total += 1;

        let levels_match = info.levels == expected_levels;
        let para_match = info.paragraph_level == expected_para_level;
        let order_match = info.reorder == expected_order;

        if !levels_match || !para_match || !order_match {
            failed += 1;
            if first_failures.len() < 20 {
                first_failures.push(format!(
                    "Line {}: cps={:?} dir={} expected_para={} got_para={}\n\
                     expected_levels={:?}\n\
                     got_levels    ={:?}\n\
                     expected_order={:?}\n\
                     got_order     ={:?}",
                    line_no + 1,
                    cps,
                    para_dir,
                    expected_para_level,
                    info.paragraph_level,
                    expected_levels,
                    info.levels,
                    expected_order,
                    info.reorder,
                ));
            }
        }
    }

    if failed > 0 {
        let msg = format!(
            "BidiCharacterTest: {}/{} failed.\nFirst failures:\n{}",
            failed,
            total,
            first_failures.join("\n\n")
        );
        panic!("{}", msg);
    } else {
        eprintln!("BidiCharacterTest: {}/{} passed.", total, total);
    }
}

// ---------------------------------------------------------------------------
// BidiTest.txt: class-level tests
// ---------------------------------------------------------------------------

fn name_to_class(name: &str) -> Class {
    match name {
        "L" => Class::L,
        "R" => Class::R,
        "AL" => Class::AL,
        "EN" => Class::EN,
        "ES" => Class::ES,
        "ET" => Class::ET,
        "AN" => Class::AN,
        "CS" => Class::CS,
        "NSM" => Class::NSM,
        "BN" => Class::BN,
        "B" => Class::B,
        "S" => Class::S,
        "WS" => Class::WS,
        "ON" => Class::ON,
        "LRE" => Class::LRE,
        "RLE" => Class::RLE,
        "LRO" => Class::LRO,
        "RLO" => Class::RLO,
        "PDF" => Class::PDF,
        "LRI" => Class::LRI,
        "RLI" => Class::RLI,
        "FSI" => Class::FSI,
        "PDI" => Class::PDI,
        _ => panic!("Unknown bidi class: {}", name),
    }
}

#[test]
fn bidi_class_test() {
    let data = std::fs::read_to_string("_development/data/ucd/BidiTest.txt").unwrap();

    let mut current_levels: Vec<u8> = Vec::new();
    let mut current_reorder: Vec<usize> = Vec::new();
    let mut total = 0;
    let mut failed = 0;
    let mut first_failures: Vec<String> = Vec::new();

    for (line_no, line) in data.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with("@Levels:") {
            let rest = line["@Levels:".len()..].trim();
            current_levels = parse_levels(rest);
            continue;
        }

        if line.starts_with("@Reorder:") {
            let rest = line["@Reorder:".len()..].trim();
            current_reorder = parse_order(rest);
            continue;
        }

        // Skip unknown @ directives
        if line.starts_with('@') {
            continue;
        }

        // Data line: <classes> ; <bitset>
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() < 2 {
            continue;
        }

        let classes: Vec<Class> = parts[0]
            .split_whitespace()
            .map(|tok| name_to_class(tok))
            .collect();

        let bitset: u8 = u8::from_str_radix(parts[1].trim(), 16).unwrap();

        // Test for each paragraph direction in the bitset.
        // Bit 0 (value 1) = auto-LTR, bit 1 (value 2) = LTR, bit 2 (value 4) = RTL
        let directions: [(u8, Option<u8>); 3] = [
            (1, None),    // auto
            (2, Some(0)), // LTR
            (4, Some(1)), // RTL
        ];

        for &(bit, para_override) in &directions {
            if bitset & bit == 0 {
                continue;
            }

            let info = bidi::resolve_classes(&classes, para_override);
            total += 1;

            // Compare levels (only non-x positions).
            let mut levels_ok = info.levels.len() == current_levels.len();
            if levels_ok {
                for j in 0..current_levels.len() {
                    if current_levels[j] == u8::MAX {
                        // Expected 'x': our level should also be x (u8::MAX)
                        // or we don't care about removed chars.
                        continue;
                    }
                    if info.levels[j] != current_levels[j] {
                        levels_ok = false;
                        break;
                    }
                }
            }

            let order_ok = info.reorder == current_reorder;

            if !levels_ok || !order_ok {
                failed += 1;
                if first_failures.len() < 20 {
                    first_failures.push(format!(
                        "Line {}: classes={:?} dir={:?}\n\
                         expected_levels={:?}\n\
                         got_levels    ={:?}\n\
                         expected_order={:?}\n\
                         got_order     ={:?}",
                        line_no + 1,
                        classes,
                        para_override,
                        current_levels,
                        info.levels,
                        current_reorder,
                        info.reorder,
                    ));
                }
            }
        }
    }

    if failed > 0 {
        let msg = format!(
            "BidiTest: {}/{} failed.\nFirst failures:\n{}",
            failed,
            total,
            first_failures.join("\n\n")
        );
        panic!("{}", msg);
    } else {
        eprintln!("BidiTest: {}/{} passed.", total, total);
    }
}
