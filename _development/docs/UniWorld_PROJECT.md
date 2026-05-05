# UniWorld: Correct Unicode Text Handling for Every Script

## Project Overview

UniWorld is an open-source library that correctly handles Unicode text operations — bidirectional layout, line breaking, cursor movement, text segmentation, normalization, and display — for all scripts in the Unicode standard. It is the missing layer between the Unicode specification (which defines how text *should* work) and the software ecosystem (which largely handles only Latin text correctly).

The library assembles published Unicode algorithms into a single, correct, well-tested, easy-to-integrate package. The algorithms are not new. The implementation as a cohesive, production-quality library that developers can drop into their projects is what's missing.

## Problem Statement

### The Current State

Billions of people use writing systems that current software handles poorly or incorrectly:

**Arabic and Hebrew** (right-to-left): Bidirectional text mixing (e.g., an Arabic sentence containing an English brand name) is routinely broken. Cursor movement through mixed-direction text is often wrong. Selection highlighting fails at direction boundaries. This affects ~450 million Arabic speakers and ~9 million Hebrew speakers, plus Persian, Urdu, and other RTL languages.

**Indic scripts** (Hindi, Bengali, Tamil, Telugu, etc.): Complex consonant clusters (conjuncts) are rendered incorrectly. Cursor movement treats visual clusters as multiple characters when they should be single units. Backspace deletes half a character. This affects over 1 billion people across South and Southeast Asia.

**Thai and Lao**: These languages don't use spaces between words. Line breaking requires dictionary-based word segmentation. Most software either breaks in the wrong place or doesn't break at all, producing lines that overflow. Combined: ~80 million speakers.

**CJK (Chinese, Japanese, Korean)**: Generally better supported, but edge cases in line breaking (particularly around punctuation and mixed Latin/CJK text) remain common. Vertical text layout support is rare outside specialized publishing software. Combined: ~1.5 billion speakers.

**Emoji and combining sequences**: Modern emoji use combining characters, skin tone modifiers, zero-width joiners, and flag sequences. Many text handling libraries treat these multi-codepoint sequences as multiple characters, breaking them during truncation, word wrap, or cursor movement.

**Developer environments specifically**: Code editors including Cursor on Windows exhibit Unicode handling issues when file paths, variable names, comments, or string literals contain non-Latin characters. This is directly relevant to the practitioner's experience and represents a concrete pain point that the project can address immediately.

### Why This Persists

The Unicode Consortium publishes comprehensive specifications for every algorithm needed: UAX #9 (Bidirectional Algorithm), UAX #14 (Line Breaking), UAX #29 (Text Segmentation), UAX #15 (Normalization), and others. These specifications are detailed, correct, and freely available.

The problem is that implementing them correctly is laborious, and most software developers work in Latin-script environments where the default behavior is "good enough." Partial implementations exist in various languages and frameworks, but they are scattered, inconsistently maintained, often incomplete (handling the common cases but failing on edge cases), and difficult to integrate as a cohesive system.

No single library currently provides a correct, complete implementation of the core Unicode text handling algorithms in a form that's easy to integrate into existing projects across multiple programming languages.

## What UniWorld Provides

### Core Algorithms (Unicode Technical Standard Implementations)

