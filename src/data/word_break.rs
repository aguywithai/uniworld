//! Word_Break property (UAX #29 Section 4).
//!
//! Minimal inline classification for default word boundaries.
//! Full table from WordBreakProperty.txt can be used for full conformance.

/// Word_Break property value (UAX #29 Table 2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wb {
    /// Carriage return U+000D
    Cr,
    /// Line feed U+000A
    Lf,
    /// Newline (VT, FF, NEL, LS, PS)
    Newline,
    /// Do not break before (combining marks, ZWNJ)
    Extend,
    /// Do not break before (format chars except CR/LF/ZWNJ/ZWJ)
    Format,
    /// Zero-width joiner U+200D
    Zwj,
    /// Alphabetic letter (used in words)
    ALetter,
    /// Hebrew letter
    HebrewLetter,
    /// Numeric
    Numeric,
    /// Katakana (no break between Katakana)
    Katakana,
    /// Single quote U+0027 (APOSTROPHE)
    SingleQuote,
    /// Double quote U+0022
    DoubleQuote,
    /// MidLetter (e.g. U+00B7 in "we're")
    MidLetter,
    /// MidNumLet (e.g. U+003A in "a:b")
    MidNumLet,
    /// MidNum (e.g. comma in numbers)
    MidNum,
    /// ExtendNumLet (e.g. U+005F LOW LINE, connects identifier parts)
    ExtendNumLet,
    /// Word-segmenting space (space, tab, etc.)
    WSegSpace,
    /// Regional_Indicator (U+1F1E6..U+1F1FF, flag emoji)
    RegionalIndicator,
    /// Any other
    Other,
}

/// Returns Word_Break for the given code point (UAX #29).
#[must_use]
pub fn wb(c: char) -> Wb {
    let u = c as u32;
    if u == 0x000D {
        return Wb::Cr;
    }
    if u == 0x000A {
        return Wb::Lf;
    }
    if matches!(u, 0x000B | 0x000C | 0x0085 | 0x2028 | 0x2029) {
        return Wb::Newline;
    }
    if u == 0x200D {
        return Wb::Zwj;
    }
    if u == 0x200C {
        return Wb::Extend;
    }
    if u == 0x0022 {
        return Wb::DoubleQuote;
    }
    if u == 0x0027 {
        return Wb::SingleQuote;
    }
    // ExtendNumLet (e.g. U+005F LOW LINE)
    if u == 0x005F {
        return Wb::ExtendNumLet;
    }
    // Unicode 17.0 reclassifications: these were Format in older versions
    // U+06DD ARABIC END OF AYAH: now Word_Break=Numeric
    if u == 0x06DD {
        return Wb::Numeric;
    }
    // U+070F SYRIAC ABBREVIATION MARK: now Word_Break=ALetter
    if u == 0x070F {
        return Wb::ALetter;
    }
    // Format: Cf except ZWNJ, ZWJ, and reclassified code points
    if is_format(u) {
        return Wb::Format;
    }
    if is_extend(u) {
        return Wb::Extend;
    }
    // WSegSpace: space, tab, no-break space, etc.
    if matches!(u, 0x0020 | 0x0009 | 0x00A0 | 0x1680 | 0x2000..=0x200B | 0x202F | 0x205F | 0x3000) {
        return Wb::WSegSpace;
    }
    // Hebrew_Letter
    if (0x05D0..=0x05EA).contains(&u)
        || (0x05EF..=0x05F2).contains(&u)
        || (0xFB1D..=0xFB4F).contains(&u)
    {
        return Wb::HebrewLetter;
    }
    // Katakana (Hiragana and Katakana letter ranges; 3031 = VERTICAL KANA REPEAT MARK)
    // Note: 3099-309A are combining marks caught by is_extend() above.
    if (0x3031..=0x3035).contains(&u)
        || (0x30A1..=0x30FA).contains(&u)
        || (0x30FC..=0x30FE).contains(&u)
        || (0x3041..=0x3096).contains(&u)
    {
        return Wb::Katakana;
    }
    // MidLetter (UAX #29): used in AHLetter x MidLetter AHLetter (WB6/WB7)
    if matches!(
        u,
        0x003A | 0x00B7 | 0x0387 | 0x055F | 0x05F4 | 0x2027 | 0xFE13 | 0xFE55 | 0xFF1A
    ) {
        return Wb::MidLetter;
    }
    // MidNumLet (UAX #29): participates in BOTH letter and numeric mid-word rules
    if matches!(u, 0x002E | 0x2018 | 0x2019 | 0x2024 | 0xFE52 | 0xFF0E | 0xFF07) {
        return Wb::MidNumLet;
    }
    // MidNum (UAX #29): used only in Numeric x MidNum Numeric (WB11/WB12)
    if matches!(
        u,
        0x002C | 0x037E | 0x0589 | 0x060C | 0x060D | 0x07F8 | 0x2044 | 0xFE10 | 0xFE50
            | 0xFF0C
    ) {
        return Wb::MidNum;
    }
    // Numeric: Nd, Nl, No
    if is_numeric(u) {
        return Wb::Numeric;
    }
    // Regional_Indicator (U+1F1E6..U+1F1FF)
    if (0x1F1E6..=0x1F1FF).contains(&u) {
        return Wb::RegionalIndicator;
    }
    // ALetter: Lu, Ll, Lt, Lm, Lo (letters)
    if is_letter(u) {
        return Wb::ALetter;
    }
    Wb::Other
}

