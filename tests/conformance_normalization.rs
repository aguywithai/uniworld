//! UAX #15 normalization conformance tests using NormalizationTest.txt.
//! Download: run _development/scripts/download_ucd_tests.ps1 from repo root.
//! If the file is missing, this test is skipped (passes without running).
//!
//! The test file format: source;NFC;NFD;NFKC;NFKD
//! Conformance invariants:
//!   NFC:  c2 == toNFC(c1) == toNFC(c2) == toNFC(c3)
//!         c4 == toNFC(c4) == toNFC(c5)
//!   NFD:  c3 == toNFD(c1) == toNFD(c2) == toNFD(c3)
//!         c5 == toNFD(c4) == toNFD(c5)
//!   NFKC: c4 == toNFKC(c1) == toNFKC(c2) == toNFKC(c3) == toNFKC(c4) == toNFKC(c5)
//!   NFKD: c5 == toNFKD(c1) == toNFKD(c2) == toNFKD(c3) == toNFKD(c4) == toNFKD(c5)

use std::env;
use std::fs;

use uniworld::normalize::{nfc, nfd, nfkc, nfkd};

/// Parse a column of hex code points "HHHH HHHH ..." into a String.
fn parse_column(col: &str) -> String {
    col.trim()
        .split_whitespace()
        .filter_map(|hex| {
            let cp = u32::from_str_radix(hex, 16).ok()?;
            char::from_u32(cp)
        })
        .collect()
}

/// Parse one line of NormalizationTest.txt.
/// Returns (c1, c2, c3, c4, c5) as Strings, or None for comments/headers.
fn parse_line(line: &str) -> Option<(String, String, String, String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with('@') {
        return None;
    }
    let parts: Vec<&str> = line.split(';').collect();
    if parts.len() < 5 {
        return None;
    }
    Some((
        parse_column(parts[0]),
        parse_column(parts[1]),
        parse_column(parts[2]),
        parse_column(parts[3]),
        parse_column(parts[4]),
    ))
}

#[test]
fn normalization_conformance_test() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let default_path = format!(
        "{}/_development/data/ucd/NormalizationTest.txt",
        manifest
    );
    let path = env::var("UNICORE_NORM_TEST").unwrap_or(default_path);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!(
                "Skipping normalization conformance test: {} not found. \
                 Run _development/scripts/download_ucd_tests.ps1",
                path
            );
            return;
        }
    };

    let mut passed = 0u32;
    let mut failed = 0u32;

    for (line_num, line) in content.lines().enumerate() {
        let line_no = line_num + 1;
        let Some((c1, c2, c3, c4, c5)) = parse_line(line) else {
            continue;
        };

        // NFC invariants: c2 == toNFC(c1) == toNFC(c2) == toNFC(c3)
        let checks: Vec<(&str, String, &str)> = vec![
            ("NFC(c1)==c2", nfc(&c1), &c2),
            ("NFC(c2)==c2", nfc(&c2), &c2),
            ("NFC(c3)==c2", nfc(&c3), &c2),
            ("NFC(c4)==c4", nfc(&c4), &c4),
            ("NFC(c5)==c4", nfc(&c5), &c4),
            // NFD invariants: c3 == toNFD(c1) == toNFD(c2) == toNFD(c3)
            ("NFD(c1)==c3", nfd(&c1), &c3),
            ("NFD(c2)==c3", nfd(&c2), &c3),
            ("NFD(c3)==c3", nfd(&c3), &c3),
            ("NFD(c4)==c5", nfd(&c4), &c5),
            ("NFD(c5)==c5", nfd(&c5), &c5),
            // NFKC invariants: c4 == toNFKC(c1..c5)
            ("NFKC(c1)==c4", nfkc(&c1), &c4),
            ("NFKC(c2)==c4", nfkc(&c2), &c4),
            ("NFKC(c3)==c4", nfkc(&c3), &c4),
            ("NFKC(c4)==c4", nfkc(&c4), &c4),
            ("NFKC(c5)==c4", nfkc(&c5), &c4),
            // NFKD invariants: c5 == toNFKD(c1..c5)
            ("NFKD(c1)==c5", nfkd(&c1), &c5),
            ("NFKD(c2)==c5", nfkd(&c2), &c5),
            ("NFKD(c3)==c5", nfkd(&c3), &c5),
            ("NFKD(c4)==c5", nfkd(&c4), &c5),
            ("NFKD(c5)==c5", nfkd(&c5), &c5),
        ];

        let mut line_ok = true;
        for (label, got, expected) in &checks {
            if got != *expected {
                if failed < 20 {
                    let got_cps: Vec<String> = got.chars().map(|c| format!("{:04X}", c as u32)).collect();
                    let exp_cps: Vec<String> = expected.chars().map(|c| format!("{:04X}", c as u32)).collect();
                    eprintln!(
                        "Line {}: {} FAILED: got [{}], expected [{}]",
                        line_no,
                        label,
                        got_cps.join(" "),
                        exp_cps.join(" "),
                    );
                }
                line_ok = false;
                break; // Only report first failure per line
            }
        }

        if line_ok {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    eprintln!(
        "Normalization conformance: {} passed, {} failed",
        passed, failed
    );
    assert!(
        failed == 0,
        "{} normalization conformance failures ({} passed). \
         Fix normalization tables or algorithm.",
        failed,
        passed
    );
}