**1. Bidirectional Algorithm (UAX #9)**
Correct ordering of mixed left-to-right and right-to-left text. Handles all 23 bidi character types, explicit directional overrides, bracket pairing, and paragraph-level direction detection.

```python
class BidiResolver:
    """
    Full implementation of the Unicode Bidirectional Algorithm.
    
    Input: A string containing mixed-direction text.
    Output: The string with characters reordered for visual display,
            plus a mapping from logical to visual positions
            (needed for cursor movement and selection).
    
    Implements all rules in UAX #9 including:
    - Paragraph-level direction detection (P1-P3)
    - Explicit directional embeddings and overrides (X1-X10)
    - Weak type resolution (W1-W7)
    - Neutral type resolution (N0-N2)
    - Implicit level resolution (I1-I2)
    - Reordering (L1-L4)
    - Bracket pair resolution (BD16, N0)
    """
    
    def resolve(self, text: str) -> BidiResult:
        """
        Resolve bidirectional text into visual order.
        
        Returns BidiResult containing:
        - visual_order: text reordered for display
        - logical_to_visual: position mapping for cursor navigation
        - visual_to_logical: inverse mapping
        - paragraph_direction: detected base direction
        - embedding_levels: per-character embedding level (for rendering)
        """
        pass  # Full implementation in development
```

**2. Line Breaking Algorithm (UAX #14)**
Correct identification of permissible line break points for all scripts, including dictionary-based segmentation for Thai, Lao, Khmer, and Myanmar.

```python
class LineBreaker:
    """
    Full implementation of the Unicode Line Breaking Algorithm.
    
    Handles:
    - Standard break opportunities (spaces, hyphens, etc.)
    - CJK ideographic break points (break anywhere between ideographs)
    - Thai/Lao/Khmer word segmentation (dictionary-based)
    - Complex break rules around punctuation, numbers, URLs
    - Emergency breaks for long unbreakable sequences
    - Tailoring for specific languages and writing systems
    
    Dictionary data for Thai/Lao segmentation:
    - Thai: sourced from ICU Thai dictionary (~60,000 entries)
    - Lao: sourced from ICU Lao dictionary (~10,000 entries)
    - Myanmar: sourced from ICU Myanmar dictionary (~30,000 entries)
    - Khmer: sourced from ICU Khmer dictionary (~30,000 entries)
    
    All dictionaries are Unicode-licensed and freely redistributable.
    """
    
    def find_breaks(self, text: str, locale: str = None) -> List[BreakPoint]:
        """
        Find all permissible line break points in the text.
        
        Returns list of BreakPoint objects indicating:
        - position: character offset
        - type: mandatory, optional, or emergency
        - priority: for line-breaking algorithms that need to choose
                    among multiple options
        """
        pass
```

**3. Text Segmentation (UAX #29)**
Correct identification of grapheme cluster, word, and sentence boundaries.

```python
class TextSegmenter:
    """
    Full implementation of Unicode Text Segmentation.
    
    Grapheme cluster boundaries: What a user perceives as a single
    "character." Critical for:
    - Cursor movement (one keypress = one grapheme cluster)
    - Backspace (delete one grapheme cluster)
    - Text truncation (never split a grapheme cluster)
    - Character counting (user-visible character count)
    
    Examples of multi-codepoint grapheme clusters:
    - Emoji with skin tone: 👩🏽 (2 codepoints)
    - Family emoji: 👨‍👩‍👧‍👦 (7 codepoints, 1 grapheme cluster)
    - Hindi conjuncts: क्ष (3 codepoints, 1 visual unit)
    - Korean syllables: 한 (1 codepoint, or 3 jamo if decomposed)
    - Flag sequences: 🇯🇵 (2 regional indicator codepoints)
    
    Word boundaries: For word selection (double-click),
    spell checking, and word-level operations.
    
    Sentence boundaries: For sentence-level operations and
    text-to-speech segmentation.
    """
    
    def grapheme_clusters(self, text: str) -> List[GraphemeCluster]:
        """Segment text into grapheme clusters."""
        pass
    
    def words(self, text: str, locale: str = None) -> List[Word]:
        """Segment text into words (locale-sensitive)."""
        pass
    
    def sentences(self, text: str, locale: str = None) -> List[Sentence]:
        """Segment text into sentences."""
        pass
```

**4. Normalization (UAX #15)**
All four Unicode normalization forms: NFC, NFD, NFKC, NFKD. Plus stream-safe normalization for processing text in chunks.

**5. Case Mapping and Folding**
Full Unicode case mapping including locale-specific rules (Turkish İ/i, German ß/SS, Greek final sigma, Lithuanian dot-above handling).

### Composite Operations (Built on Core Algorithms)

**6. Cursor Navigation**
Correct cursor movement through complex text, combining bidi resolution with grapheme cluster boundaries:

```python
class CursorNavigator:
    """
    Handles cursor movement in text containing any combination of:
    - Mixed-direction text (Arabic/English, Hebrew/English)
    - Complex grapheme clusters (Indic conjuncts, emoji sequences)
    - Combining marks and diacritics
    
    Arrow-right moves to the next VISUAL position, which may be
    the next logical position (in LTR text), the previous logical
    position (in RTL text), or may skip multiple codepoints
    (in complex grapheme clusters).
    """
    
    def move_right(self, text: str, current_position: int, bidi_result: BidiResult) -> int:
        """Move cursor one visual position to the right."""
        pass
    
    def move_left(self, text: str, current_position: int, bidi_result: BidiResult) -> int:
        """Move cursor one visual position to the left."""
        pass
    
    def delete_forward(self, text: str, current_position: int) -> Tuple[str, int]:
        """Delete one grapheme cluster forward (Delete key)."""
        pass
    
    def delete_backward(self, text: str, current_position: int) -> Tuple[str, int]:
        """Delete one grapheme cluster backward (Backspace key)."""
        pass
    
    def select_word(self, text: str, position: int) -> Tuple[int, int]:
        """Select the word at the given position (double-click)."""
        pass
```

**7. Display Width Calculation**
Correct calculation of display width for terminal and fixed-width font contexts, handling full-width CJK characters, zero-width joiners, combining marks, and ambiguous-width characters.

**8. String Truncation**
Safe truncation that never breaks grapheme clusters, never splits bidi runs at unsafe positions, and preserves text validity.

## Language Coverage and Testing Strategy

### Scripts and Languages Covered

The Unicode standard (version 16.0) defines 161 scripts. UniWorld aims for correct handling of all scripts, with explicit test coverage for the following priority tiers:

**Tier 1 — Full test coverage, all algorithms (10 scripts, ~5 billion speakers):**
- Latin (English, French, German, Spanish, Portuguese, Turkish, Vietnamese, etc.)
- Arabic (Arabic, Persian, Urdu, Pashto)
- Devanagari (Hindi, Marathi, Sanskrit, Nepali)
- Bengali (Bengali, Assamese)
- Han/CJK (Chinese Simplified, Chinese Traditional, Japanese Kanji)
- Hiragana/Katakana (Japanese)
- Hangul (Korean)
- Thai
- Hebrew
- Cyrillic (Russian, Ukrainian, Bulgarian, Serbian)

**Tier 2 — Core algorithm test coverage (15 scripts, ~1 billion speakers):**
- Tamil, Telugu, Kannada, Malayalam (South Indian)
- Gujarati, Gurmukhi, Odia (North Indian)
- Myanmar (Burmese)
- Khmer (Cambodian)
- Lao
- Tibetan
- Sinhala
- Georgian
- Armenian
- Ethiopic (Amharic, Tigrinya)
- Greek

**Tier 3 — Conformance test pass, limited manual testing (remaining scripts):**
- All other scripts defined in Unicode 16.0
- Tested against Unicode Consortium conformance test suites
- Manual test cases added as community contributions come in

### Test Data Sources

The Unicode Consortium provides official conformance test files for each algorithm:

- **UAX #9 (Bidi)**: `BidiTest.txt` and `BidiCharacterTest.txt` — exhaustive test cases for the bidirectional algorithm. Freely downloadable from unicode.org.
- **UAX #14 (Line Breaking)**: `LineBreakTest.txt` — test cases for line break identification. Freely downloadable.
- **UAX #29 (Segmentation)**: `GraphemeBreakTest.txt`, `WordBreakTest.txt`, `SentenceBreakTest.txt` — test cases for all segmentation types. Freely downloadable.
- **UAX #15 (Normalization)**: `NormalizationTest.txt` — comprehensive normalization test cases. Freely downloadable.

These are the primary test suites. In addition:

- **ICU test suite**: The International Components for Unicode project has extensive test cases. ICU is licensed under the ICU License (Unicode License), fully compatible with open source.
- **CLDR**: The Common Locale Data Repository provides locale-specific tailoring data for segmentation, case mapping, and collation. Freely available.
- **Script-specific test cases**: Generated from native-language corpora for each Tier 1 script. Sources include Wikipedia dumps (freely available), Project Gutenberg texts, and community-contributed test documents.

### Dictionary Data for Southeast Asian Languages

Thai, Lao, Khmer, and Myanmar line breaking requires word segmentation dictionaries. Sources:

- **ICU dictionaries**: The ICU project includes word break dictionaries for these languages, licensed under the ICU License. These are the standard reference dictionaries and are freely redistributable.
- **PyThaiNLP**: Open-source Thai NLP library with word segmentation dictionaries and models. Apache 2.0 license.
- **Supplementary**: Community contributions for specialized vocabulary (technical terms, neologisms).

Dictionary formats and loading:
```python
class SegmentationDictionary:
    """
    Dictionary for word segmentation in scripts without spaces.
    
    Supports:
    - Trie-based longest-match segmentation (fast, deterministic)
    - Fallback heuristic for out-of-vocabulary sequences
    - Dictionary hot-loading for adding custom vocabulary
    
    Built-in dictionaries are loaded lazily on first use of the
    relevant locale. Total dictionary size for all languages: ~15MB.
    """
    
    BUILTIN_DICTIONARIES = {
        'th': 'data/dictionaries/thai_icu.dat',      # ~60,000 entries
        'lo': 'data/dictionaries/lao_icu.dat',       # ~10,000 entries
        'km': 'data/dictionaries/khmer_icu.dat',     # ~30,000 entries
        'my': 'data/dictionaries/myanmar_icu.dat',   # ~30,000 entries
    }
```

## Architecture

### Implementation Languages

**Core library: Rust**

Rationale: Unicode text processing is performance-sensitive (it runs on every keystroke in text editors and on every line in text layout). Rust provides C-level performance with memory safety, no runtime dependencies, and excellent FFI for bindings to other languages.

The Rust core compiles to a shared library with a C ABI, enabling bindings for:

**Bindings (generated from Rust core):**
- **Python** (via PyO3): For scripting, data processing, NLP pipelines
- **JavaScript/TypeScript** (via wasm-bindgen): For web applications and Node.js
- **C/C++** (via cbindgen): For native applications and editor integration
- **Go** (via CGo): For server-side applications

### Module Structure

```
uniworld/
├── core/                    # Rust core library
│   ├── bidi/               # UAX #9 Bidirectional Algorithm
│   │   ├── types.rs        # Character type classification
│   │   ├── levels.rs       # Level resolution
│   │   ├── reorder.rs      # Visual reordering
│   │   └── brackets.rs     # Bracket pair resolution
│   ├── linebreak/          # UAX #14 Line Breaking
│   │   ├── rules.rs        # Break rule table and resolution
│   │   ├── dictionary.rs   # Dictionary-based segmentation
│   │   └── tailoring.rs    # Locale-specific tailoring
│   ├── segment/            # UAX #29 Text Segmentation
│   │   ├── grapheme.rs     # Grapheme cluster boundaries
│   │   ├── word.rs         # Word boundaries
│   │   └── sentence.rs     # Sentence boundaries
│   ├── normalize/          # UAX #15 Normalization Forms
│   ├── casemap/            # Case mapping and folding
│   ├── cursor/             # Composite cursor navigation
│   ├── width/              # Display width calculation
│   ├── truncate/           # Safe string truncation
│   └── data/               # Unicode property tables and dictionaries
│       ├── ucd/            # Unicode Character Database (auto-generated)
│       ├── dictionaries/   # Word break dictionaries
│       └── cldr/           # Locale-specific data from CLDR
├── bindings/
│   ├── python/             # PyO3 bindings
│   ├── js/                 # wasm-bindgen bindings
│   ├── c/                  # C header generation
│   └── go/                 # CGo bindings
├── tools/
│   ├── ucd_gen/            # Tool to regenerate UCD data tables from Unicode releases
│   ├── conformance/        # Conformance test runners
│   └── benchmark/          # Performance benchmarks
├── tests/
│   ├── conformance/        # Official Unicode conformance tests
│   ├── scripts/            # Per-script integration tests
│   └── regression/         # Regression tests from bug reports
└── docs/
    ├── integration/        # Integration guides per language/platform
    ├── scripts/            # Per-script usage guides
    └── contributing/       # How to add test cases and dictionary entries
```

### Unicode Version Management

The library includes a code generation tool (`ucd_gen`) that reads the Unicode Character Database files for any Unicode version and generates the Rust property lookup tables. When a new Unicode version is released, updating the library is:

```bash
# Download new UCD files
./tools/ucd_gen/download.sh 16.0

# Regenerate property tables
cargo run --bin ucd_gen -- --version 16.0 --output core/data/ucd/

# Run conformance tests against new test files
cargo test --features conformance
```

This ensures the library tracks Unicode releases without manual table editing.

## Development Phases

### Phase 1: Core Algorithms in Rust (Target: 4-6 weeks)

**Week 1-2: Text Segmentation (UAX #29)**
- Grapheme cluster boundary detection
- Word boundary detection
- Sentence boundary detection
- Pass all official conformance tests
- This is the foundation — cursor movement, truncation, and most user-facing operations depend on correct grapheme clusters

**Week 2-3: Normalization (UAX #15)**
- All four normalization forms (NFC, NFD, NFKC, NFKD)
- Stream-safe normalization
- Quick check properties for fast-path normalization testing
- Pass all official conformance tests

**Week 3-5: Bidirectional Algorithm (UAX #9)**
- Full bidi resolution including bracket pairing
- Logical-to-visual and visual-to-logical position mapping
- Pass all official conformance tests (BidiTest.txt: ~500,000 test cases)

**Week 5-6: Line Breaking (UAX #14)**
- Rule-based line breaking for all scripts
- Dictionary integration for Thai, Lao, Khmer, Myanmar
- Locale-specific tailoring
- Pass all official conformance tests

### Phase 2: Composite Operations and Bindings (Target: 3-4 weeks)

**Week 7-8: Composite operations**
- Cursor navigation (bidi + grapheme clusters)
- Display width calculation
- Safe truncation
- Case mapping with locale awareness

**Week 8-10: Language bindings**
- Python bindings via PyO3 (highest priority — immediate utility)
- JavaScript/WASM bindings (web applications)
- C header generation (editor and application integration)
- Per-binding test suites

### Phase 3: Testing and Documentation (Target: 2-3 weeks)

**Week 10-11: Script-specific integration tests**
- Tier 1 scripts: comprehensive real-world test cases
- Mixed-script test cases (the hardest and most commonly broken scenarios)
- Performance benchmarks across scripts

**Week 11-12: Documentation**
- Integration guides for Python, JavaScript, C
- Per-script usage guides with examples
- Contributing guide for test cases and dictionary additions
- API reference (auto-generated from doc comments)

### Phase 4: Community and Ecosystem (Ongoing)

- Package publication: crates.io (Rust), PyPI (Python), npm (JS)
- Editor integration examples (VS Code extension, Cursor plugin)
- Community test case submission process
- Dictionary contribution pipeline for Southeast Asian languages
- Outreach to international developer communities

## Specific Value for Developer Environments

### Cursor/VS Code Integration Path

UniWorld can be integrated into text editors as a library that handles:
- Correct cursor movement in files containing non-Latin text
- Correct word selection (double-click) in mixed-script text
- Correct line wrapping in terminal panels
- Correct display width calculation for alignment in code files
- Correct handling of Unicode in file paths and identifiers

A VS Code/Cursor extension that replaces the built-in text handling for specific operations with UniWorld calls would directly address the Unicode issues experienced during development. This extension would be a concrete, visible demonstration of the library's value.

### Terminal Emulator Integration

Terminal emulators (Windows Terminal, iTerm, Alacritty) all have Unicode handling issues. UniWorld's display width calculation and grapheme cluster segmentation can replace the incomplete implementations in these tools. A patch to a popular terminal emulator using UniWorld would be high-visibility and immediately beneficial.

## Licensing

MIT license for all code. Unicode Character Database data is used under the Unicode License. ICU dictionaries are used under the ICU License. All licenses are compatible with unrestricted open-source use including commercial applications.

## Community and Contribution Strategy

### International Contributions

The project's value proposition is inherently international. The people best equipped to test Arabic handling are Arabic speakers. The people best equipped to test Tamil handling are Tamil speakers. The contribution model should make it easy for non-Rust-developers to contribute:

- **Test case contributions**: Simple JSON or text format for submitting "this text should produce these grapheme clusters / line breaks / bidi ordering." No Rust knowledge required.
- **Dictionary contributions**: Plain text word lists for expanding Thai/Lao/Khmer/Myanmar dictionaries. Review process includes native speaker verification.
- **Bug reports with examples**: Template that captures the text, the expected behavior, the actual behavior, and the platform. No code knowledge required.
- **Localization of documentation**: Guides for using UniWorld to handle specific scripts, written by native speakers of those scripts.

### Corporate Engagement

Companies with international user bases (Google, Apple, Microsoft, Meta, Adobe) have internal teams working on Unicode handling. UniWorld as an open-source reference implementation could serve as a test oracle for their implementations, even if they don't adopt the library directly. Engagement path: present conformance test results showing where major platforms fail, offer UniWorld as a reference.

## Press and Visibility Strategy

UniWorld is designed to be a press-worthy project because:
1. **It solves a problem affecting billions of people** that the tech industry has neglected
2. **It's fully open source** with a clear contribution path for international communities
3. **It's backed by standards** (Unicode Technical Standards), not opinions
4. **It has a visible demo**: side-by-side comparison of text handling in popular software vs UniWorld-corrected handling, across multiple scripts
5. **It invites corporate accountability**: conformance test results showing where major platforms fail

For incorporation and investment context, UniWorld demonstrates:
- Technical leadership and execution capability
- Values alignment (solving problems for underserved populations)
- Community building competence
- Standards-based engineering discipline
- International perspective and market awareness
