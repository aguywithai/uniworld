# UniWorld - Unicode Text Tools for VS Code

Correct Unicode text handling in your editor. Grapheme-aware cursor and delete, bidi visualization, display width, line break decorations, normalization, and Unicode inspection -- all powered by a conformance-tested Rust/WASM core.

**UniWorld is more than an extension.** It is a complete Unicode text-handling ecosystem: a [Rust library](https://github.com/aguywithai/uniworld) with bindings for Python, JavaScript/WASM, C, and Go, a [PowerShell module](https://www.powershellgallery.com/packages/UniWorld), and this VS Code extension. Visit **[uniworld.world](https://uniworld.world)** for the full project, documentation, integration guides, and other tools.

## The problems this extension solves

These are problems every VS Code user encounters, whether working in English, Arabic, Chinese, or any other language:

- **Emoji splitting**: Your cursor lands inside ZWJ sequences (family, skin-tone, flag emoji), splitting what should be a single character. Backspace removes only part of an emoji. *UniWorld treats each emoji as one grapheme cluster.*
- **Combining mark orphaning**: Backspace after an accented character (French, German, Vietnamese, or any language with diacritics) removes just the accent and leaves the base character. *UniWorld deletes the full cluster: base + marks together.*
- **Indic conjunct breaking**: Cursor movement in Devanagari, Bengali, Tamil, and other Indic scripts lands inside ligatures, breaking visual characters. *UniWorld respects conjunct boundaries.*
- **CJK/emoji column miscount**: The status bar reports wrong column numbers for lines containing CJK ideographs or emoji, because it counts characters instead of display columns. *UniWorld shows true column width (CJK=2, emoji=2).*
- **No normalization tools**: Pasted text may mix NFC and NFD invisibly, causing string comparisons to fail silently. *UniWorld provides NFC/NFD/NFKC/NFKD normalization commands.*
- **No Unicode inspection**: When you encounter an unexpected character, there's no built-in way to see what codepoints you're looking at. *UniWorld shows codepoint, category, and display width on hover.*
- **RTL cursor confusion**: In Arabic and Hebrew text, the Left arrow key moves the cursor visually to the right. *UniWorld's visual bidi cursor makes Left go left on screen.*
- **Thai/Lao/Khmer/Myanmar line wrapping**: These scripts have no spaces between words, so the editor wraps at arbitrary positions. *UniWorld provides dictionary-based line break decorations.*

## Features

### On by default

| Feature | Description |
|---------|-------------|
| **Grapheme-aware cursor** | Left/Right arrow keys skip over entire grapheme clusters (emoji, Indic conjuncts, combining marks). |
| **Grapheme-aware delete** | Backspace/Delete remove an entire grapheme cluster in one keypress. |
| **Unicode hover inspector** | Hover over any character to see its codepoints, grapheme cluster boundaries, and display width. |
| **Display width in status bar** | Shows true display columns and grapheme cluster count for the current line or selection. |

### Opt-in (toggle in Settings)

| Feature | Setting | Description |
|---------|---------|-------------|
| **Visual bidi cursor** | `uniworld.enableBidiVisualCursor` | Left/Right follow visual direction in Arabic/Hebrew text. |
| **Script-aware word selection** | `uniworld.enableGraphemeWordSelect` | Double-click and Ctrl+D use script-specific word boundaries. |
| **Line break decorations** | `uniworld.showLineBreakOpportunities` | Subtle markers at UAX #14 line-break positions, including dictionary-based Thai/Lao/Khmer/Myanmar. |
| **Bidi run highlighting** | `uniworld.showBidiVisualization` | LTR runs highlighted blue, RTL runs orange, so you can see bidi structure at a glance. |

### Commands (Command Palette)

| Command | Description |
|---------|-------------|
| `UniWorld: Inspect Selection` | Full breakdown: codepoints, grapheme clusters, word and sentence boundaries. |
| `UniWorld: Truncate to Display Width` | Truncate selection to N display columns without breaking clusters. |
| `UniWorld: Normalize NFC` | Normalize selection to NFC (canonical composition). |
| `UniWorld: Normalize NFD` | Normalize selection to NFD (canonical decomposition). |
| `UniWorld: Normalize NFKC` | Normalize selection to NFKC (compatibility composition). |
| `UniWorld: Normalize NFKD` | Normalize selection to NFKD (compatibility decomposition). |
| `UniWorld: Select Word at Cursor` | Script-aware word selection. |
| `UniWorld: Toggle Line Break Opportunity Decorations` | Toggle line-break markers on/off. |

## Settings

All settings are under **UniWorld** in VS Code Settings (`Ctrl+,`).

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `uniworld.enableGraphemeCursor` | boolean | `true` | Left/Right arrow keys use grapheme-aware movement. |
| `uniworld.enableGraphemeDelete` | boolean | `true` | Backspace/Delete use grapheme-aware deletion. |
| `uniworld.enableHoverInspector` | boolean | `true` | Show Unicode codepoint and width info when hovering over text. |
| `uniworld.enableBidiVisualCursor` | boolean | `false` | Left/Right follow visual direction in RTL text. |
| `uniworld.enableGraphemeWordSelect` | boolean | `false` | Double-click and Ctrl+D use UniWorld word boundaries. |
| `uniworld.showLineBreakOpportunities` | boolean | `false` | Show line-break opportunity decorations. |
| `uniworld.showBidiVisualization` | boolean | `false` | Highlight LTR and RTL bidi runs in the editor. |

## Architecture

- TypeScript extension running in the VS Code extension host
- Loads the UniWorld WASM module (built from the same Rust core as the library)
- WASM is loaded lazily on activation; falls back gracefully if unavailable
- No native dependencies; cross-platform via WASM
- Unicode 17.0 conformant: passes UAX #29 (grapheme/word/sentence), UAX #14 (line break), UAX #9 (bidi), UAX #15 (normalization) conformance test suites (UCD 17.0.0)

## Development

From the repo root, build the WASM module:

```powershell
# Build WASM
wasm-pack build --target nodejs --features wasm --no-default-features

# Copy to extension
Copy-Item pkg\uniworld.js extensions\vscode\wasm\
Copy-Item pkg\uniworld.d.ts extensions\vscode\wasm\
Copy-Item pkg\uniworld_bg.wasm extensions\vscode\wasm\
Copy-Item pkg\uniworld_bg.wasm.d.ts extensions\vscode\wasm\
```

Then build and test the extension:

```powershell
cd extensions/vscode
npm install
npm run compile
```

Press F5 in VS Code to launch the Extension Development Host for testing.

## Related

| Resource | Link |
|----------|------|
| **UniWorld website** | [uniworld.world](https://uniworld.world) |
| **UniWorld library** (Rust core) | [GitHub](https://github.com/aguywithai/uniworld) |
| **PowerShell module** | [PowerShell Gallery](https://www.powershellgallery.com/packages/UniWorld) / [README](../powershell/README.md) |
| **Python integration** | [Integration guide](../../docs/integration/python.md) |
| **JavaScript/WASM integration** | [Integration guide](../../docs/integration/javascript-wasm.md) |
| **C integration** | [Integration guide](../../docs/integration/c.md) |
| **Go integration** | [Integration guide](../../docs/integration/go.md) |
| **Unicode Showcase** | [Stress-test document](../../docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md) |
| **A Guy With AI** (publisher) | [aguywithai.world](https://aguywithai.world) |
| **HAIMU AI development methodology** | [haimu.world](https://haimu.world) |
| **Grand Beta** (funding) | [grandbeta.world](https://grandbeta.world) |

## About

UniWorld is an [A Guy With AI](https://aguywithai.world) project by Sean MacNutt. Built using [HAIMU](https://haimu.world), MacNutt's AI development methodology -- HAIMU generated the insight leading to UniWorld when prompting for the highest-impact neglected technical projects, and the library was largely built within 14 hours of idea generation. "Move fast and fix things." Development funded by [Grand Beta](https://grandbeta.world).

## License

MIT. See [LICENSE](./LICENSE) for details.

Unicode Character Database data is used under the [Unicode License](https://www.unicode.org/license.txt). ICU dictionary data is used under the [ICU License](https://github.com/nickel-org/nickel.rs).
