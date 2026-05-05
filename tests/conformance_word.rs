//! UAX #29 word boundary conformance tests using WordBreakTest.txt.
//! Download: run _development/scripts/download_ucd_tests.ps1 from repo root.
//! If the file is missing, this test is skipped (passes without running).

use std::env;
use std::fs;

use uniworld::segment::word_boundaries;

/// Parse one line of WordBreakTest.txt: "(÷|×) HEX (÷|×) HEX ..."
fn parse_line(line: &str) -> Option<(String, Vec<usize>)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut i = 0;
    let mut s = String::new();
    let mut expected = Vec::new();
    let mut byte_off = 0usize;
    while i < tokens.len() {
        let break_before = match tokens[i] {
            "÷" => true,
            "×" => false,
            _ => {
                i += 1;
                continue;
            }
        };
        i += 1;
        if i >= tokens.len() {
            break;
        }
        let hex = tokens[i];
        if hex.starts_with('#') {
            break;
        }
        let cp = u32::from_str_radix(hex, 16).ok()?;
        if break_before {
            expected.push(byte_off);
        }
        if let Some(ch) = char::from_u32(cp) {
            s.push(ch);
            byte_off += ch.len_utf8();
        }
        i += 1;
    }
    if s.is_empty() {
        return None;
    }
    Some((s, expected))
}

#[test]
fn word_conformance_word_break_test() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let default_path = format!("{}/_development/data/ucd/WordBreakTest.txt", manifest);
    let path = env::var("UNICORE_WORD_TEST").unwrap_or(default_path);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Skipping word conformance test: {} not found. Run _development/scripts/download_ucd_tests.ps1", path);
            return;
        }
    };
    let mut passed = 0u32;
    let mut failed = 0u32;
    for (line_num, line) in content.lines().enumerate() {
        let line_no = line_num + 1;
        let Some((s, expected)) = parse_line(line) else { continue };
        let got = word_boundaries(&s, None);
        if got != expected {
            if failed < 10 {
                eprintln!("Line {}: expected {:?}, got {:?} for {:?}", line_no, expected, got, s);
            }
            failed += 1;
        } else {
            passed += 1;
        }
    }
    assert!(failed == 0, "{} word conformance failures ({} passed). Fix WB property/rules or update test data.", failed, passed);
}
