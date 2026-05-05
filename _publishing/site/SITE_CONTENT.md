# UniWorld website content plan

This document defines what the site (GitHub Pages / uniworld.world) should include. Use it as the brief for building `index.html` and any additional pages. The site is the public face of the project: it should explain what UniWorld is, why it matters, how to get it, and what tools are available.

---

## Page structure

### 1. Hero / above the fold

**Headline**: UniWorld

**Tagline**: Correct Unicode text handling for every script.

**One-paragraph summary**: UniWorld is an open-source library that implements the Unicode standard's text algorithms -- bidirectional layout (UAX #9), line breaking (UAX #14), text segmentation (UAX #29), and normalization (UAX #15) -- in a single, tested Rust core with bindings for Python, JavaScript/WASM, C, and Go. It also ships as a VS Code extension and a PowerShell module. Visit **[uniworld.world](https://uniworld.world)** for the full project.

**Primary CTA**: Link to GitHub repo.
**Secondary CTAs**: VS Code Marketplace | PowerShell Gallery | uniworld.world

---

### 2. The problem

Correct text handling is one of the most pervasive unsolved problems in everyday software -- and it affects everyone, not only users of non-Latin scripts.

**If you work in English or other Latin-script languages:**
- Your cursor splits emoji apart. Backspace removes only half a flag or family emoji.
- Accented characters (French, German, Spanish, Vietnamese) get their combining marks orphaned by backspace.
- Strings that look identical fail comparison because of invisible normalization differences (NFC vs NFD).
- Your terminal miscounts column widths when CJK or fullwidth characters appear.
- Your truncation logic cuts strings in the middle of grapheme clusters, producing garbled output.

**If you work with other scripts, the problems multiply:**
- **Arabic and Hebrew** need right-to-left layout, mixed with left-to-right numbers and Latin words. Most tools get bidi ordering wrong.
- **Thai, Lao, Khmer, Myanmar** don't put spaces between words; line breaking requires dictionary segmentation that most tools simply don't have.
- **Hindi, Bengali, Tamil** and other Indic scripts form complex conjuncts where cursor movement, selection, and backspace must respect grapheme clusters.
- **Chinese, Japanese, Korean** characters occupy two terminal columns; naive code counts them as one.

The Unicode Consortium has published the algorithms to handle all of this for decades. Few implementations are complete and easy to integrate. This has been neglected for too long. UniWorld changes that.

---

### 3. What UniWorld provides

Present as a grid or feature cards:

| Feature | Standard | What it does |
|---------|----------|--------------|
| **Bidirectional algorithm** | UAX #9 | Correct visual ordering and cursor mapping for mixed LTR/RTL text |
| **Line breaking** | UAX #14 | Rule-based + dictionary-based (Thai, Lao, Khmer, Myanmar) line break opportunities |
| **Text segmentation** | UAX #29 | Grapheme cluster, word, and sentence boundaries (cursor, backspace, selection) |
| **Normalization** | UAX #15 | NFC, NFD, NFKC, NFKD for canonical equivalence and compatibility |
| **Display width** | East Asian Width | True terminal column count (CJK=2, emoji=2) |
| **Safe truncation** | -- | Truncate to N columns without breaking grapheme clusters |
| **Case mapping** | Unicode CaseFolding | Full Unicode upper/lower/title/fold with locale awareness (Turkish, Lithuanian, Greek final sigma) |
| **Cursor navigation** | UAX #9 + #29 | Logical and visual cursor movement that respects grapheme clusters and bidi |

---

### 4. Why UniWorld (the value proposition)

Short narrative section. Key points:

- **One core, many languages**: Rust core with Python, JS/WASM, C, Go bindings. Every binding shares the same behavior and the same tests.
- **Spec-aligned and tested**: Full conformance against Unicode 17.0 test suites (770K+ bidi tests, 19K+ line break tests, 3K+ segmentation tests).
- **Pipeline consistency**: When we built the Unicode showcase (a stress-test document rendered to HTML and PDF), the hard problems were not in the library -- they were in the rendering pipeline. If those tools had used UniWorld for segmentation, width, and line breaking, the layout would have been predictable. That is the value: every layer agrees on boundaries and widths.
- **Tools for developers**: The VS Code extension and PowerShell module bring UniWorld directly into the editor and terminal, so you can inspect, navigate, and transform Unicode text without writing code.

---

### 5. Get UniWorld

Present as tabs or cards by platform:

