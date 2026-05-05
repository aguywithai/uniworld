//! Sentence_Break property (UAX #29 Section 5).
//!
//! Classifies code points for default sentence boundary detection.
//! Full table from SentenceBreakProperty.txt can be used for full conformance.

/// Sentence_Break property value (UAX #29 Table 3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sb {
    /// Carriage return U+000D
    Cr,
    /// Line feed U+000A
    Lf,
    /// Paragraph separator (NEL, LS, PS)
    Sep,
    /// Combining marks (Mn, Mc, Me)
    Extend,
    /// Format characters (Cf except ZWNJ/ZWJ)
    Format,
    /// Whitespace (spaces, tab)
    Sp,
    /// Lowercase letters
    Lower,
    /// Uppercase letters
    Upper,
    /// Other letters (Lo, Lt, Lm, CJK, etc.)
    OLetter,
    /// Digits
    Numeric,
    /// Sentence-ending period (U+002E FULL STOP, etc.)
    ATerm,
    /// Sentence-ending terminal (!, ?, etc.)
    STerm,
    /// Closing punctuation (Pe, some Pf, quotes)
    Close,
    /// Sentence-continuing punctuation (comma, colon, dash)
    SContinue,
    /// Everything else
    Other,
}

/// Returns Sentence_Break for the given code point (UAX #29).
#[must_use]
pub fn sb(c: char) -> Sb {
    let u = c as u32;

    // CR, LF, Sep
    if u == 0x000D {
        return Sb::Cr;
    }
    if u == 0x000A {
        return Sb::Lf;
    }
    if matches!(u, 0x0085 | 0x2028 | 0x2029) {
        return Sb::Sep;
    }

    // ATerm (sentence-ending period)
    if is_aterm(u) {
        return Sb::ATerm;
    }

    // STerm (sentence-ending terminal: !, ?, etc.)
    if is_sterm(u) {
        return Sb::STerm;
    }

    // Format: Cf except ZWNJ (200C), ZWJ (200D)
    if is_sb_format(u) {
        return Sb::Format;
    }

    // Extend: combining marks (Mn, Me, Mc)
    if is_sb_extend(u) {
        return Sb::Extend;
    }

    // SContinue (comma, colon, dash, etc.)
    if is_scontinue(u) {
        return Sb::SContinue;
    }

    // Close: closing and quoting punctuation
    if is_close(u) {
        return Sb::Close;
    }

    // Sp: whitespace (Zs, tab)
    if is_sp(u) {
        return Sb::Sp;
    }

    // Numeric: decimal digits (Nd)
    if is_sb_numeric(u) {
        return Sb::Numeric;
    }

    // Lower: lowercase letters (Ll)
    if is_lower(u) {
        return Sb::Lower;
    }

    // Upper: uppercase letters (Lu)
    if is_upper(u) {
        return Sb::Upper;
    }

    // OLetter: other letters (Lo, Lt, Lm, some Nl)
    if is_oletter(u) {
        return Sb::OLetter;
    }

    Sb::Other
}

fn is_aterm(u: u32) -> bool {
    matches!(u, 0x002E | 0x2024 | 0xFE52 | 0xFF0E)
}

fn is_sterm(u: u32) -> bool {
    matches!(
        u,
        0x0021
            | 0x003F
            | 0x0589
            | 0x061E
            | 0x061F
            | 0x06D4
            | 0x0700..=0x0702
            | 0x07F9
            | 0x0837
            | 0x0839
            | 0x083D..=0x083E
            | 0x0964..=0x0965
            | 0x104A..=0x104B
            | 0x1362
            | 0x1367..=0x1368
            | 0x166E
            | 0x1735..=0x1736
            | 0x1803
            | 0x1809
            | 0x1944..=0x1945
            | 0x1AA8..=0x1AAB
            | 0x1B5A..=0x1B5B
            | 0x1B5E..=0x1B5F
            | 0x1C3B..=0x1C3C
            | 0x1C7E..=0x1C7F
            | 0x203C..=0x203D
            | 0x2047..=0x2049
            | 0x2E2E
            | 0x2E3C
            | 0x3002
            | 0xA4FF
            | 0xA60E..=0xA60F
            | 0xA6F3
            | 0xA6F7
            | 0xA876..=0xA877
            | 0xA8CE..=0xA8CF
            | 0xA92F
            | 0xA9C8..=0xA9C9
            | 0xAA5D..=0xAA5F
            | 0xAAF0..=0xAAF1
            | 0xABEB
            | 0xFE56..=0xFE57
            | 0xFF01
            | 0xFF1F
            | 0xFF61
    )
}

