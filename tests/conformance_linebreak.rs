//! Conformance tests for UAX #14 Line Breaking Algorithm.
//!
//! Parses LineBreakTest.txt and validates that our implementation produces
//! the correct break/no-break decisions for each test case.

use uniworld::linebreak::{line_break_opportunities, BreakAction};

/// Parse a LineBreakTest.txt line into (code_points, break_positions).
/// Format: "x 0041 x 0020 / 0042 /" where x = no-break, / = break.
/// (The actual file uses Unicode division sign and multiplication sign.)
fn parse_test_line(line: &str) -> Option<(Vec<u32>, Vec<bool>)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    // Strip comment.
    let data = if let Some(idx) = line.find('#') {
        &line[..idx]
    } else {
        line
    };
    let data = data.trim();
    if data.is_empty() {
        return None;
    }

    let mut code_points: Vec<u32> = Vec::new();
    // break_at[i] = whether there's a break BEFORE code_points[i].
    // Plus one extra for "after last" (end of text).
    let mut break_before: Vec<bool> = Vec::new();

    // Tokens are hex code points and break indicators.
    // The division sign (/) means break, multiplication (x) means no-break.
    // In the actual file, these are Unicode chars but might be represented
    // differently. Let's handle both ASCII and Unicode.
    let tokens: Vec<&str> = data.split_whitespace().collect();
    let mut expect_break_indicator = true;
    for tok in &tokens {
        if expect_break_indicator {
            // This should be a break indicator.
            let is_break = if *tok == "/" {
                true
            } else if *tok == "x" {
                false
            } else if tok.contains('\u{00F7}') {
                // Division sign
                true
            } else if tok.contains('\u{00D7}') {
                // Multiplication sign
                false
            } else {
                // Try to parse as hex (might be a continuation).
                // Shouldn't happen with well-formed data.
                continue;
            };
            break_before.push(is_break);
            expect_break_indicator = false;
        } else {
            // This should be a hex code point.
            if let Ok(cp) = u32::from_str_radix(tok, 16) {
                code_points.push(cp);
                expect_break_indicator = true;
            }
            // If it's another break indicator (shouldn't happen), skip.
        }
    }

    if code_points.is_empty() {
        return None;
    }

    Some((code_points, break_before))
}

#[test]
fn linebreak_conformance_test() {
    let test_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/_development/data/ucd/LineBreakTest.txt"
    );
    let data = std::fs::read_to_string(test_path).expect("Failed to read LineBreakTest.txt");

    let mut passed = 0u32;
    let mut failed = 0u32;
    let mut first_failures: Vec<String> = Vec::new();

    for (line_no, line) in data.lines().enumerate() {
        let (cps, expected_breaks) = match parse_test_line(line) {
            Some(v) => v,
            None => continue,
        };

        // Convert code points to a string.
        let text: String = cps
            .iter()
            .filter_map(|&cp| char::from_u32(cp))
            .collect();

        // Get our break decisions.
        let breaks = line_break_opportunities(&text);

        // Build the expected break array at char boundaries.
        // expected_breaks has len = cps.len() + 1
        //   [0] = break before first char (sot, should be no-break per LB2... but test says break)
        //   [1..n] = break before each subsequent char
        //   [n] = break after last char (eot, mandatory per LB3)
        // Actually, the test format:
        //   The first token is a break indicator before the first code point.
        //   Then alternating: code_point, break_indicator, code_point, ...
        //   The last token is a break indicator after the last code point.
        // So expected_breaks.len() == cps.len() + 1.

        if expected_breaks.len() != cps.len() + 1 {
            // Malformed test line, skip.
            continue;
        }

        // Map expected breaks to byte positions.
        let char_boundaries: Vec<usize> = {
            let mut bounds = Vec::new();
            bounds.push(0); // before first char
            for (idx, ch) in text.char_indices() {
                bounds.push(idx + ch.len_utf8()); // after each char
            }
            bounds
        };

        // Verify. Note: our breaks array is indexed by byte position.
        let mut ok = true;
        for (k, &expected) in expected_breaks.iter().enumerate() {
            if k >= char_boundaries.len() {
                ok = false;
                break;
            }
            let byte_pos = char_boundaries[k];
            if byte_pos >= breaks.len() {
                ok = false;
                break;
            }

            let actual_break = breaks[byte_pos] == BreakAction::Allowed
                || breaks[byte_pos] == BreakAction::Mandatory;

            if actual_break != expected {
                ok = false;
                break;
            }
        }

        if ok {
            passed += 1;
        } else {
            failed += 1;
            if first_failures.len() < 20 {
                // Build debug info.
                let expected_str: String = expected_breaks
                    .iter()
                    .zip(
                        cps.iter()
                            .map(|cp| format!("{:04X}", cp))
                            .chain(std::iter::once(String::new())),
                    )
                    .map(|(&brk, cp)| {
                        let sym = if brk { "/" } else { "x" };
                        if cp.is_empty() {
                            sym.to_string()
                        } else {
                            format!("{} {}", sym, cp)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                let actual_str: String = {
                    let mut parts = Vec::new();
                    for (k, &_expected) in expected_breaks.iter().enumerate() {
                        if k >= char_boundaries.len() || char_boundaries[k] >= breaks.len()
                        {
                            parts.push("?".to_string());
                            continue;
                        }
                        let byte_pos = char_boundaries[k];
                        let actual = breaks[byte_pos] == BreakAction::Allowed
                            || breaks[byte_pos] == BreakAction::Mandatory;
                        parts.push(if actual {
                            "/".to_string()
                        } else {
                            "x".to_string()
                        });
                    }
                    parts.join(" ")
                };

                first_failures.push(format!(
                    "  Line {}: expected [{}] got [{}]",
                    line_no + 1,
                    expected_str,
                    actual_str
                ));
            }
        }
    }

    println!(
        "LineBreakTest: {}/{} passed.",
        passed,
        passed + failed
    );

    if !first_failures.is_empty() {
        println!("First failures:");
        for f in &first_failures {
            println!("{}", f);
        }
    }

    assert_eq!(
        failed, 0,
        "LineBreakTest: {} failures out of {}",
        failed,
        passed + failed
    );
}