#### Rust (core library)
```bash
cargo add uniworld
```
Link: [crates.io/crates/uniworld](https://crates.io/crates/uniworld)

#### Python
```bash
pip install uniworld
```
Link: [pypi.org/project/uniworld](https://pypi.org/project/uniworld/)

#### JavaScript / WASM
```bash
npm install uniworld
```
Link: [npmjs.com/package/uniworld](https://www.npmjs.com/package/uniworld)

#### C / Go
Clone the repo and build with `cargo build --release`. C header via cbindgen; Go wrapper in `bindings/go/`.

Link: [GitHub](https://github.com/aguywithai/uniworld)

#### VS Code Extension
Search "UniWorld" in the VS Code Extensions panel, or:
```
ext install aguywithai.uniworld
```
Link: [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld)

Features: grapheme-aware cursor, display width, Unicode inspector, normalization, safe truncation.

#### PowerShell Module
```powershell
Install-Module UniWorld
```
Link: [PowerShell Gallery](https://www.powershellgallery.com/packages/UniWorld)

Features: `Get-GraphemeBoundaries`, `Get-DisplayWidth`, `Limit-DisplayWidth`, `ConvertTo-NFC`, `Get-BidiClasses`, `Get-LineBreakOpportunities`, and more.

---

### 6. Quick start examples

Short code snippets (2-3 per language) showing real-world use:

**Rust**
```rust
use uniworld::{grapheme_boundaries, display_width, normalize_nfc};

let text = "cafe\u{0301}";          // decomposed e-acute
let nfc = normalize_nfc(text);       // "cafe" (composed)
let clusters = grapheme_boundaries(text);  // ["c", "a", "f", "e\u{0301}"]
let width = display_width("Hello");  // 10 (CJK = 2 each)
```

**Python**
```python
import uniworld

print(uniworld.grapheme_boundaries("Hello"))  # ['H', 'e', 'l', 'l', 'o', ' ', ...]
print(uniworld.display_width("Hello"))        # 10
print(uniworld.normalize_nfc("cafe\u0301"))   # "cafe"
```

**PowerShell**
```powershell
Import-Module UniWorld
"Hello" | Get-DisplayWidth      # 10
"cafe`u{0301}" | ConvertTo-NFC  # "cafe"
```

---

### 7. Clone and build from source

```bash
git clone https://github.com/aguywithai/uniworld.git
cd uniworld
cargo build
cargo test
```

#### Building bindings

- **Python**: `pip install maturin && maturin develop --features python`
- **WASM**: `wasm-pack build --features wasm`
- **C header**: `cbindgen --config cbindgen.toml --output uniworld.h` (after `cargo build --features cffi`)
- **Go**: `cd bindings/go && go test` (requires C FFI library built first)

#### Building extensions

- **VS Code**: `cd extensions/vscode && npm install && npm run compile` (then F5 to test)
- **PowerShell**: `Import-Module extensions/powershell/UniWorld.psd1` (requires native library in `target/release/`)

---

### 8. Scripts covered

A visual or list section showing the breadth:

Latin, Greek, Cyrillic, Arabic, Hebrew, Devanagari, Bengali, Gurmukhi, Tamil, Sinhala, Thai, Lao, Khmer, Myanmar, Chinese (Simplified/Traditional), Japanese (Kanji + Hiragana + Katakana), Korean (Hangul), Ethiopic, Tifinagh, Cherokee, Canadian Aboriginal Syllabics (Cree, Inuktitut, Ojibwe), Emoji (including ZWJ sequences, skin tones, flags).

Link to the [Unicode Showcase](https://github.com/aguywithai/uniworld/blob/main/docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md) for the full stress-test document.

---

### 8.5. How UniWorld Was Built (HAIMU story block)

Brief, compelling section on the main page -- drives traffic to haimu.world.

**Heading**: "Built in 14 hours. From idea to library."

**Content** (tight, 3-4 sentences):

UniWorld was conceived and largely built within 14 hours using [HAIMU](https://haimu.world), the AI development methodology originated by developer Sean MacNutt. When HAIMU was used to identify the largest-ROI neglected technical benefit projects an AI could conceive of in the current landscape, correct Unicode text handling emerged as the clear winner. The library, five language bindings, a VS Code extension, and a PowerShell module followed. MacNutt's principle: "Move fast and fix things."

**CTA**: Learn more about HAIMU at [haimu.world](https://haimu.world). Deeper development notes at [aguywithai.world/projects/uniworld](https://aguywithai.world/projects/uniworld).

---

### 9. Footer

- **GitHub**: https://github.com/aguywithai/uniworld
- **License**: MIT
- **By**: Sean MacNutt / [A Guy With AI](https://aguywithai.world)
- **Methodology**: Developed using [HAIMU](https://haimu.world), Sean MacNutt's AI development methodology. A Grand Beta ([grandbeta.world](https://grandbeta.world)) funded project.
- **All links**: [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld) | [PowerShell Gallery](https://www.powershellgallery.com/packages/UniWorld) | [crates.io](https://crates.io/crates/uniworld) | [PyPI](https://pypi.org/project/uniworld/) | [npm](https://www.npmjs.com/package/uniworld)
- **Docs**: [Integration guides](docs/integration/) | [Unicode Showcase](docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md)
- **All projects**: [worldof.world](https://worldof.world)

---

## Design notes

- **Minimal and clean**: The current `index.html` + `style.css` uses a centered layout, system fonts, dark borders, hover transitions. This is a good starting point; the sections above would expand it into a multi-section single-page site.
- **No framework required**: Plain HTML/CSS/JS is enough. If you want a build step later (e.g. for syntax highlighting in code blocks), a lightweight tool like Eleventy or Hugo can generate from markdown.
- **Responsive**: The CSS already uses `max-width` and `padding`; add media queries if needed for mobile.
- **Registry links**: Uncomment and fill in registry links in `index.html` once packages are published.
- **Business link**: grandbeta.world (Grand Beta)
- **Cross-links**: Every section should link out to relevant resources (GitHub, registries, integration guides, extension/module READMEs, showcase). The site is a hub; the READMEs link back to it. This mutual cross-linking is deliberate and mirrors the README structure.