fn is_sb_format(u: u32) -> bool {
    if u == 0x200C || u == 0x200D {
        return false;
    }
    u == 0x00AD
        || (0x0600..=0x0605).contains(&u)
        || u == 0x06DD
        || u == 0x070F
        || (0x200E..=0x200F).contains(&u)
        || u == 0x200B
        || (0x202A..=0x202E).contains(&u)
        || (0x2060..=0x2064).contains(&u)
        || (0x2066..=0x206F).contains(&u)
        || u == 0xFEFF
        || u == 0xFFF9
        || u == 0xFFFA
        || u == 0xFFFB
}

fn is_sb_extend(u: u32) -> bool {
    // Reuse the same combining mark ranges as word_break::is_extend, plus ZWJ/ZWNJ
    // ZWJ (200D) and ZWNJ (200C) are Extend for sentence break purposes
    u == 0x200D
        || u == 0x200C
        || (0x0300..=0x036F).contains(&u)
        || (0x0483..=0x0489).contains(&u)
        || (0x0591..=0x05BD).contains(&u)
        || u == 0x05BF
        || (0x05C1..=0x05C2).contains(&u)
        || (0x05C4..=0x05C5).contains(&u)
        || u == 0x05C7
        || (0x0610..=0x061A).contains(&u)
        || (0x064B..=0x065F).contains(&u)
        || u == 0x0670
        || (0x06D6..=0x06DC).contains(&u)
        || (0x06DF..=0x06E4).contains(&u)
        || (0x06E7..=0x06E8).contains(&u)
        || (0x06EA..=0x06ED).contains(&u)
        || (0x0730..=0x074A).contains(&u)
        || (0x07A6..=0x07B0).contains(&u)
        || (0x0816..=0x0819).contains(&u)
        || (0x081B..=0x0823).contains(&u)
        || (0x0825..=0x0827).contains(&u)
        || (0x0829..=0x082D).contains(&u)
        || (0x0859..=0x085B).contains(&u)
        || (0x0900..=0x0903).contains(&u)
        || (0x093A..=0x094F).contains(&u)
        || (0x0951..=0x0957).contains(&u)
        || (0x0962..=0x0963).contains(&u)
        || (0x0981..=0x0983).contains(&u)
        || (0x09BC..=0x09CD).contains(&u)
        || u == 0x09D7
        || (0x09E2..=0x09E3).contains(&u)
        || (0x0A01..=0x0A03).contains(&u)
        || u == 0x0A3C
        || (0x0A3E..=0x0A42).contains(&u)
        || (0x0A47..=0x0A48).contains(&u)
        || (0x0A4B..=0x0A4D).contains(&u)
        || u == 0x0A51
        || (0x0A70..=0x0A71).contains(&u)
        || u == 0x0A75
        || (0x0A81..=0x0A83).contains(&u)
        || (0x0ABC..=0x0ACD).contains(&u)
        || (0x0AE2..=0x0AE3).contains(&u)
        || (0x0B01..=0x0B03).contains(&u)
        || (0x0B3C..=0x0B4D).contains(&u)
        || (0x0B56..=0x0B57).contains(&u)
        || (0x0B62..=0x0B63).contains(&u)
        || u == 0x0B82
        || (0x0BBE..=0x0BCD).contains(&u)
        || u == 0x0BD7
        || (0x0C00..=0x0C04).contains(&u)
        || (0x0C3E..=0x0C56).contains(&u)
        || (0x0C62..=0x0C63).contains(&u)
        || (0x0C81..=0x0C83).contains(&u)
        || (0x0CBC..=0x0CCD).contains(&u)
        || (0x0CD5..=0x0CD6).contains(&u)
        || (0x0CE2..=0x0CE3).contains(&u)
        || (0x0D00..=0x0D03).contains(&u)
        || (0x0D3B..=0x0D4D).contains(&u)
        || u == 0x0D57
        || (0x0D62..=0x0D63).contains(&u)
        || (0x0D82..=0x0D83).contains(&u)
        || (0x0DCA..=0x0DDF).contains(&u)
        || (0x0DF2..=0x0DF3).contains(&u)
        || u == 0x0E31
        || (0x0E34..=0x0E3A).contains(&u)
        || (0x0E47..=0x0E4E).contains(&u)
        || u == 0x0EB1
        || (0x0EB4..=0x0EBC).contains(&u)
        || (0x0EC8..=0x0ECE).contains(&u)
        || (0x0F18..=0x0F19).contains(&u)
        || u == 0x0F35
        || u == 0x0F37
        || u == 0x0F39
        || (0x0F3E..=0x0F3F).contains(&u)
        || (0x0F71..=0x0F84).contains(&u)
        || (0x0F86..=0x0F87).contains(&u)
        || (0x0F8D..=0x0FBC).contains(&u)
        || u == 0x0FC6
        || (0x102B..=0x103E).contains(&u)
        || (0x1056..=0x1059).contains(&u)
        || (0x105E..=0x1060).contains(&u)
        || (0x1062..=0x1064).contains(&u)
        || (0x1067..=0x106D).contains(&u)
        || (0x1071..=0x1074).contains(&u)
        || (0x1082..=0x108D).contains(&u)
        || u == 0x108F
        || (0x109A..=0x109D).contains(&u)
        || (0x135D..=0x135F).contains(&u)
        || (0x1712..=0x1714).contains(&u)
        || (0x1732..=0x1734).contains(&u)
        || (0x1752..=0x1753).contains(&u)
        || (0x1772..=0x1773).contains(&u)
        || (0x17B4..=0x17D3).contains(&u)
        || u == 0x17DD
        || (0x180B..=0x180D).contains(&u)
        || u == 0x18A9
        || (0x1920..=0x193B).contains(&u)
        || (0x1A17..=0x1A1B).contains(&u)
        || (0x1A55..=0x1A7F).contains(&u)
        || (0x1AB0..=0x1AC0).contains(&u)
        || (0x1B00..=0x1B04).contains(&u)
        || (0x1B34..=0x1B44).contains(&u)
        || (0x1B6B..=0x1B73).contains(&u)
        || (0x1B80..=0x1B82).contains(&u)
        || (0x1BA1..=0x1BAD).contains(&u)
        || (0x1BE6..=0x1BF3).contains(&u)
        || (0x1C24..=0x1C37).contains(&u)
        || (0x1CD0..=0x1CE8).contains(&u)
        || u == 0x1CED
        || (0x1CF4..=0x1CF9).contains(&u)
        || (0x1DC0..=0x1DFF).contains(&u)
        || (0x20D0..=0x20F0).contains(&u)
        || (0x2CEF..=0x2CF1).contains(&u)
        || u == 0x2D7F
        || (0x2DE0..=0x2DFF).contains(&u)
        || (0x302A..=0x302F).contains(&u)
        || (0x3099..=0x309A).contains(&u)
        || (0xA66F..=0xA672).contains(&u)
        || (0xA674..=0xA67D).contains(&u)
        || (0xA69E..=0xA69F).contains(&u)
        || (0xA6F0..=0xA6F1).contains(&u)
        || (0xA802..=0xA827).contains(&u)
        || u == 0xA82C
        || (0xA880..=0xA8C4).contains(&u)
        || (0xA8E0..=0xA8F1).contains(&u)
        || (0xA926..=0xA92D).contains(&u)
        || (0xA947..=0xA953).contains(&u)
        || (0xA980..=0xA983).contains(&u)
        || (0xA9B3..=0xA9C0).contains(&u)
        || u == 0xA9E5
        || (0xAA29..=0xAA36).contains(&u)
        || u == 0xAA43
        || (0xAA4C..=0xAA4D).contains(&u)
        || (0xAA7B..=0xAA7D).contains(&u)
        || (0xAAB0..=0xAABF).contains(&u)
        || u == 0xAAC1
        || (0xAAEB..=0xAAEF).contains(&u)
        || (0xAAF5..=0xAAF6).contains(&u)
        || (0xABE3..=0xABEA).contains(&u)
        || (0xABEC..=0xABED).contains(&u)
        || u == 0xFB1E
        || (0xFE00..=0xFE0F).contains(&u)
        || (0xFE20..=0xFE2F).contains(&u)
        || (0xFF9E..=0xFF9F).contains(&u)
        || (0x1F3FB..=0x1F3FF).contains(&u)
        || (0xE0100..=0xE01EF).contains(&u)
}