fn is_format(u: u32) -> bool {
    // Cf except 200C, 200D, and reclassified code points (06DD, 070F)
    if u == 0x200C || u == 0x200D {
        return false;
    }
    u == 0x00AD
        || (0x0600..=0x0605).contains(&u)
        || (0x200E..=0x200F).contains(&u)
        || (0x202A..=0x202E).contains(&u)
        || (0x2060..=0x2064).contains(&u)
        || (0x2066..=0x206F).contains(&u)
        || (0xFEFF..=0xFEFF).contains(&u)
}

fn is_extend(u: u32) -> bool {
    // Mn, Me, Mc (combining) - representative set
    (0x0300..=0x036F).contains(&u)
        || (0x0483..=0x0489).contains(&u)
        || (0x0591..=0x05BD).contains(&u)
        || (0x0610..=0x061A).contains(&u)
        || (0x064B..=0x065F).contains(&u)
        || (0x0670..=0x0670).contains(&u)
        || (0x06D6..=0x06DC).contains(&u)
        || (0x06DF..=0x06E4).contains(&u)
        || (0x06E7..=0x06E8).contains(&u)
        || (0x06EA..=0x06ED).contains(&u)
        || (0x0730..=0x074A).contains(&u)
        || (0x07A6..=0x07B0).contains(&u)
        || (0x0900..=0x0902).contains(&u)
        || (0x093A..=0x094D).contains(&u)
        || (0x0951..=0x0957).contains(&u)
        || (0x0962..=0x0963).contains(&u)
        || (0x0981..=0x0983).contains(&u)
        || (0x09BC..=0x09C4).contains(&u)
        || (0x09C7..=0x09C8).contains(&u)
        || (0x09CB..=0x09CD).contains(&u)
        || (0x09D7..=0x09D7).contains(&u)
        || (0x09E2..=0x09E3).contains(&u)
        || (0x0A01..=0x0A03).contains(&u)
        || (0x0A3C..=0x0A3C).contains(&u)
        || (0x0A3E..=0x0A42).contains(&u)
        || (0x0A47..=0x0A48).contains(&u)
        || (0x0A4B..=0x0A4D).contains(&u)
        || (0x0A51..=0x0A51).contains(&u)
        || (0x0A70..=0x0A71).contains(&u)
        || (0x0A75..=0x0A75).contains(&u)
        || (0x0A81..=0x0A83).contains(&u)
        || (0x0ABC..=0x0ACD).contains(&u)
        || (0x0AE2..=0x0AE3).contains(&u)
        || (0x0B01..=0x0B03).contains(&u)
        || (0x0B3C..=0x0B44).contains(&u)
        || (0x0B47..=0x0B48).contains(&u)
        || (0x0B4B..=0x0B4D).contains(&u)
        || (0x0B56..=0x0B57).contains(&u)
        || (0x0B62..=0x0B63).contains(&u)
        || (0x0B82..=0x0B82).contains(&u)
        || (0x0BBE..=0x0BCC).contains(&u)
        || (0x0BD7..=0x0BD7).contains(&u)
        || (0x0C00..=0x0C44).contains(&u)
        || (0x0C46..=0x0C4D).contains(&u)
        || (0x0C55..=0x0C56).contains(&u)
        || (0x0C62..=0x0C63).contains(&u)
        || (0x0C81..=0x0C83).contains(&u)
        || (0x0CBC..=0x0CCD).contains(&u)
        || (0x0CD5..=0x0CD6).contains(&u)
        || (0x0CE2..=0x0CE3).contains(&u)
        || (0x0D01..=0x0D03).contains(&u)
        || (0x0D3B..=0x0D3C).contains(&u)
        || (0x0D3E..=0x0D44).contains(&u)
        || (0x0D46..=0x0D48).contains(&u)
        || (0x0D4A..=0x0D4D).contains(&u)
        || (0x0D57..=0x0D57).contains(&u)
        || (0x0D62..=0x0D63).contains(&u)
        || (0x0D82..=0x0D83).contains(&u)
        || (0x0DCA..=0x0DDF).contains(&u)
        || (0x0DF2..=0x0DF3).contains(&u)
        || (0x0E31..=0x0E31).contains(&u)
        || (0x0E34..=0x0E3A).contains(&u)
        || (0x0E47..=0x0E4E).contains(&u)
        || (0x0EB1..=0x0EB1).contains(&u)
        || (0x0EB4..=0x0EBC).contains(&u)
        || (0x0EC8..=0x0ECE).contains(&u)
        || (0x0F18..=0x0F19).contains(&u)
        || (0x0F35..=0x0F35).contains(&u)
        || (0x0F37..=0x0F37).contains(&u)
        || (0x0F39..=0x0F39).contains(&u)
        || (0x0F3E..=0x0F3F).contains(&u)
        || (0x0F71..=0x0F84).contains(&u)
        || (0x0F86..=0x0F87).contains(&u)
        || (0x0F8D..=0x0FBC).contains(&u)
        || (0x0FC6..=0x0FC6).contains(&u)
        || (0x102B..=0x103E).contains(&u)
        || (0x1056..=0x1059).contains(&u)
        || (0x105E..=0x1060).contains(&u)
        || (0x1062..=0x1064).contains(&u)
        || (0x1067..=0x106D).contains(&u)
        || (0x1071..=0x1074).contains(&u)
        || (0x1082..=0x108D).contains(&u)
        || (0x108F..=0x108F).contains(&u)
        || (0x109A..=0x109D).contains(&u)
        || (0x135D..=0x135F).contains(&u)
        || (0x1712..=0x1714).contains(&u)
        || (0x1732..=0x1734).contains(&u)
        || (0x1752..=0x1753).contains(&u)
        || (0x1772..=0x1773).contains(&u)
        || (0x17B4..=0x17D3).contains(&u)
        || (0x17DD..=0x17DD).contains(&u)
        || (0x180B..=0x180D).contains(&u)
        || (0x18A9..=0x18A9).contains(&u)
        || (0x1920..=0x193B).contains(&u)
        || (0x1A17..=0x1A1B).contains(&u)
        || (0x1A55..=0x1A72).contains(&u)
        || (0x1A7F..=0x1A7F).contains(&u)
        || (0x1AB0..=0x1ABE).contains(&u)
        || (0x1ABF..=0x1AC0).contains(&u)
        || (0x1B00..=0x1B04).contains(&u)
        || (0x1B34..=0x1B44).contains(&u)
        || (0x1B6B..=0x1B73).contains(&u)
        || (0x1B80..=0x1B82).contains(&u)
        || (0x1BA1..=0x1BAD).contains(&u)
        || (0x1BE6..=0x1BF3).contains(&u)
        || (0x1C24..=0x1C37).contains(&u)
        || (0x1CD0..=0x1CE8).contains(&u)
        || (0x1CED..=0x1CED).contains(&u)
        || (0x1CF4..=0x1CF9).contains(&u)
        || (0x1DC0..=0x1DFF).contains(&u)
        || (0x20D0..=0x20F0).contains(&u)
        || (0x2CEF..=0x2CF1).contains(&u)
        || (0x2D7F..=0x2D7F).contains(&u)
        || (0x2DE0..=0x2DFF).contains(&u)
        || (0x302A..=0x302F).contains(&u)
        || (0x3099..=0x309A).contains(&u)
        || (0xA66F..=0xA672).contains(&u)
        || (0xA674..=0xA67D).contains(&u)
        || (0xA69E..=0xA69F).contains(&u)
        || (0xA6F0..=0xA6F1).contains(&u)
        || (0xA802..=0xA827).contains(&u)
        || (0xA82C..=0xA82C).contains(&u)
        || (0xA880..=0xA8C4).contains(&u)
        || (0xA8E0..=0xA8F1).contains(&u)
        || (0xA926..=0xA92D).contains(&u)
        || (0xA947..=0xA953).contains(&u)
        || (0xA980..=0xA983).contains(&u)
        || (0xA9B3..=0xA9C0).contains(&u)
        || (0xA9E5..=0xA9E5).contains(&u)
        || (0xAA29..=0xAA36).contains(&u)
        || (0xAA43..=0xAA43).contains(&u)
        || (0xAA4C..=0xAA4D).contains(&u)
        || (0xAA7B..=0xAA7D).contains(&u)
        || (0xAAB0..=0xAABF).contains(&u)
        || (0xAAC1..=0xAAC1).contains(&u)
        || (0xAAEB..=0xAAEF).contains(&u)
        || (0xAAF5..=0xAAF6).contains(&u)
        || (0xABE3..=0xABEA).contains(&u)
        || (0xABEC..=0xABED).contains(&u)
        || (0xFB1E..=0xFB1E).contains(&u)
        || (0xFE00..=0xFE0F).contains(&u)
        || (0xFE20..=0xFE2F).contains(&u)
        || (0xFF9E..=0xFF9F).contains(&u)
        // Emoji skin tone modifiers (GC=Sk but Word_Break=Extend)
        || (0x1F3FB..=0x1F3FF).contains(&u)
        // Variation selectors supplement
        || (0xE0100..=0xE01EF).contains(&u)
}

