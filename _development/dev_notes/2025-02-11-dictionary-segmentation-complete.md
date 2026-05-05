# Dictionary-Based Word Segmentation - Complete

**Date**: 2025-02-11  
**Session**: Dictionary segmentation for Thai/Lao/Khmer/Myanmar  
**Status**: Complete  

## Summary

Implemented dictionary-based word segmentation for Southeast Asian scripts
(Line_Break class SA) using ICU-sourced word lists. This enables correct
line break opportunities in Thai, Lao, Khmer, and Myanmar text, which do
not use spaces between words.

## Results

- **179,081 dictionary words** across 4 languages
- **15 integration tests** passing
- **52 total tests** in full suite (all passing, zero warnings)
- **No regressions** to existing conformance tests (884,508 cases still pass)

## Files Created/Modified

### New Files
- `src/linebreak/dictionary.rs` -- Dictionary module (OnceLock lazy init,
  HashSet lookup, longest-match segmentation, script detection)
- `src/data/dictionaries/thai.dict` -- 26,383 Thai words (492 KB)
- `src/data/dictionaries/lao.dict` -- 30,550 Lao words (672 KB)
- `src/data/dictionaries/khmer.dict` -- 81,028 Khmer words (2.0 MB)
- `src/data/dictionaries/myanmar.dict` -- 41,120 Myanmar words (1.2 MB)
- `tests/dictionary_segmentation.rs` -- 15 integration tests
- `_development/scripts/generate_dictionary_data.py` -- Dictionary cleaner
- `_development/data/dictionaries/` -- Raw ICU source dictionaries

### Modified Files
- `src/linebreak/mod.rs` -- Added `pub mod dictionary`, new public API
  `line_break_opportunities_with_dictionary()`, internal `apply_dictionary_breaks()`
- `.cursor/rules/PROJECT_ROADMAP.mdc` -- Dictionary segmentation marked complete

## Architecture

### Data Pipeline
1. ICU dictionaries downloaded from `unicode-org/icu` GitHub (brkitr/dictionaries)
2. Python script cleans files (removes BOM, comments, blank lines)
3. Clean `.dict` files placed in `src/data/dictionaries/`
4. Rust `include_str!()` embeds at compile time
5. `OnceLock<Dictionary>` lazily initializes HashSet on first use

### Algorithm: Forward Longest Match
```
For each position in an SA run:
  1. Try dictionary lookup from longest possible match down to 1 char
  2. If match found: advance past the word, record word boundary
  3. If no match: advance by one character (unknown word)
  4. Repeat until end of run
```

### Integration with UAX #14
- `line_break_opportunities_with_dictionary(text)` wraps the standard
  `line_break_opportunities(text)` and post-processes SA-class runs
- Within SA runs: first prohibit all internal breaks, then allow breaks
  at dictionary-determined word boundaries
- Non-SA text is completely unaffected

### Script Detection
Code point ranges map to languages:
- Thai: U+0E01-U+0E5B
- Lao: U+0E81-U+0EDF
- Myanmar: U+1000-U+109F, U+AA60-U+AA7F
- Khmer: U+1780-U+17FF, U+19E0-U+19FF

## Complexity Assessment

This task was **substantially easier** than UAX #14 line breaking:

| Dimension | UAX #14 | Dictionary Segmentation |
|-----------|---------|------------------------|
| Algorithm | 30+ interacting rules | Single longest-match |
| Data structures | Complex rule engine | HashSet lookup |
| Conformance tests | 19,338 strict | None (validation only) |
| Context sensitivity | CM transparency, lookahead | Script range matching |
| Risk of regression | High (rule interactions) | Low (post-processing) |
| Development time | ~1.5 hours model time | ~20 minutes model time |
| Estimated cost | ~$25 | ~$3-5 |

## Dictionary Sources

All dictionaries sourced from ICU (International Components for Unicode):
- License: Unicode License (http://www.unicode.org/copyright.html)
- Repository: https://github.com/unicode-org/icu
- Path: icu4c/source/data/brkitr/dictionaries/
- Thai: IBM/Apple + contributors
- Lao: Brian Eugene Wilson, Robert Martin Campbell (BSD-2-Clause)
- Khmer: ICU contributors
- Myanmar: ICU contributors

## Potential Future Improvements

1. **Trie-based lookup**: Replace HashSet with a packed trie for O(word_length)
   lookup instead of O(1) amortized but with higher init cost
2. **Feature flag**: Make dictionary data opt-in to reduce binary size (~4.4 MB)
3. **DAFSA compression**: Directed Acyclic Finite State Automaton for minimal
   memory footprint
4. **Backtracking**: More sophisticated segmentation (NewMM algorithm) that
   tries alternate segmentations when the greedy longest-match produces
   poor results
5. **Tai Tham, New Tai Lue, Tai Le**: Additional SE Asian scripts without
   ICU dictionaries