fn is_scontinue(u: u32) -> bool {
    matches!(
        u,
        0x002C
            | 0x002D
            | 0x003A
            | 0x055D
            | 0x060C
            | 0x060D
            | 0x07F8
            | 0x1802
            | 0x1808
            | 0x2013..=0x2014
            | 0x3001
            | 0xFE10..=0xFE11
            | 0xFE50..=0xFE51
            | 0xFF0C
            | 0xFF1A
    )
}

fn is_close(u: u32) -> bool {
    // General_Category Pe (close punctuation) and Pf (final punctuation)
    // plus some Pi (initial punctuation) that function as Close in sentence context
    matches!(
        u,
        0x0022 // QUOTATION MARK
            | 0x0027 // APOSTROPHE
            | 0x0028 // LEFT PARENTHESIS
            | 0x0029 // RIGHT PARENTHESIS
            | 0x005B // LEFT SQUARE BRACKET
            | 0x005D // RIGHT SQUARE BRACKET
            | 0x007B // LEFT CURLY BRACKET
            | 0x007D // RIGHT CURLY BRACKET
            | 0x00AB // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
            | 0x00BB // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
            | 0x0F3A..=0x0F3D
            | 0x169B..=0x169C
            | 0x2018..=0x201F // various quotation marks
            | 0x2039..=0x203A // single angle quotation marks
            | 0x2045..=0x2046
            | 0x207D..=0x207E
            | 0x208D..=0x208E
            | 0x2308..=0x230B
            | 0x2329..=0x232A
            | 0x2768..=0x2775
            | 0x27C5..=0x27C6
            | 0x27E6..=0x27EF
            | 0x2983..=0x2998
            | 0x29D8..=0x29DB
            | 0x29FC..=0x29FD
            | 0x2E22..=0x2E25
            | 0x3008..=0x3011
            | 0x3014..=0x301B
            | 0x301D..=0x301F
            | 0xFD3E..=0xFD3F
            | 0xFE17..=0xFE18
            | 0xFE35..=0xFE44
            | 0xFE47..=0xFE48
            | 0xFE59..=0xFE5E
            | 0xFF08..=0xFF09
            | 0xFF3B // FULLWIDTH LEFT SQUARE BRACKET
            | 0xFF3D // FULLWIDTH RIGHT SQUARE BRACKET
            | 0xFF5B // FULLWIDTH LEFT CURLY BRACKET
            | 0xFF5D // FULLWIDTH RIGHT CURLY BRACKET
            | 0xFF5F..=0xFF60
            | 0xFF62..=0xFF63
    )
}