fn is_numeric(u: u32) -> bool {
    // Nd, Nl, No
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
        || (0x0F20..=0x0F33).contains(&u)
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

fn is_letter(u: u32) -> bool {
    // Lu, Ll, Lt, Lm, Lo (Unicode general category)
    matches!(u,
        0x0041..=0x005A | 0x0061..=0x007A  // ASCII
        | 0x00C0..=0x00D6 | 0x00D8..=0x00F6 | 0x00F8..=0x024F
        | 0x0250..=0x02AF | 0x0370..=0x0373 | 0x0376..=0x0377
        | 0x037A..=0x037D | 0x0386..=0x0386 | 0x0388..=0x038A
        | 0x038C..=0x038C | 0x038E..=0x03A1 | 0x03A3..=0x03CF
        | 0x03D0..=0x03FF | 0x0400..=0x0484 | 0x0487..=0x052F
        | 0x0531..=0x0556 | 0x0561..=0x0587 | 0x0589..=0x058A
        | 0x0591..=0x05BD | 0x05BF..=0x05BF | 0x05C1..=0x05C2
        | 0x05C4..=0x05C5 | 0x05C7..=0x05C7
    ) || (0x05D0..=0x05EA).contains(&u)
    || (0x05F0..=0x05F2).contains(&u)
    || (0x0610..=0x061A).contains(&u)
    || (0x0621..=0x063F).contains(&u)
    || (0x0641..=0x064A).contains(&u)
    || (0x064B..=0x0657).contains(&u)
    || (0x0659..=0x065F).contains(&u)
    || (0x066E..=0x06D3).contains(&u)
    || (0x06D5..=0x06DC).contains(&u)
    || (0x06E1..=0x06E8).contains(&u)
    || (0x06EA..=0x06ED).contains(&u)
    || (0x06F0..=0x06F9).contains(&u)
    || (0x06FA..=0x06FC).contains(&u)
    || (0x06FF..=0x06FF).contains(&u)
    || (0x0710..=0x074A).contains(&u)
    || (0x074D..=0x07B1).contains(&u)
    || (0x07C0..=0x07F5).contains(&u)
    || (0x07FA..=0x07FA).contains(&u)
    || (0x0800..=0x082D).contains(&u)
    || (0x0840..=0x085B).contains(&u)
    || (0x08A0..=0x08B4).contains(&u)
    || (0x08B6..=0x08BD).contains(&u)
    || (0x0900..=0x0977).contains(&u)
    || (0x0979..=0x097F).contains(&u)
    || (0x0981..=0x0983).contains(&u)
    || (0x0985..=0x098C).contains(&u)
    || (0x098F..=0x0990).contains(&u)
    || (0x0993..=0x09A8).contains(&u)
    || (0x09AA..=0x09B0).contains(&u)
    || (0x09B2..=0x09B2).contains(&u)
    || (0x09B6..=0x09B9).contains(&u)
    || (0x09BC..=0x09C4).contains(&u)
    || (0x09C7..=0x09C8).contains(&u)
    || (0x09CB..=0x09CE).contains(&u)
    || (0x09D7..=0x09D7).contains(&u)
    || (0x09DC..=0x09DD).contains(&u)
    || (0x09DF..=0x09E3).contains(&u)
    || (0x09E6..=0x09FB).contains(&u)
    || (0x0A01..=0x0A03).contains(&u)
    || (0x0A05..=0x0A0A).contains(&u)
    || (0x0A0F..=0x0A10).contains(&u)
    || (0x0A13..=0x0A28).contains(&u)
    || (0x0A2A..=0x0A30).contains(&u)
    || (0x0A32..=0x0A33).contains(&u)
    || (0x0A35..=0x0A36).contains(&u)
    || (0x0A38..=0x0A39).contains(&u)
    || (0x0A3C..=0x0A3C).contains(&u)
    || (0x0A3E..=0x0A42).contains(&u)
    || (0x0A47..=0x0A48).contains(&u)
    || (0x0A4B..=0x0A4D).contains(&u)
    || (0x0A51..=0x0A51).contains(&u)
    || (0x0A59..=0x0A5C).contains(&u)
    || (0x0A5E..=0x0A5E).contains(&u)
    || (0x0A66..=0x0A75).contains(&u)
    || (0x0A81..=0x0A83).contains(&u)
    || (0x0A85..=0x0A8D).contains(&u)
    || (0x0A8F..=0x0A91).contains(&u)
    || (0x0A93..=0x0AA8).contains(&u)
    || (0x0AAA..=0x0AB0).contains(&u)
    || (0x0AB2..=0x0AB3).contains(&u)
    || (0x0AB5..=0x0AB9).contains(&u)
    || (0x0ABC..=0x0AC5).contains(&u)
    || (0x0AC7..=0x0AC9).contains(&u)
    || (0x0ACB..=0x0ACD).contains(&u)
    || (0x0AD0..=0x0AD0).contains(&u)
    || (0x0AE0..=0x0AE3).contains(&u)
    || (0x0AE6..=0x0AEF).contains(&u)
    || (0x0AF9..=0x0AFC).contains(&u)
    || (0x0B01..=0x0B03).contains(&u)
    || (0x0B05..=0x0B0C).contains(&u)
    || (0x0B0F..=0x0B10).contains(&u)
    || (0x0B13..=0x0B28).contains(&u)
    || (0x0B2A..=0x0B30).contains(&u)
    || (0x0B32..=0x0B33).contains(&u)
    || (0x0B35..=0x0B39).contains(&u)
    || (0x0B3C..=0x0B44).contains(&u)
    || (0x0B47..=0x0B48).contains(&u)
    || (0x0B4B..=0x0B4D).contains(&u)
    || (0x0B56..=0x0B57).contains(&u)
    || (0x0B5C..=0x0B5D).contains(&u)
    || (0x0B5F..=0x0B63).contains(&u)
    || (0x0B66..=0x0B77).contains(&u)
    || (0x0B82..=0x0B83).contains(&u)
    || (0x0B85..=0x0B8A).contains(&u)
    || (0x0B8E..=0x0B90).contains(&u)
    || (0x0B92..=0x0B95).contains(&u)
    || (0x0B99..=0x0B9A).contains(&u)
    || (0x0B9C..=0x0B9C).contains(&u)
    || (0x0B9E..=0x0B9F).contains(&u)
    || (0x0BA3..=0x0BA4).contains(&u)
    || (0x0BA8..=0x0BAA).contains(&u)
    || (0x0BAE..=0x0BB9).contains(&u)
    || (0x0BBE..=0x0BC2).contains(&u)
    || (0x0BC6..=0x0BC8).contains(&u)
    || (0x0BCA..=0x0BCD).contains(&u)
    || (0x0BD0..=0x0BD0).contains(&u)
    || (0x0BD7..=0x0BD7).contains(&u)
    || (0x0BE6..=0x0BFA).contains(&u)
    || (0x0C01..=0x0C03).contains(&u)
    || (0x0C05..=0x0C0C).contains(&u)
    || (0x0C0E..=0x0C10).contains(&u)
    || (0x0C12..=0x0C28).contains(&u)
    || (0x0C2A..=0x0C39).contains(&u)
    || (0x0C3D..=0x0C44).contains(&u)
    || (0x0C46..=0x0C48).contains(&u)
    || (0x0C4A..=0x0C4D).contains(&u)
    || (0x0C55..=0x0C56).contains(&u)
    || (0x0C58..=0x0C5A).contains(&u)
    || (0x0C60..=0x0C63).contains(&u)
    || (0x0C66..=0x0C6F).contains(&u)
    || (0x0C78..=0x0C7F).contains(&u)
    || (0x0C82..=0x0C83).contains(&u)
    || (0x0C85..=0x0C8C).contains(&u)
    || (0x0C8E..=0x0C90).contains(&u)
    || (0x0C92..=0x0CA8).contains(&u)
    || (0x0CAA..=0x0CB3).contains(&u)
    || (0x0CB5..=0x0CB9).contains(&u)
    || (0x0CBC..=0x0CC4).contains(&u)
    || (0x0CC6..=0x0CC8).contains(&u)
    || (0x0CCA..=0x0CCD).contains(&u)
    || (0x0CD5..=0x0CD6).contains(&u)
    || (0x0CDE..=0x0CDE).contains(&u)
    || (0x0CE0..=0x0CE3).contains(&u)
    || (0x0CE6..=0x0CEF).contains(&u)
    || (0x0CF1..=0x0CF2).contains(&u)
    || (0x0D02..=0x0D03).contains(&u)
    || (0x0D05..=0x0D0C).contains(&u)
    || (0x0D0E..=0x0D10).contains(&u)
    || (0x0D12..=0x0D3A).contains(&u)
    || (0x0D3D..=0x0D44).contains(&u)
    || (0x0D46..=0x0D48).contains(&u)
    || (0x0D4A..=0x0D4E).contains(&u)
    || (0x0D57..=0x0D57).contains(&u)
    || (0x0D60..=0x0D63).contains(&u)
    || (0x0D66..=0x0D75).contains(&u)
    || (0x0D79..=0x0D7F).contains(&u)
    || (0x0D82..=0x0D83).contains(&u)
    || (0x0D85..=0x0D96).contains(&u)
    || (0x0D9A..=0x0DB1).contains(&u)
    || (0x0DB3..=0x0DBB).contains(&u)
    || (0x0DBD..=0x0DBD).contains(&u)
    || (0x0DC0..=0x0DC6).contains(&u)
    || (0x0DCA..=0x0DCA).contains(&u)
    || (0x0DCF..=0x0DD4).contains(&u)
    || (0x0DD6..=0x0DD6).contains(&u)
    || (0x0DD8..=0x0DDF).contains(&u)
    || (0x0DF2..=0x0DF4).contains(&u)
    || (0x0E01..=0x0E3A).contains(&u)
    || (0x0E40..=0x0E4E).contains(&u)
    || (0x0E50..=0x0E59).contains(&u)
    || (0x0E81..=0x0E82).contains(&u)
    || (0x0E84..=0x0E84).contains(&u)
    || (0x0E87..=0x0E88).contains(&u)
    || (0x0E8A..=0x0E8A).contains(&u)
    || (0x0E8D..=0x0E8D).contains(&u)
    || (0x0E94..=0x0E97).contains(&u)
    || (0x0E99..=0x0E9F).contains(&u)
    || (0x0EA1..=0x0EA3).contains(&u)
    || (0x0EA5..=0x0EA5).contains(&u)
    || (0x0EA7..=0x0EA7).contains(&u)
    || (0x0EAA..=0x0EAB).contains(&u)
    || (0x0EAD..=0x0EB9).contains(&u)
    || (0x0EBB..=0x0EBD).contains(&u)
    || (0x0EC0..=0x0EC4).contains(&u)
    || (0x0EC6..=0x0EC6).contains(&u)
    || (0x0EC8..=0x0ECD).contains(&u)
    || (0x0ED0..=0x0ED9).contains(&u)
    || (0x0EDC..=0x0EDF).contains(&u)
    || (0x0F00..=0x0F47).contains(&u)
    || (0x0F49..=0x0F6C).contains(&u)
    || (0x0F71..=0x0F97).contains(&u)
    || (0x0F99..=0x0FBC).contains(&u)
    || (0x0FBE..=0x0FCC).contains(&u)
    || (0x0FCE..=0x0FD4).contains(&u)
    || (0x1000..=0x109F).contains(&u)
    || (0x10A0..=0x10C5).contains(&u)
    || (0x10C7..=0x10C7).contains(&u)
    || (0x10CD..=0x10CD).contains(&u)
    || (0x10D0..=0x10FA).contains(&u)
    || (0x10FC..=0x1248).contains(&u)
    || (0x124A..=0x124D).contains(&u)
    || (0x1250..=0x1256).contains(&u)
    || (0x1258..=0x1258).contains(&u)
    || (0x125A..=0x125D).contains(&u)
    || (0x1260..=0x1288).contains(&u)
    || (0x128A..=0x128D).contains(&u)
    || (0x1290..=0x12B0).contains(&u)
    || (0x12B2..=0x12B5).contains(&u)
    || (0x12B8..=0x12BE).contains(&u)
    || (0x12C0..=0x12C0).contains(&u)
    || (0x12C2..=0x12C5).contains(&u)
    || (0x12C8..=0x12D6).contains(&u)
    || (0x12D8..=0x1310).contains(&u)
    || (0x1312..=0x1315).contains(&u)
    || (0x1318..=0x135A).contains(&u)
    || (0x1369..=0x137C).contains(&u)
    || (0x1380..=0x138F).contains(&u)
    || (0x13A0..=0x13F5).contains(&u)
    || (0x13F8..=0x13FD).contains(&u)
    || (0x1401..=0x166C).contains(&u)
    || (0x166F..=0x167F).contains(&u)
    || (0x1681..=0x169A).contains(&u)
    || (0x16A0..=0x16EA).contains(&u)
    || (0x16EE..=0x16F8).contains(&u)
    || (0x1700..=0x1711).contains(&u)
    || (0x171F..=0x1731).contains(&u)
    || (0x1734..=0x1736).contains(&u)
    || (0x1740..=0x1751).contains(&u)
    || (0x1760..=0x176C).contains(&u)
    || (0x176E..=0x1770).contains(&u)
    || (0x1772..=0x1773).contains(&u)
    || (0x1780..=0x17B3).contains(&u)
    || (0x17B6..=0x17D3).contains(&u)
    || (0x17D7..=0x17DC).contains(&u)
    || (0x17DD..=0x17DD).contains(&u)
    || (0x17E0..=0x17E9).contains(&u)
    || (0x180B..=0x180D).contains(&u)
    || (0x1810..=0x1819).contains(&u)
    || (0x1820..=0x1878).contains(&u)
    || (0x1880..=0x18A8).contains(&u)
    || (0x18AA..=0x18AA).contains(&u)
    || (0x18B0..=0x18F5).contains(&u)
    || (0x1900..=0x191E).contains(&u)
    || (0x1920..=0x192B).contains(&u)
    || (0x1930..=0x1938).contains(&u)
    || (0x1946..=0x196D).contains(&u)
    || (0x1970..=0x1974).contains(&u)
    || (0x1980..=0x19AB).contains(&u)
    || (0x19B0..=0x19C9).contains(&u)
    || (0x19D0..=0x19DA).contains(&u)
    || (0x1A00..=0x1A16).contains(&u)
    || (0x1A20..=0x1A54).contains(&u)
    || (0x1A80..=0x1A89).contains(&u)
    || (0x1A90..=0x1A99).contains(&u)
    || (0x1AA0..=0x1AAD).contains(&u)
    || (0x1B05..=0x1B33).contains(&u)
    || (0x1B45..=0x1B4B).contains(&u)
    || (0x1B83..=0x1BA0).contains(&u)
    || (0x1BAE..=0x1BAF).contains(&u)
    || (0x1BBA..=0x1BE5).contains(&u)
    || (0x1C00..=0x1C23).contains(&u)
    || (0x1C4D..=0x1C4F).contains(&u)
    || (0x1C5A..=0x1C7D).contains(&u)
    || (0x1C80..=0x1C88).contains(&u)
    || (0x1C90..=0x1CBA).contains(&u)
    || (0x1CBD..=0x1CBF).contains(&u)
    || (0x1CE9..=0x1CEC).contains(&u)
    || (0x1CEE..=0x1CF3).contains(&u)
    || (0x1CF5..=0x1CF6).contains(&u)
    || (0x1CFA..=0x1CFA).contains(&u)
    || (0x1D00..=0x1DBF).contains(&u)
    || (0x1E00..=0x1F15).contains(&u)
    || (0x1F18..=0x1F1D).contains(&u)
    || (0x1F20..=0x1F45).contains(&u)
    || (0x1F48..=0x1F4D).contains(&u)
    || (0x1F50..=0x1F57).contains(&u)
    || (0x1F59..=0x1F59).contains(&u)
    || (0x1F5B..=0x1F5B).contains(&u)
    || (0x1F5D..=0x1F5D).contains(&u)
    || (0x1F5F..=0x1F7D).contains(&u)
    || (0x1F80..=0x1FB4).contains(&u)
    || (0x1FB6..=0x1FBC).contains(&u)
    || (0x1FBE..=0x1FBE).contains(&u)
    || (0x1FC2..=0x1FC4).contains(&u)
    || (0x1FC6..=0x1FCC).contains(&u)
    || (0x1FD0..=0x1FD3).contains(&u)
    || (0x1FD6..=0x1FDB).contains(&u)
    || (0x1FE0..=0x1FEC).contains(&u)
    || (0x1FF2..=0x1FF4).contains(&u)
    || (0x1FF6..=0x1FFC).contains(&u)
    || (0x2071..=0x2071).contains(&u)
    || (0x207F..=0x207F).contains(&u)
    || (0x2090..=0x209C).contains(&u)
    || (0x2102..=0x2102).contains(&u)
    || (0x2107..=0x2107).contains(&u)
    || (0x210A..=0x2113).contains(&u)
    || (0x2115..=0x2115).contains(&u)
    || (0x2119..=0x211D).contains(&u)
    || (0x2124..=0x2124).contains(&u)
    || (0x2126..=0x2126).contains(&u)
    || (0x2128..=0x2128).contains(&u)
    || (0x212A..=0x212D).contains(&u)
    || (0x212F..=0x2139).contains(&u)
    || (0x213C..=0x213F).contains(&u)
    || (0x2145..=0x2149).contains(&u)
    || (0x214E..=0x214E).contains(&u)
    || (0x2160..=0x2188).contains(&u)
    || (0x24B6..=0x24E9).contains(&u)
    || (0x2C00..=0x2C2E).contains(&u)
    || (0x2C30..=0x2C5E).contains(&u)
    || (0x2C60..=0x2CE4).contains(&u)
    || (0x2CEB..=0x2CEE).contains(&u)
    || (0x2CF2..=0x2CF3).contains(&u)
    || (0x2D00..=0x2D25).contains(&u)
    || (0x2D27..=0x2D27).contains(&u)
    || (0x2D2D..=0x2D2D).contains(&u)
    || (0x2D30..=0x2D67).contains(&u)
    || (0x2D6F..=0x2D70).contains(&u)
    || (0x2D80..=0x2D96).contains(&u)
    || (0x2DA0..=0x2DA6).contains(&u)
    || (0x2DA8..=0x2DAE).contains(&u)
    || (0x2DB0..=0x2DB6).contains(&u)
    || (0x2DB8..=0x2DBE).contains(&u)
    || (0x2DC0..=0x2DC6).contains(&u)
    || (0x2DC8..=0x2DCE).contains(&u)
    || (0x2DD0..=0x2DD6).contains(&u)
    || (0x2DD8..=0x2DDE).contains(&u)
    || (0x2E2F..=0x2E2F).contains(&u)
    || (0x3005..=0x3007).contains(&u)
    || (0x3021..=0x3029).contains(&u)
    || (0x3031..=0x3035).contains(&u)
    || (0x3038..=0x303C).contains(&u)
    || (0x3041..=0x3096).contains(&u)
    || (0x309D..=0x309F).contains(&u)
    || (0x30A1..=0x30FA).contains(&u)
    || (0x30FC..=0x30FF).contains(&u)
    || (0x3105..=0x312F).contains(&u)
    || (0x3131..=0x318E).contains(&u)
    || (0x31A0..=0x31BA).contains(&u)
    || (0x31F0..=0x31FF).contains(&u)
    || (0x3400..=0x4DB5).contains(&u)
    || (0x4E00..=0x9FEF).contains(&u)
    || (0xA000..=0xA48C).contains(&u)
    || (0xA4D0..=0xA4FD).contains(&u)
    || (0xA500..=0xA60C).contains(&u)
    || (0xA610..=0xA61F).contains(&u)
    || (0xA62A..=0xA62B).contains(&u)
    || (0xA640..=0xA66E).contains(&u)
    || (0xA674..=0xA67B).contains(&u)
    || (0xA67F..=0xA6EF).contains(&u)
    || (0xA717..=0xA71F).contains(&u)
    || (0xA722..=0xA788).contains(&u)
    || (0xA78B..=0xA7BF).contains(&u)
    || (0xA7C2..=0xA7C6).contains(&u)
    || (0xA7F7..=0xA801).contains(&u)
    || (0xA803..=0xA805).contains(&u)
    || (0xA807..=0xA80A).contains(&u)
    || (0xA80C..=0xA822).contains(&u)
    || (0xA840..=0xA873).contains(&u)
    || (0xA882..=0xA8B3).contains(&u)
    || (0xA8F2..=0xA8F7).contains(&u)
    || (0xA8FB..=0xA8FB).contains(&u)
    || (0xA8FD..=0xA8FE).contains(&u)
    || (0xA90A..=0xA925).contains(&u)
    || (0xA930..=0xA946).contains(&u)
    || (0xA960..=0xA97C).contains(&u)
    || (0xA984..=0xA9B2).contains(&u)
    || (0xA9CF..=0xA9CF).contains(&u)
    || (0xA9E0..=0xA9E4).contains(&u)
    || (0xA9E6..=0xA9EF).contains(&u)
    || (0xA9FA..=0xA9FE).contains(&u)
    || (0xAA00..=0xAA28).contains(&u)
    || (0xAA40..=0xAA42).contains(&u)
    || (0xAA44..=0xAA4B).contains(&u)
    || (0xAA60..=0xAA76).contains(&u)
    || (0xAA7A..=0xAA7A).contains(&u)
    || (0xAA7E..=0xAAAF).contains(&u)
    || (0xAAB1..=0xAAB1).contains(&u)
    || (0xAAB5..=0xAAB6).contains(&u)
    || (0xAAB9..=0xAABD).contains(&u)
    || (0xAAC0..=0xAAC0).contains(&u)
    || (0xAAC2..=0xAAC2).contains(&u)
    || (0xAADB..=0xAADC).contains(&u)
    || (0xAADD..=0xAADD).contains(&u)
    || (0xAAE0..=0xAAEA).contains(&u)
    || (0xAAF2..=0xAAF4).contains(&u)
    || (0xAB01..=0xAB06).contains(&u)
    || (0xAB09..=0xAB0E).contains(&u)
    || (0xAB11..=0xAB16).contains(&u)
    || (0xAB20..=0xAB26).contains(&u)
    || (0xAB28..=0xAB2E).contains(&u)
    || (0xAB30..=0xAB5A).contains(&u)
    || (0xAB5C..=0xAB69).contains(&u)
    || (0xAB6B..=0xAB6B).contains(&u)
    || (0xAB70..=0xABE2).contains(&u)
    || (0xAC00..=0xD7A3).contains(&u)
    || (0xD7B0..=0xD7C6).contains(&u)
    || (0xD7CB..=0xD7FB).contains(&u)
    || (0xF900..=0xFA6D).contains(&u)
    || (0xFA70..=0xFAD9).contains(&u)
    || (0xFB00..=0xFB06).contains(&u)
    || (0xFB13..=0xFB17).contains(&u)
    || (0xFB1D..=0xFB1D).contains(&u)
    || (0xFB1F..=0xFB28).contains(&u)
    || (0xFB2A..=0xFB36).contains(&u)
    || (0xFB38..=0xFB3C).contains(&u)
    || (0xFB3E..=0xFB3E).contains(&u)
    || (0xFB40..=0xFB41).contains(&u)
    || (0xFB43..=0xFB44).contains(&u)
    || (0xFB46..=0xFBB1).contains(&u)
    || (0xFBD3..=0xFD3D).contains(&u)
    || (0xFD50..=0xFD8F).contains(&u)
    || (0xFD92..=0xFDC7).contains(&u)
    || (0xFDF0..=0xFDFB).contains(&u)
    || (0xFE70..=0xFE74).contains(&u)
    || (0xFE76..=0xFEFC).contains(&u)
    || (0xFF21..=0xFF3A).contains(&u)
    || (0xFF41..=0xFF5A).contains(&u)
    || (0xFF66..=0xFFBE).contains(&u)
    || (0xFFC2..=0xFFC7).contains(&u)
    || (0xFFCA..=0xFFCF).contains(&u)
    || (0xFFD2..=0xFFD7).contains(&u)
    || (0xFFDA..=0xFFDC).contains(&u)
}

/// Returns true if `c` has the Extended_Pictographic property (emoji-data.txt).
///
/// Used for WB3c: ZWJ x Extended_Pictographic.
#[must_use]
pub fn is_extended_pictographic(c: char) -> bool {
    let u = c as u32;
    matches!(
        u,
        0x00A9
            | 0x00AE
            | 0x203C
            | 0x2049
            | 0x2122
            | 0x2139
            | 0x2194..=0x2199
            | 0x21A9..=0x21AA
            | 0x231A..=0x231B
            | 0x2328
            | 0x2388
            | 0x23CF
            | 0x23E9..=0x23F3
            | 0x23F8..=0x23FA
            | 0x24C2
            | 0x25AA..=0x25AB
            | 0x25B6
            | 0x25C0
            | 0x25FB..=0x25FE
            | 0x2600..=0x2700
            | 0x2702..=0x27BF
            | 0x2934..=0x2935
            | 0x2B05..=0x2B07
            | 0x2B1B..=0x2B1C
            | 0x2B50
            | 0x2B55
            | 0x3030
            | 0x303D
            | 0x3297
            | 0x3299
            | 0x1F000..=0x1F0FF
            | 0x1F10D..=0x1F10F
            | 0x1F12F
            | 0x1F16C..=0x1F171
            | 0x1F17E..=0x1F17F
            | 0x1F18E
            | 0x1F191..=0x1F19A
            | 0x1F1AD..=0x1F1E5
            | 0x1F201..=0x1F20F
            | 0x1F21A
            | 0x1F22F
            | 0x1F232..=0x1F23A
            | 0x1F23B..=0x1F23F
            | 0x1F249..=0x1FFFD
    )
}
