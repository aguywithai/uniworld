# UniWorld Outreach, Listings, and Press

Where to list, contact, submit, and announce UniWorld across the Unicode ecosystem,
developer communities, and broader press. Includes compliance notes and submission details.

---

## Part 1: Unicode Ecosystem Listings

### Unicode Consortium

**What**: The Unicode Consortium maintains a list of implementations and resources.
They are the standards body behind UAX #9, #14, #15, #29 and all Unicode data.

- **ICU Conformance**: UniWorld implements the same algorithms as ICU but independently.
  Mentioning conformance test pass rates (770K+ bidi, 19K+ line break, etc.) is relevant.
- **Contact**: https://www.unicode.org/reporting.html (for reporting conformance or
  requesting listing)
- **CLDR Survey Tool**: Not directly applicable (UniWorld doesn't implement CLDR locale data)
  but if dictionary-based line breaking expands, CLDR may become relevant.
- **Action**: After public release, email unicode-org to request listing on
  https://www.unicode.org/resources/libraries.html or equivalent. Include:
  - Library name, URL, license (MIT)
  - Algorithms implemented: UAX #9 (Bidi), UAX #14 (Line Breaking), UAX #15 (Normalization),
    UAX #29 (Segmentation: Grapheme, Word, Sentence), East Asian Width, Case Mapping
  - Conformance test results with counts
  - Language bindings available
- **Compliance**: UniWorld targets full conformance with the Unicode Standard.
  Current conformance: Unicode 17.0 / UCD 17.0.0 (verify version before submission).
  No known non-conformances as of development completion.

### Unicode Mailing Lists

- **unicode@unicode.org** (public discussion list): Appropriate for announcing
  a new conformant implementation. Keep announcement brief and factual.
- **Sign up**: https://www.unicode.org/consortium/distlist.html

### CLDR / ICU Project

- **ICU**: https://icu.unicode.org/ -- UniWorld uses ICU dictionary data for
  Thai/Lao/Khmer/Myanmar line breaking. Acknowledge this in announcements.
- **Action**: No submission needed, but if UniWorld finds bugs in ICU dictionaries,
  report to ICU issue tracker.

---

## Part 2: Package Registry Listings

### crates.io (Rust)

- **URL**: https://crates.io/
- **Signup**: Login with GitHub (aguywithai)
- **Submission**: `cargo publish` (see ACCOUNTS_AND_TOKENS.md)
- **Listing details**: Description, keywords (unicode, text-segmentation, bidi,
  line-breaking, normalization, grapheme, east-asian-width), categories
  (text-processing, internationalization), README auto-rendered from repo
- **Compliance**: crates.io requires MIT/Apache-2.0 license (we have MIT -- compliant)

### PyPI (Python)

- **URL**: https://pypi.org/
- **Signup**: Create account, enable 2FA
- **Submission**: `maturin publish` (PyO3 binding)
- **Listing details**: Description, classifiers (Development Status, License,
  Programming Language, Topic :: Text Processing :: Linguistic), README
- **Compliance**: PyPI requires a license classifier and a license file -- compliant

### npm (JavaScript/WASM)

- **URL**: https://www.npmjs.com/
- **Signup**: Create account, enable 2FA
- **Submission**: `npm publish`
- **Listing details**: package.json keywords, README
- **Compliance**: npm requires valid package.json -- compliant

### VS Code Marketplace

- **URL**: https://marketplace.visualstudio.com/
- **Signup**: Azure DevOps PAT (see ACCOUNTS_AND_TOKENS.md)
- **Submission**: `vsce publish` (publisher: aguywithai)
- **Listing details**: package.json metadata, icon, README, CHANGELOG, categories
  (Other, Programming Languages), keywords
- **Compliance**: Marketplace requires publisher account, icon (256x256+), README,
  LICENSE, valid package.json -- all compliant
- **Note**: Can publish as UNLISTED for pre-release testing (set via Marketplace dashboard
  after initial publish)

### PowerShell Gallery

- **URL**: https://www.powershellgallery.com/
- **Signup**: Microsoft account, NuGet API key
- **Submission**: `Publish-Module` (see ACCOUNTS_AND_TOKENS.md)
- **Listing details**: .psd1 manifest (tags, description, project URI, license URI)
- **Compliance**: Gallery requires .psd1 with GUID, version, description, license URI,
  author -- all compliant
- **Note**: Pre-release versions can be published with `-Prerelease` suffix in version

---

## Part 3: Developer Community Listings and Announcements

### GitHub

- **Awesome lists**: Submit PR to relevant awesome-* lists after public release:
  - awesome-rust (https://github.com/rust-unofficial/awesome-rust) -- Text Processing section
  - awesome-unicode (if exists)
  - awesome-vscode (https://github.com/viatsko/awesome-vscode) -- Language/Text section
- **GitHub Topics**: Tag repo with topics: unicode, text-segmentation, bidi,
  line-breaking, normalization, grapheme-cluster, rust, powershell, vscode-extension
- **GitHub Discussions/Releases**: Create a detailed release post for v0.1.0

### Reddit

- **r/rust**: Appropriate for announcing a new Rust crate with real conformance.
  "Show r/rust" style post. Emphasize: full UAX conformance, 870K+ test cases,
  five binding languages, dictionary line breaking.
- **r/programming**: Broader reach. Angle: "Why Unicode text handling is still broken
  in most tools, and what UniWorld does about it."
- **r/PowerShell**: Novel angle -- first Unicode-aware text toolkit for PowerShell.
  Demonstrate CJK width, grapheme boundaries, bidi analysis in terminal.
- **r/vscode**: Extension announcement. Angle: "Finally, correct emoji/CJK/RTL
  handling in VS Code."
- **Sign up**: reddit.com (if not already have account)

### Hacker News

- **URL**: https://news.ycombinator.com/
- **Post type**: "Show HN: UniWorld -- correct Unicode for every language"
- **Angle**: Technical depth. HN audience appreciates conformance numbers, algorithm
  correctness, cross-platform ambition. Lead with the problem (Unicode handling is
  broken everywhere) and the numbers (870K+ conformance tests, 5 UAX standards,
  6 language bindings).
- **Timing**: Post early morning US time (9-10am ET) for best visibility
- **Sign up**: Create account if needed; new accounts can post but may have rate limits

### Dev.to / Hashnode / Medium

- **Dev.to**: Technical blog post. "Building a conformant Unicode library in Rust
  (and what I learned about the Bidi Algorithm)"
- **Hashnode**: Similar. Good for SEO.
- **Medium**: Broader audience. "The Unicode Problems You Don't Know You Have"
- **Action**: Write 1-2 blog posts timed with public launch

### Twitter/X and Mastodon

- **Angle**: Thread format. "I built a Unicode library that passes 870,000+ conformance
  tests. Here's why that matters for your emoji, your Arabic text, and your terminal."
- **Tag**: @unicode (Unicode Consortium's account), relevant Rust/VS Code accounts
- **Mastodon**: fosstodon.org is the main FOSS instance; good for open-source reach

### Stack Overflow

- **Not for announcements**. But: answer Unicode-related questions with references to
  UniWorld where genuinely helpful. Common question categories:
  - "How to get string display width in terminal" (CJK)
  - "How to iterate grapheme clusters in [language]"
  - "PowerShell string length wrong for emoji/CJK"
  - "VS Code cursor behavior with emoji/RTL"
- **Action**: Monitor these tags after release; provide helpful answers with UniWorld
  as one option among others

---

## Part 4: Press Releases and Broader Announcements

### When to Issue

- **After public release** and all registries are live
- **After patent filings are secured** (if coordinating with broader portfolio)
- Consider two waves:
  1. **Technical announcement**: Registry listings, GitHub public, developer communities
  2. **Broader narrative**: Ties to Grand Beta, AI methodology, investment thesis

### Press Release Angle: Technical

**Headline**: "UniWorld: First open-source library to implement five Unicode algorithms
with full conformance across six programming languages"

**Key claims** (all verifiable):
- 870,000+ conformance test cases passing across UAX #9, #14, #15, #29
- Dictionary-based line breaking for Thai, Lao, Khmer, Myanmar (rare in non-ICU implementations)
- Single Rust core with bindings for Python, JavaScript/WASM, C, Go, plus
  VS Code extension and PowerShell module
- MIT licensed, free for all use

**Supporting numbers**:
- 770,241 bidi test cases (UAX #9)
- 91,707 bidi character test cases
- 19,338 line break test cases (UAX #14)
- 1,944 word segmentation test cases (UAX #29)
- 766 grapheme segmentation test cases (UAX #29)
- 512 sentence segmentation test cases (UAX #29)

**Distribution**: Tech press (if warranted), developer community channels (above),
Unicode Consortium mailing list

### Press Release Angle: Broader (Grand Beta / Investment Context)

**Headline**: "Developer demonstrates HAIMU methodology with production-grade
Unicode library built in under a week"

**Angle**: The story isn't just the library -- it's the methodology. Sean MacNutt,
using HAIMU (Human-AI Mutual Understandability) -- a structured AI-assisted
development methodology -- produced a conformance-tested, multi-language,
multi-platform Unicode toolkit in 4 days. A Grand Beta funded Cursor subscription
provided the AI development environment; the methodology and technical direction
are MacNutt's. The same methodology has been applied to additional domains
[fusion/protein/networking -- mention after patents secured].

**Links**: [haimu.world](https://haimu.world), [aguywithai.world](https://aguywithai.world),
[A Guy With AI podcast](https://aguywithai.world)

**Caution**: This angle should wait until patents are filed and you're ready to
discuss the broader portfolio publicly.

### Wire Services (if appropriate scale)

- **Canada NewsWire** (CNW): For Canadian companies; reaches Canadian media
- **PR Newswire**: Broader North American reach
- **Cost**: $400-800 per release depending on distribution
- **When**: Only if tying to investment announcement or significant milestone
- **For UniWorld alone**: Developer community channels are more appropriate and free

---

## Part 5: Compliance and Licensing Summary

| Aspect | Status |
|--------|--------|
| MIT License | Compliant (LICENSE file in root and extensions) |
| Unicode License | Data files derived from Unicode standard; Unicode License applies to data files (permissive, compatible with MIT) |
| ICU License | Dictionary data from ICU; ICU license is Unicode License (permissive) |
| crates.io requirements | MIT license present, valid Cargo.toml |
| PyPI requirements | License classifier, license file, valid pyproject.toml |
| npm requirements | Valid package.json with license field |
| VS Code Marketplace | Publisher, icon, README, LICENSE, CHANGELOG |
| PowerShell Gallery | .psd1 manifest with all required fields |
| Unicode Consortium listing | No formal approval needed; self-reported conformance |

---

## Part 6: Submission Checklist

Use this when you're ready to go public:

- [ ] Email Unicode Consortium requesting library listing
- [ ] Subscribe to unicode@unicode.org mailing list and post announcement
- [ ] Publish to crates.io
- [ ] Publish to PyPI
- [ ] Publish to npm
- [ ] Publish VS Code extension to Marketplace
- [ ] Publish PowerShell module to Gallery
- [ ] Submit PR to awesome-rust
- [ ] Submit PR to awesome-vscode
- [ ] Tag GitHub repo with relevant topics
- [ ] Create GitHub Release with release notes
- [ ] Post to r/rust
- [ ] Post to r/programming
- [ ] Post to r/PowerShell
- [ ] Post to r/vscode
- [ ] Post to Hacker News (Show HN)
- [ ] Publish blog post (Dev.to or preferred platform)
- [ ] Post announcement thread on Twitter/X
- [ ] Post on Mastodon (fosstodon.org)
- [ ] Notify A Guy With AI channels (podcast, aguywithai.world)
- [ ] Cross-post to worldof.world project listing
- [ ] Monitor Stack Overflow for relevant questions