fn is_sp(u: u32) -> bool {
    // Whitespace: tab and General_Category Zs
    matches!(
        u,
        0x0009 | 0x0020 | 0x00A0 | 0x1680 | 0x2000..=0x200A | 0x202F | 0x205F | 0x3000
    )
}

fn is_sb_numeric(u: u32) -> bool {
    // Decimal digits (Nd) - same as word_break
    (0x0030..=0x0039).contains(&u)
        || (0x0660..=0x0669).contains(&u)
        || (0x06F0..=0x06F9).contains(&u)
        || (0x0966..=0x096F).contains(&u)
        || (0x09E6..=0x09EF).contains(&u)
        || (0x0A66..=0x0A6F).contains(&u)
        || (0x0AE6..=0x0AEF).contains(&u)
        || (0x0B66..=0x0B6F).contains(&u)
        || (0x0BE6..=0x0BEF).contains(&u)
        || (0x0C66..=0x0C6F).contains(&u)
        || (0x0CE6..=0x0CEF).contains(&u)
        || (0x0D66..=0x0D6F).contains(&u)
        || (0x0DE6..=0x0DEF).contains(&u)
        || (0x0E50..=0x0E59).contains(&u)
        || (0x0ED0..=0x0ED9).contains(&u)
        || (0x0F20..=0x0F29).contains(&u)
        || (0x1040..=0x1049).contains(&u)
        || (0x1090..=0x1099).contains(&u)
        || (0x17E0..=0x17E9).contains(&u)
        || (0x1810..=0x1819).contains(&u)
        || (0x1946..=0x194F).contains(&u)
        || (0x19D0..=0x19D9).contains(&u)
        || (0x1A80..=0x1A99).contains(&u)
        || (0x1B50..=0x1B59).contains(&u)
        || (0x1BB0..=0x1BB9).contains(&u)
        || (0x1C40..=0x1C49).contains(&u)
        || (0x1C50..=0x1C59).contains(&u)
        || (0xA620..=0xA629).contains(&u)
        || (0xA8D0..=0xA8D9).contains(&u)
        || (0xA900..=0xA909).contains(&u)
        || (0xA9D0..=0xA9D9).contains(&u)
        || (0xAA50..=0xAA59).contains(&u)
        || (0xABF0..=0xABF9).contains(&u)
        || (0xFF10..=0xFF19).contains(&u)
}

