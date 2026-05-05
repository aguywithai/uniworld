# UniWorld PowerShell Module

Correct Unicode text handling in PowerShell. Grapheme boundaries, display width, normalization, bidi analysis, line breaking, and more -- 12 cmdlets backed by a conformance-tested Rust core.

**UniWorld is more than a PowerShell module.** It is a complete Unicode text-handling ecosystem: a [Rust library](https://github.com/aguywithai/uniworld) with bindings for Python, JavaScript/WASM, C, and Go, a [VS Code extension](https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld), and this module. Visit the **[project website](https://aguywithai.world/opensource/uniworld/)** for the full project, documentation, integration guides, and other tools.

## The problems this module solves

These are real problems in everyday PowerShell work, in any language:

- **Grapheme clusters**: `"cafe" + combining accent` is one visual character but `.Length` says two. Log parsing, CSV processing, and string formatting all get this wrong. *UniWorld segments by grapheme cluster boundaries, not code points.*
- **Display width**: CJK ideographs and emoji take 2 terminal columns, but PowerShell's string length counts them as 1 (or 2 code units for surrogates). Your `Format-Table` columns don't line up. *UniWorld gives you true display width.*
- **Safe truncation**: Cutting a string at a byte or character offset can split emoji, accents, or CJK characters, producing garbled output. *UniWorld truncates to a display-width limit without breaking grapheme clusters.*
- **Normalization**: Text pasted from different sources may use different normalization forms. Two strings that look identical fail `-eq` because one is NFC and the other NFD. *UniWorld normalizes to any of the four standard forms.*
- **Bidirectional text**: Arabic, Hebrew, and mixed-direction text need bidi level analysis for correct processing. *UniWorld provides per-character embedding levels and direction.*
- **Line breaking**: Thai, Lao, Khmer, and Myanmar have no spaces between words. PowerShell (and most terminal tools) wrap these scripts at arbitrary positions. *UniWorld includes dictionary-based word segmentation (179,081-word dictionary from ICU).*

## Installation

### From PowerShell Gallery (when published)

```powershell
Install-Module -Name UniWorld -Scope CurrentUser
```

### From source

```powershell
# Build the native library (requires Rust toolchain)
cargo build --release --features cffi

# Import the module
Import-Module ./extensions/powershell/UniWorld.psd1
```

The module looks for the native library in `native/<rid>/` (CI artifacts), then `native/`, then `../../target/release/`.

## Publishing to PowerShell Gallery (maintainers)

`Publish-Module -Path .` from this folder fails because the directory name must match the module (`UniWorld.psd1` expects a parent folder named `UniWorld`). Use:

```powershell
# Windows PowerShell 5.1: if you see "Could not create SSL/TLS secure channel", the script
# sets TLS 1.2 before upload. You can also run once per session:
# [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

# One-time if Publish-Module asks for NuGet:
# Install-PackageProvider -Name NuGet -MinimumVersion 2.8.5.201 -Force

cd extensions/powershell
$env:PSGALLERY_API_KEY = '<your API key>'
.\Publish-ToGallery.ps1
```

## Cmdlets

### Text Segmentation ([UAX #29](https://unicode.org/reports/tr29/))

| Cmdlet | Description |
|--------|-------------|
| `Get-GraphemeBoundaries` | Segment text into grapheme clusters |
| `Get-WordBoundaries` | Segment text into words |
| `Get-SentenceBoundaries` | Segment text into sentences |

```powershell
# Grapheme-aware: emoji ZWJ sequence is ONE cluster
Get-GraphemeBoundaries -InputObject "family emoji"

# Word segmentation with full Unicode rules
"Hello, World!" | Get-WordBoundaries
```

### Display Width ([East Asian Width](https://unicode.org/reports/tr11/))

| Cmdlet | Description |
|--------|-------------|
| `Get-DisplayWidth` | True terminal column count (CJK=2, emoji=2, combining=0) |
| `Limit-DisplayWidth` | Truncate to N columns without breaking graphemes |

```powershell
# CJK ideographs are width 2 each
Get-DisplayWidth -InputObject "`u{4E16}`u{754C}"    # 4

# Safe truncation: never splits an emoji or accent
"Hello World" | Limit-DisplayWidth -MaxWidth 7
```

### Normalization ([UAX #15](https://unicode.org/reports/tr15/))

| Cmdlet | Description |
|--------|-------------|
| `ConvertTo-NFC` | Canonical composition (recommended for interchange) |
| `ConvertTo-NFD` | Canonical decomposition |
| `ConvertTo-NFKC` | Compatibility composition (search, identifiers) |
| `ConvertTo-NFKD` | Compatibility decomposition |

```powershell
# Compose decomposed text
"cafe`u{0301}" | ConvertTo-NFC    # precomposed e-acute

# Collapse compatibility variants
"`u{FB01}" | ConvertTo-NFKC       # fi ligature -> "fi"
```

### Bidi Analysis ([UAX #9](https://unicode.org/reports/tr9/))

| Cmdlet | Description |
|--------|-------------|
| `Get-BidiClasses` | Per-character embedding level and direction (LTR/RTL) |

```powershell
Get-BidiClasses -InputObject "Hello" | Format-Table
# Character  CodePoint  BidiLevel  Direction
# ---------  ---------  ---------  ---------
# H          U+0048     0          LTR
# ...
```

### Line Breaking ([UAX #14](https://unicode.org/reports/tr14/))

| Cmdlet | Description |
|--------|-------------|
| `Get-LineBreakOpportunities` | Break positions with Mandatory/Allowed action |

```powershell
Get-LineBreakOpportunities "Hello World" | Format-Table
# Includes dictionary-based breaking for Thai, Lao, Khmer, Myanmar
```

### Inspection

| Cmdlet | Description |
|--------|-------------|
| `Get-UnicodeInfo` | Codepoint, category, display width per text element |

```powershell
"A" | Get-UnicodeInfo
# Character: A, CodePoint: U+0041, Category: UppercaseLetter, DisplayWidth: 1

# Inspect an entire string
"Hello" | Get-UnicodeInfo | Format-Table
```

## Pipeline Support

All cmdlets accept pipeline input via `-InputObject`:

```powershell
"Hello" | Get-GraphemeBoundaries
"text" | Get-DisplayWidth
"cafe`u{0301}" | ConvertTo-NFC | Get-DisplayWidth
Get-Content file.txt | Get-WordBoundaries
```

## Compatibility

- **Windows PowerShell 5.1** and **PowerShell 7+** (cross-platform)
- Native library required: `uniworld.dll` (Windows), `libuniworld.so` (Linux), `libuniworld.dylib` (macOS)
- Falls back to .NET built-in normalization if the native library is unavailable (NFC/NFD/NFKC/NFKD only)
- All 12 cmdlets tested with [Pester 5](https://pester.dev/) (68 tests covering grapheme clusters, emoji, CJK width, combining marks, normalization, bidi, line breaks, pipeline integration)

## Testing

```powershell
# Requires Pester 5+
Install-Module -Name Pester -MinimumVersion 5.0 -Force -Scope CurrentUser
Invoke-Pester -Path extensions/powershell/Tests/ -Output Detailed
```

## Architecture

```
UniWorld Rust core
    |
    v
cdylib (uniworld.dll / libuniworld.so / libuniworld.dylib)
    |
    v
C# P/Invoke interop (inline via Add-Type)
    |
    v
PowerShell cmdlets (UniWorld.psm1)
```

The same Rust code that passes 770,000+ Unicode conformance tests runs beneath these cmdlets. The behavior is identical to the library, the VS Code extension, and every other UniWorld binding.

## Related

| Resource | Link |
|----------|------|
| **UniWorld website** | [aguywithai.world/opensource/uniworld](https://aguywithai.world/opensource/uniworld/) |
| **UniWorld library** (Rust core) | [GitHub](https://github.com/aguywithai/uniworld) |
| **VS Code extension** | [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld) / [README](../vscode/README.md) |
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

MIT. See [LICENSE](../../LICENSE).
