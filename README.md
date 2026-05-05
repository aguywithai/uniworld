# UniWorld

**Correct Unicode text handling for every script.**

UniWorld is an open-source library, a set of language bindings, and developer tools that implement the Unicode standard's core text algorithms -- all from a single, conformance-tested Rust core. It ships as a library (Rust, Python, JavaScript/WASM, C, Go), a [VS Code extension](extensions/vscode/README.md), and a [PowerShell module](extensions/powershell/README.md).

**[Project website](https://aguywithai.world/opensource/uniworld/)** -- Full documentation, install guides, and the complete UniWorld ecosystem. Short domain [uniworld.world](https://uniworld.world) should redirect to that URL once DNS is configured.

---

## The problem UniWorld solves

Unicode text handling is one of the most pervasive unsolved problems in everyday software. It affects everyone:

**If you work in English or other Latin-script languages**, you've seen emoji split apart by your cursor, combining accents orphaned by backspace, and pasted text that looks identical but doesn't match because of invisible normalization differences. Your terminal miscounts column widths when it encounters fullwidth characters. Your truncation logic cuts strings in the middle of grapheme clusters. These are Unicode problems, and they happen constantly in English-language workflows.

**If you work with Arabic, Hebrew, or any right-to-left script**, correct bidirectional layout is essential and routinely broken. Numbers embedded in RTL paragraphs reorder incorrectly. Cursor movement goes the wrong direction. Mixed-direction text renders as gibberish.

**If you work with Thai, Lao, Khmer, or Myanmar**, your text has no spaces between words. Line breaking requires dictionary-based segmentation that most tools simply don't have. Text wraps mid-word or not at all.

**If you work with CJK (Chinese, Japanese, Korean), Indic scripts (Devanagari, Bengali, Tamil), or emoji**, selection and editing break on complex characters. Cursors land inside ligatures, conjuncts, and ZWJ sequences. Column counts are wrong. Truncation corrupts display.

The Unicode Consortium publishes the algorithms to handle all of this correctly. Most implementations address only one or two, partially, for a subset of scripts. UniWorld implements five core standards completely and makes them available everywhere.

## What UniWorld provides

| Algorithm | Standard | What it does |
|-----------|----------|--------------|
| **Bidirectional layout** | [UAX #9](https://unicode.org/reports/tr9/) | Correct visual ordering and cursor mapping for mixed LTR/RTL text |
| **Line breaking** | [UAX #14](https://unicode.org/reports/tr14/) | Rule-based and dictionary-based break opportunities, including Thai, Lao, Khmer, Myanmar (179,081-word dictionary from ICU) |
| **Text segmentation** | [UAX #29](https://unicode.org/reports/tr29/) | Grapheme cluster, word, and sentence boundaries for cursor movement, backspace, selection |
| **Normalization** | [UAX #15](https://unicode.org/reports/tr15/) | NFC, NFD, NFKC, NFKD for canonical equivalence and compatibility |
| **Display width** | [East Asian Width](https://unicode.org/reports/tr11/) | True terminal column count (CJK=2, emoji=2, combining=0) |
| **Safe truncation** | -- | Truncate to N display columns without breaking grapheme clusters |
| **Case mapping** | [Unicode CaseFolding](https://www.unicode.org/reports/tr44/) | Full Unicode upper/lower/title/fold with special casing (Turkish, Lithuanian, Greek final sigma) |
| **Cursor navigation** | UAX #9 + #29 | Logical and visual cursor movement respecting grapheme clusters and bidi |

### Conformance

Every algorithm is tested against the official Unicode conformance test suites for **UCD 17.0.0**. Run `cargo test --features conformance`; the harness prints pass totals. Row counts below match the number of test lines in each file except **BidiTest.txt**, which expands each data row across paragraph directions (see printed total).

| Test suite | Cases (rows in UCD 17.0.0 files) |
|------------|--------------|
| Bidi (BidiTest.txt) | total printed by tests |
| Bidi character (BidiCharacterTest.txt) | 91,707 |
| Line break (LineBreakTest.txt) | 19,338 |
| Word segmentation (WordBreakTest.txt) | 1,944 |
| Grapheme segmentation (GraphemeBreakTest.txt) | 766 |
| Sentence segmentation (SentenceBreakTest.txt) | 512 |
| Normalization (NormalizationTest.txt) | Full (all 5 parts) |

Unicode 17.0 throughout (UCD 17.0.0 data files).

## Get UniWorld

### Rust (core library)

```bash
cargo add uniworld
```

[crates.io/crates/uniworld](https://crates.io/crates/uniworld) | [API docs](https://docs.rs/uniworld)

### Python

```bash
pip install uniworld
```

[pypi.org/project/uniworld](https://pypi.org/project/uniworld/) | [Integration guide](docs/integration/python.md)

### JavaScript / WASM

```bash
npm install uniworld
```

[npmjs.com/package/uniworld](https://www.npmjs.com/package/uniworld) | [Integration guide](docs/integration/javascript-wasm.md)

### C

```bash
cargo build --release --features cffi
cbindgen --crate uniworld --output uniworld.h
```

[Integration guide](docs/integration/c.md)

### Go

```bash
cargo build --release --features cffi
cd bindings/go && go test
```

[Integration guide](docs/integration/go.md)

### VS Code extension

Search "UniWorld" in the Extensions panel, or:

```
ext install aguywithai.uniworld
```

[VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld) | [Extension README](extensions/vscode/README.md)

Grapheme-aware cursor and delete, bidi visualization, display width, Unicode inspector, normalization commands, line break decorations, script-aware word selection. See the [full feature list](extensions/vscode/README.md#features).

### PowerShell module

```powershell
Install-Module UniWorld
```

[PowerShell Gallery](https://www.powershellgallery.com/packages/UniWorld) | [Module README](extensions/powershell/README.md)

12 cmdlets: `Get-GraphemeBoundaries`, `Get-WordBoundaries`, `Get-SentenceBoundaries`, `Get-DisplayWidth`, `Limit-DisplayWidth`, `ConvertTo-NFC`, `ConvertTo-NFD`, `ConvertTo-NFKC`, `ConvertTo-NFKD`, `Get-BidiClasses`, `Get-LineBreakOpportunities`, `Get-UnicodeInfo`. See the [full cmdlet reference](extensions/powershell/README.md#cmdlets).

## Quick start

**Rust**
```rust
use uniworld::{grapheme_boundaries, display_width, normalize_nfc};

let clusters = grapheme_boundaries("cafe\u{0301}");  // ["c", "a", "f", "e\u{0301}"]
let nfc = normalize_nfc("cafe\u{0301}");              // "cafe" (composed e-acute)
let width = display_width("Hello");                    // 5
```

**Python**
```python
import uniworld

uniworld.grapheme_boundaries("cafe\u0301")   # ["c", "a", "f", "e\u0301"]
uniworld.display_width("Hello")              # 10 (CJK)
uniworld.normalize_nfc("cafe\u0301")         # "cafe" (composed)
```

**PowerShell**
```powershell
Import-Module UniWorld
"Hello" | Get-DisplayWidth                   # 5
"cafe`u{0301}" | ConvertTo-NFC              # composed e-acute
Get-BidiClasses "Hello" | Format-Table       # per-character bidi levels
```

## Architecture

```
                         UniWorld Rust core
                        /    |    |    \    \
                      /      |    |     \     \
                 Python   JS/WASM  C    Go    cdylib
                (PyO3)  (wasm-   (FFI) (CGo)  (DLL/so/dylib)
                         bindgen)              |
                                        C# P/Invoke
                                              |
                    VS Code extension    PowerShell module
                    (WASM binding)       (native FFI)
```

One Rust implementation. Every binding shares the same algorithms, the same data tables, and the same conformance test results. The behavior is identical everywhere because it is the same code.

## Build and test

```bash
# Core library
cargo build
cargo test

# With conformance tests (requires test data in _development/data/)
cargo test --features conformance

# C FFI (for PowerShell / C / Go)
cargo build --release --features cffi

# WASM (for VS Code / JavaScript)
wasm-pack build --release --features wasm --no-default-features

# VS Code extension
cd extensions/vscode && npm install && npm run compile

# PowerShell module
Import-Module extensions/powershell/UniWorld.psd1
Invoke-Pester -Path extensions/powershell/Tests/
```

## Scripts covered

UniWorld correctly handles text in: Latin, Greek, Cyrillic, Arabic, Hebrew, Devanagari, Bengali, Gurmukhi, Tamil, Sinhala, Thai, Lao, Khmer, Myanmar, Chinese (Simplified/Traditional), Japanese (Kanji + Hiragana + Katakana), Korean (Hangul), Ethiopic, Tifinagh, Cherokee, Canadian Aboriginal Syllabics (Cree, Inuktitut, Ojibwe), and emoji (including ZWJ sequences, skin tones, and flag pairs).

See the [Unicode Showcase](docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md) for a comprehensive stress-test document demonstrating UniWorld across all supported scripts.

## Documentation

| Document | Description |
|----------|-------------|
| [aguywithai.world/opensource/uniworld](https://aguywithai.world/opensource/uniworld/) | Project website (canonical); [uniworld.world](https://uniworld.world) redirects here when live |
| [VS Code Extension README](extensions/vscode/README.md) | Features, settings, commands, development |
| [PowerShell Module README](extensions/powershell/README.md) | Cmdlets, pipeline usage, architecture |
| [Python integration](docs/integration/python.md) | PyO3 binding setup and API |
| [JavaScript/WASM integration](docs/integration/javascript-wasm.md) | wasm-bindgen setup and API |
| [C integration](docs/integration/c.md) | C FFI API and header generation |
| [Go integration](docs/integration/go.md) | CGo wrapper setup and API |
| [Unicode Showcase](docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md) | Multi-script stress test and demo |
| [Project specification](_development/docs/UniWorld_PROJECT.md) | Full architecture, design decisions, and phase history |

## Repository layout

```
README.md                          # This file
src/                               # Rust core (algorithms, data tables, bindings)
tests/                             # Rust integration tests
docs/                              # User-facing docs (integration guides, showcase)
extensions/vscode/                 # VS Code extension (TypeScript + WASM)
extensions/powershell/             # PowerShell module (cmdlets + native FFI)
bindings/go/                       # Go CGo wrapper
_development/                      # Dev-only: notes, scripts, working docs
_publishing/                       # Publishing: marketing, site, outreach
.github/workflows/                 # CI: cross-platform native library builds
```

## Contributing

See [CONTRIBUTING.md](docs/contributing/CONTRIBUTING.md) for build instructions, test procedures, and how to submit test cases or dictionary entries.

## License

MIT. See [LICENSE](LICENSE).

Unicode Character Database data is used under the [Unicode License](https://www.unicode.org/license.txt). ICU dictionary data is used under the [ICU License](https://github.com/nickel-org/nickel.rs). Both are permissive and compatible with commercial use.

---

UniWorld is an [A Guy With AI](https://aguywithai.world) project by Sean MacNutt, developed using [HAIMU](https://haimu.world), the AI development methodology also originated by MacNutt. HAIMU (Human-AI Mutual Understandability) generated the insight that led to UniWorld -- when prompted for the largest-ROI neglected technical benefit projects an AI could conceive of, correct Unicode handling emerged as the clear winner. The library was largely built within 14 hours of project idea generation. "Move fast and fix things." Initial development funded by [Grand Beta](https://grandbeta.world). Visit the **[project website](https://aguywithai.world/opensource/uniworld/)** for the full ecosystem.