fn is_lower(u: u32) -> bool {
    // Lowercase letters (Ll) - representative set
    (0x0061..=0x007A).contains(&u)  // ASCII lowercase
        || (0x00DF..=0x00F6).contains(&u)
        || (0x00F8..=0x00FF).contains(&u)
        || (0x0101..=0x0101).contains(&u) // with even/odd alternation, cover common
        || (0x00B5..=0x00B5).contains(&u) // MICRO SIGN
        || (0x00E0..=0x00F6).contains(&u) // Latin accented lowercase
        || (0x0100..=0x024F).contains(&u) && is_ll_in_latin_extended(u)
        || (0x0250..=0x02AF).contains(&u) // IPA extensions (all Ll)
        || (0x0371..=0x0373).contains(&u) && (u % 2 == 1) // Greek odd = lowercase
        || (0x03AC..=0x03CE).contains(&u) // Greek lowercase
        || (0x03D0..=0x03D1).contains(&u)
        || (0x03D5..=0x03D7).contains(&u)
        || (0x03D9..=0x03EF).contains(&u) && (u % 2 == 1)
        || (0x03F0..=0x03F3).contains(&u)
        || (0x03F5..=0x03F5).contains(&u)
        || (0x03F8..=0x03F8).contains(&u)
        || (0x03FB..=0x03FC).contains(&u)
        || (0x0430..=0x044F).contains(&u) // Cyrillic lowercase
        || (0x0450..=0x045F).contains(&u)
        || (0x0461..=0x04BF).contains(&u) && (u % 2 == 1)
        || (0x0561..=0x0587).contains(&u) // Armenian lowercase
        || (0x1D00..=0x1DBF).contains(&u) // Phonetic extensions (mostly Ll)
        || (0x1E01..=0x1EFF).contains(&u) && (u % 2 == 1) // Latin extended additional
        || (0x1F00..=0x1F07).contains(&u)
        || (0x1F10..=0x1F15).contains(&u)
        || (0x1F20..=0x1F27).contains(&u)
        || (0x1F30..=0x1F37).contains(&u)
        || (0x1F40..=0x1F45).contains(&u)
        || (0x1F50..=0x1F57).contains(&u)
        || (0x1F60..=0x1F67).contains(&u)
        || (0x1F70..=0x1F7D).contains(&u)
        || (0x1F80..=0x1F87).contains(&u)
        || (0x1F90..=0x1F97).contains(&u)
        || (0x1FA0..=0x1FA7).contains(&u)
        || (0x1FB0..=0x1FB4).contains(&u)
        || (0x1FB6..=0x1FB7).contains(&u)
        || (0x1FC2..=0x1FC4).contains(&u)
        || (0x1FC6..=0x1FC7).contains(&u)
        || (0x1FD0..=0x1FD3).contains(&u)
        || (0x1FD6..=0x1FD7).contains(&u)
        || (0x1FE0..=0x1FE7).contains(&u)
        || (0x1FF2..=0x1FF4).contains(&u)
        || (0x1FF6..=0x1FF7).contains(&u)
        || (0x2071..=0x2071).contains(&u)
        || (0x207F..=0x207F).contains(&u)
        || (0x210A..=0x210A).contains(&u)
        || (0x210E..=0x210F).contains(&u)
        || (0x2113..=0x2113).contains(&u)
        || (0x212F..=0x212F).contains(&u)
        || (0x2134..=0x2134).contains(&u)
        || (0x2139..=0x2139).contains(&u)
        || (0x213C..=0x213D).contains(&u)
        || (0x2146..=0x2149).contains(&u)
        || (0x214E..=0x214E).contains(&u)
        || (0x2184..=0x2184).contains(&u)
        || (0xA641..=0xA66D).contains(&u) && (u % 2 == 1)
        || (0xA723..=0xA76F).contains(&u) && (u % 2 == 1)
        || (0xA77A..=0xA77C).contains(&u)
        || (0xA77F..=0xA787).contains(&u) && (u % 2 == 1)
        || (0xA793..=0xA795).contains(&u)
        || (0xFF41..=0xFF5A).contains(&u) // Fullwidth lowercase
}

/// Helper: is this code point lowercase in Latin Extended-A/B?
fn is_ll_in_latin_extended(u: u32) -> bool {
    // In Latin Extended A/B, odd code points tend to be lowercase.
    // This is a simplification; covers most common cases.
    // Exclude known Lo (Other Letter) characters at odd positions:
    if matches!(u, 0x01BB | 0x01C0..=0x01C3) {
        return false;
    }
    u % 2 == 1 || matches!(u, 0x0137 | 0x0138 | 0x0148 | 0x0149 | 0x017A | 0x017C | 0x017E
        | 0x0180 | 0x0183 | 0x0185 | 0x0188 | 0x018C..=0x018D | 0x0192 | 0x0195
        | 0x0199..=0x019B | 0x019E | 0x01A1 | 0x01A3 | 0x01A5 | 0x01A8 | 0x01AA..=0x01AB
        | 0x01AD | 0x01B0 | 0x01B4 | 0x01B6 | 0x01B9..=0x01BA | 0x01BD..=0x01BF
        | 0x01C6 | 0x01C9 | 0x01CC | 0x01CE | 0x01D0 | 0x01D2 | 0x01D4 | 0x01D6
        | 0x01D8 | 0x01DA | 0x01DC..=0x01DD | 0x01DF | 0x01E1 | 0x01E3 | 0x01E5
        | 0x01E7 | 0x01E9 | 0x01EB | 0x01ED | 0x01EF..=0x01F0 | 0x01F3 | 0x01F5
        | 0x01F9 | 0x01FB | 0x01FD | 0x01FF | 0x0201 | 0x0203 | 0x0205 | 0x0207
        | 0x0209 | 0x020B | 0x020D | 0x020F | 0x0211 | 0x0213 | 0x0215 | 0x0217
        | 0x0219 | 0x021B | 0x021D | 0x021F | 0x0221 | 0x0223 | 0x0225 | 0x0227
        | 0x0229 | 0x022B | 0x022D | 0x022F | 0x0231 | 0x0233..=0x0239 | 0x023C
        | 0x023F..=0x0240 | 0x0242 | 0x0247 | 0x0249 | 0x024B | 0x024D | 0x024F)
}

fn is_upper(u: u32) -> bool {
    // Uppercase letters (Lu) - representative set
    (0x0041..=0x005A).contains(&u)  // ASCII uppercase
        || (0x00C0..=0x00D6).contains(&u)
        || (0x00D8..=0x00DE).contains(&u)
        || (0x0100..=0x024E).contains(&u) && !is_ll_in_latin_extended(u) && (u % 2 == 0)
            && !(0x0250..=0x02AF).contains(&u)
        || (0x0370..=0x0372).contains(&u) && (u % 2 == 0)
        || u == 0x0376
        || (0x0386..=0x0386).contains(&u)
        || (0x0388..=0x038A).contains(&u)
        || u == 0x038C
        || (0x038E..=0x03A1).contains(&u)
        || (0x03A3..=0x03AB).contains(&u)
        || (0x03D2..=0x03D4).contains(&u)
        || u == 0x03D8 // and even in 03D8..03EF
        || (0x03D8..=0x03EE).contains(&u) && (u % 2 == 0)
        || u == 0x03F4
        || u == 0x03F7
        || (0x03F9..=0x03FA).contains(&u)
        || (0x0400..=0x042F).contains(&u) // Cyrillic uppercase
        || (0x0460..=0x04BE).contains(&u) && (u % 2 == 0)
        || (0x0531..=0x0556).contains(&u) // Armenian uppercase
        || (0x10A0..=0x10C5).contains(&u) // Georgian uppercase
        || (0x1E00..=0x1EFE).contains(&u) && (u % 2 == 0)
        || (0x1F08..=0x1F0F).contains(&u)
        || (0x1F18..=0x1F1D).contains(&u)
        || (0x1F28..=0x1F2F).contains(&u)
        || (0x1F38..=0x1F3F).contains(&u)
        || (0x1F48..=0x1F4D).contains(&u)
        || u == 0x1F59 || u == 0x1F5B || u == 0x1F5D || u == 0x1F5F
        || (0x1F68..=0x1F6F).contains(&u)
        || (0x1FB8..=0x1FBB).contains(&u)
        || (0x1FC8..=0x1FCB).contains(&u)
        || (0x1FD8..=0x1FDB).contains(&u)
        || (0x1FE8..=0x1FEC).contains(&u)
        || (0x1FF8..=0x1FFB).contains(&u)
        || u == 0x2102 || u == 0x2107
        || (0x210B..=0x210D).contains(&u)
        || (0x2110..=0x2112).contains(&u)
        || u == 0x2115
        || (0x2119..=0x211D).contains(&u)
        || u == 0x2124 || u == 0x2126 || u == 0x2128
        || (0x212A..=0x212D).contains(&u)
        || (0x2130..=0x2133).contains(&u)
        || (0x213E..=0x213F).contains(&u)
        || u == 0x2145
        || (0x2183..=0x2183).contains(&u)
        || (0xFF21..=0xFF3A).contains(&u) // Fullwidth uppercase
}

fn is_oletter(u: u32) -> bool {
    // OLetter: Lt (titlecase), Lm (modifier), Lo (other letter), and some Nl
    // This is a broad category covering CJK, Arabic, Thai, etc.
    // We check after Lower and Upper, so this catches remaining letters.
    let is_lt = matches!(u, 0x01C5 | 0x01C8 | 0x01CB | 0x01F2 | 0x1FBC | 0x1FCC | 0x1FFC);
    if is_lt {
        return true;
    }
    // U+01BB specifically (test uses it as OLetter representative)
    if u == 0x01BB {
        return true;
    }
    // Lo: broad letter ranges (after Hebrew, Arabic are handled as Lower/Upper/OLetter)
    // CJK unified ideographs
    if (0x4E00..=0x9FFF).contains(&u) || (0x3400..=0x4DBF).contains(&u) {
        return true;
    }
    // Hangul syllables
    if (0xAC00..=0xD7AF).contains(&u) {
        return true;
    }
    // Arabic letters (Lo)
    if (0x0620..=0x064A).contains(&u) || (0x066E..=0x066F).contains(&u)
        || (0x0671..=0x06D3).contains(&u) || u == 0x06D5
        || (0x06E5..=0x06E6).contains(&u) || (0x06EE..=0x06EF).contains(&u)
        || (0x06FA..=0x06FC).contains(&u) || u == 0x06FF
    {
        return true;
    }
    // Hebrew letters
    if (0x05D0..=0x05EA).contains(&u) || (0x05EF..=0x05F2).contains(&u) {
        return true;
    }
    // Thai
    if (0x0E01..=0x0E30).contains(&u) || (0x0E32..=0x0E33).contains(&u)
        || (0x0E40..=0x0E46).contains(&u)
    {
        return true;
    }
    // Devanagari and other Indic
    if (0x0904..=0x0939).contains(&u) || u == 0x093D || u == 0x0950
        || (0x0958..=0x0961).contains(&u) || (0x0972..=0x097F).contains(&u)
    {
        return true;
    }
    // Katakana, Hiragana (already handled in word_break as Katakana, but here they're OLetter)
    if (0x3041..=0x3096).contains(&u) || (0x309D..=0x309F).contains(&u)
        || (0x30A1..=0x30FA).contains(&u) || (0x30FC..=0x30FF).contains(&u)
        || (0x3105..=0x312F).contains(&u) || (0x31A0..=0x31BA).contains(&u)
    {
        return true;
    }
    // Lm: modifier letters (common ones)
    if matches!(u, 0x02B0..=0x02C1 | 0x02C6..=0x02D1 | 0x02E0..=0x02E4 | 0x02EC | 0x02EE
        | 0x0374 | 0x037A | 0x0559 | 0x06E5..=0x06E6 | 0x07F4..=0x07F5 | 0x07FA
        | 0x1C78..=0x1C7D | 0x1D2C..=0x1D6A | 0x1D78 | 0x1D9B..=0x1DBF
        | 0x2071 | 0x207F | 0x2090..=0x209C | 0x2C7C..=0x2C7D
        | 0xA69C..=0xA69D | 0xA717..=0xA71F | 0xA770 | 0xA788
        | 0xAB5C..=0xAB5F
        | 0xFF70 | 0xFF9E..=0xFF9F)
    {
        return true;
    }
    // Syriac letters
    if (0x0710..=0x072F).contains(&u) {
        return true;
    }
    // Remaining broad Lo ranges
    if (0xA000..=0xA48C).contains(&u)  // Yi
        || (0xA4D0..=0xA4FD).contains(&u)  // Lisu
        || (0xA500..=0xA60C).contains(&u)  // Vai
        || (0xA610..=0xA61F).contains(&u)
        || (0xA62A..=0xA62B).contains(&u)
        || (0xA840..=0xA873).contains(&u)  // Phags-pa
        || (0xA882..=0xA8B3).contains(&u)  // Saurashtra
        || (0xFB50..=0xFDCF).contains(&u)  // Arabic Presentation Forms
        || (0xFDF0..=0xFDFB).contains(&u)
        || (0xFE70..=0xFEFC).contains(&u)
    {
        return true;
    }
    false
}
