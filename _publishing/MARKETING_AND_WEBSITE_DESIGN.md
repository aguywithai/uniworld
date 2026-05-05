# UniWorld: Marketing Strategy and Website Design Brief

**Date**: 2026-02-10
**For**: uniworld.world (GitHub Pages) and general project marketing
**Publisher**: [A Guy With AI](https://aguywithai.world) (Sean MacNutt)
**Methodology**: [HAIMU](https://haimu.world) (Human-AI Mutual Understandability)
**Funding**: [Grand Beta](https://grandbeta.world)

---

## Part 1: Marketing -- What to Say and Why

### The Core Message

**UniWorld is the Unicode implementation that should have existed already.**

Correct text handling is one of the most pervasive unsolved problems in everyday software. It is not a niche concern for non-English speakers -- it affects everyone who uses emoji, accented characters, column-aligned output, or pasted text from different sources. Every operating system, every editor, every terminal has its own partial, inconsistent interpretation of Unicode. This has been neglected for too long.

UniWorld replaces that patchwork with a single, spec-conformant core that any tool can use. One library. Every script. Every algorithm. Tested against every official conformance suite.

### Who This Is For

UniWorld serves four audiences. Marketing should speak to all of them, in this priority order. **Critically, each audience includes English-speaking and Latin-script users, not only multilingual users.** The "Latin world practical use" angle should be front and center: emoji, combining accents, display width, normalization, and safe truncation are daily problems for developers working exclusively in English.

1. **Developers building text-handling software** (library users: Rust, Python, JS, C, Go). These people have hit the wall -- emoji splitting in English chat apps, normalization mismatches in English databases, column miscounts in English terminal output, and truncation bugs in English APIs. They may also face bidi rendering bugs, Thai text that won't wrap, Indic conjunct splitting. They're searching for "unicode grapheme rust" or "string display width python" or "emoji cursor fix." UniWorld is the answer they're looking for and probably didn't expect to find as a single, complete package.

2. **VS Code users** (extension users). They may not know they have Unicode problems until the extension fixes them. The hook for English-speaking users: "your cursor splits emoji, your backspace orphans accents in French and German text, and your column count is wrong for any line with CJK or emoji." The hook for multilingual users: "your editor breaks Arabic text, Indic scripts, and Thai word boundaries." Once installed, it works invisibly. The value is immediate.

3. **PowerShell and terminal users** (module users). System administrators and DevOps engineers working with log files, CSV with CJK, text processing pipelines, or any data that passes through multilingual systems. Even English-only workflows encounter emoji in logs, fullwidth characters in imports, and normalization mismatches in user input. "Get the display width right. Truncate without breaking characters. Normalize consistently."

4. **The Unicode-curious** (educators, linguists, type designers). People who want to understand how text actually works. The Unicode showcase, the inspector, the bidi visualization -- these are educational tools as much as practical ones.

### Key Claims (All Substantiated)

These are the points to lead with. Every one is verifiable:

- **770,241 bidirectional algorithm test cases passing** (BidiTest.txt + BidiCharacterTest.txt, Unicode 17.0).
- **19,338 line break test cases passing** (LineBreakTest.txt, UAX #14 revision 55).
- **3,222 segmentation test cases passing** (grapheme: 766, word: 1,944, sentence: 512).
- **Full normalization conformance** (NormalizationTest.txt, all five parts).
- **Unicode 17.0** throughout -- not an older version, not a subset.
- **One core, five languages**: Rust, Python, JS/WASM, C, Go.
- **Two developer tools**: VS Code extension, PowerShell module.
- **179,081-word dictionary** for Thai/Lao/Khmer/Myanmar segmentation (from ICU).
- **MIT licensed**: no restrictions on commercial use.

### The Value Proposition in Plain Language

**For the library page / hero section:**

> Software gets text wrong -- in every language, including yours.
>
> If you work in English, your cursor splits emoji, your backspace orphans accents, and your column counts break on CJK or fullwidth characters. Strings that look identical fail comparison because of invisible normalization differences. Your truncation logic cuts in the middle of grapheme clusters.
>
> If you work with Arabic, Hebrew, Thai, Hindi, Chinese, Japanese, Korean, or any of dozens of other scripts, the problems multiply. Bidi layout is broken. Line breaking is wrong. Conjuncts split. Word boundaries are guessed.
>
> UniWorld implements the Unicode standard's core algorithms completely and correctly -- bidirectional layout, line breaking, text segmentation, normalization, display width, case mapping, and cursor navigation -- in a tested Rust core with bindings for Python, JavaScript, C, and Go, plus a VS Code extension and PowerShell module.
>
> One library. Every script. Every algorithm. No more guessing.

**For the VS Code extension page:**

> Your editor doesn't understand Unicode. UniWorld fixes that.
>
> Grapheme-aware cursor and delete that never split emoji, Indic conjuncts, or combining marks. Visual bidi cursor for Arabic and Hebrew. Dictionary-based line breaking for Thai, Lao, Khmer, and Myanmar. True display width. Unicode inspection on hover. All powered by a Rust/WASM core that passes every official conformance test.

**For the PowerShell module page:**

> Correct Unicode text processing in your terminal.
>
> Get true display widths. Segment text by grapheme, word, or sentence. Normalize to NFC/NFD/NFKC/NFKD. Truncate strings without breaking characters. Analyse bidi structure. All from PowerShell cmdlets backed by a conformance-tested Rust core.

### Differentiation

What makes UniWorld different from other Unicode libraries:

1. **Completeness**: Most libraries implement one or two algorithms. UniWorld implements all the core ones (UAX #9, #14, #15, #29) plus composite operations (cursor, width, truncation, case mapping).

2. **Conformance**: Most libraries don't publish their test results. UniWorld passes every official Unicode conformance test suite, and publishes the numbers.

3. **Breadth of surface**: One Rust core powering five language bindings plus a VS Code extension plus a PowerShell module. The behaviour is identical everywhere because it's the same code.

4. **Dictionary segmentation**: Thai, Lao, Khmer, and Myanmar dictionary-based word breaking is included, not an afterthought. 179,081 words from ICU dictionaries.

5. **Developer tools**: The VS Code extension and PowerShell module aren't wrappers around a different engine -- they use the exact same core. If it passes conformance in Rust, it passes conformance in VS Code.

---

## Part 2: Website Design Brief (uniworld.world / GitHub Pages)

### Design Philosophy

The site should feel like the project: **precise, confident, and quietly authoritative**. Not flashy. Not a startup landing page with gradient backgrounds and floating illustrations. Clean typography, generous whitespace, and content that respects the reader's time. The existing `style.css` is a good foundation -- system fonts, centered layout, dark borders, hover transitions. Build on that, don't replace it.

### Visual Identity

- **Colour palette**: Dark navy (`#1a1a2e`) as accent/hero background, white/off-white (`#fafafa`) for content sections, muted blue (`#4a96ff`) for links and highlights, warm orange (`#ffa03c`) as a secondary accent (echoing the RTL highlight colour from the bidi visualization). These colours connect the site to the extension's visual language.

- **Typography**: System font stack (Segoe UI, system-ui, -apple-system, sans-serif). The project deals with every script; the site should use a font stack that renders as many of them as possible natively. No custom web fonts needed -- the system stack handles Latin, Arabic, CJK, Indic, Thai on every major platform.

- **Icon**: The tech-globe with orbiting scripts (icon2.png / icon.png). Use as the hero image and favicon. The orbiting scripts (Latin, Arabic, CJK, Devanagari) visually communicate "every script" without needing to say it.

- **Code blocks**: Dark background (`#1e1e2e`), light text, subtle border. Use a monospace font stack. Syntax highlighting optional but nice; if added, use a muted palette (not aggressive neon).

### Page Structure (Single-Page, Scrolling)

**Section 1: Hero**
- Dark navy background
- UniWorld icon (the tech globe) centered or left-aligned
- "UniWorld" as large heading
- Tagline: "Correct Unicode text handling for every script."
- Sub-tagline or lead paragraph: Make clear this is a complete ecosystem -- library, VS Code extension, PowerShell module, five language bindings -- and that it solves problems for everyone, including English-speaking developers. The Latin-world angle should be visible within the first screenful.
- One paragraph: the summary from SITE_CONTENT.md (updated to include the "affects everyone" framing)
- CTA buttons: GitHub | VS Code Extension | PowerShell Module
- Attribution line: "By Sean MacNutt / [A Guy With AI](https://aguywithai.world). Built using [HAIMU](https://haimu.world), MacNutt's AI development methodology. Funded by [Grand Beta](https://grandbeta.world)."

**Section 2: The Problem**
- White background
- "Software gets text wrong." as section heading
- Short, punchy bullet points (from the value proposition above): emoji splitting, bidi confusion, Thai wrapping, CJK miscounting, combining mark orphaning
- Optional: a small visual showing a cursor stuck inside an emoji ZWJ sequence vs. UniWorld handling it correctly

**Section 3: What UniWorld Provides**
- Feature grid (cards or table): Bidi, Line Breaking, Segmentation, Normalization, Display Width, Safe Truncation, Case Mapping, Cursor Navigation
- Each card: feature name, UAX standard reference, one-line description
- Conformance numbers displayed prominently (e.g. "770,241 bidi test cases passing")

**Section 4: One Core, Many Surfaces**
- Visual showing: Rust core in the center, radiating outward to Python, JS/WASM, C, Go, VS Code, PowerShell
- Brief description of each binding / tool
- Install snippets (cargo add, pip install, npm install, ext install, Install-Module)

**Section 5: Scripts Covered**
- A visual strip or grid showing script samples (the icon's orbiting text idea, but expanded): Latin, Arabic, Hebrew, Devanagari, Thai, Chinese, Japanese, Korean, Bengali, Tamil, Khmer, Myanmar, Ethiopic, Cherokee, emoji
- Link to the Unicode Showcase document

**Section 6: Quick Start**
- Tabbed or side-by-side code examples for Rust, Python, JavaScript, PowerShell
- Real operations: grapheme boundaries, display width, normalization, bidi levels
- Keep it short -- three examples per language at most

**Section 7: Links and Resources**
- GitHub repo
- Documentation (docs/ directory)
- VS Code Extension (marketplace link)
- PowerShell Module (gallery link)
- Package registries (crates.io, PyPI, npm)
- Unicode Showcase (full stress-test document)

**Section 7.5: How UniWorld Was Built (HAIMU story block)**
- Short, compelling block before the footer. Drives traffic to haimu.world.
- "Built in 14 hours. From idea to library."
- 3-4 sentences: HAIMU methodology (originated by MacNutt) prompted for highest-ROI neglected technical projects; Unicode emerged; library + bindings + tools built from idea to implementation. Quote: "Move fast and fix things."
- CTA: link to haimu.world; link to aguywithai.world/projects/uniworld for deeper dev notes.

**Section 8: Footer**
- "UniWorld is open source under the MIT license."
- "By Sean MacNutt / [A Guy With AI](https://aguywithai.world)"
- "Built using [HAIMU](https://haimu.world), MacNutt's AI development methodology. Funded by [Grand Beta](https://grandbeta.world)."
- GitHub link
- Podcast: [A Guy With AI podcast](https://aguywithai.world)

### Responsive Behaviour

- Single column on mobile, two-column grid for feature cards on tablet+
- Hero section full-width on all sizes
- Code blocks scroll horizontally on narrow screens
- Navigation links wrap gracefully

### What NOT to Do

- No JavaScript frameworks. Plain HTML/CSS, maybe a tiny bit of vanilla JS for tab switching in the code examples section.
- No cookie banners, analytics, or tracking. The site is informational.
- No "Sign up for our newsletter." The project speaks for itself.
- No animations beyond subtle hover transitions. The content is the draw.
- No AI-generated stock photography. The icon and code samples are the visual content.

---

## Part 3: Key Links to Include

| Resource | URL |
|----------|-----|
| GitHub repo | https://github.com/aguywithai/uniworld |
| VS Code Extension | https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld |
| PowerShell Gallery | https://www.powershellgallery.com/packages/UniWorld |
| crates.io | https://crates.io/crates/uniworld |
| PyPI | https://pypi.org/project/uniworld/ |
| npm | https://www.npmjs.com/package/uniworld |
| A Guy With AI (publisher, podcast) | https://aguywithai.world |
| HAIMU methodology | https://haimu.world |
| Grand Beta (business, funding) | https://grandbeta.world |
| Sean MacNutt's projects | https://worldof.world |
| Unicode Showcase | (repo link: docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md) |
| Extension README | extensions/vscode/README.md |
| Root README | README.md |
| Integration docs | docs/integration/ (Python, JS, C, Go) |

---

## Part 4: Messaging for Specific Channels

### GitHub README (already good; minor additions when publishing)
- Add badges: crates.io version, PyPI version, npm version, VS Code installs, license
- Add "Install" section with one-liners for each platform
- Keep the current structure; it's clear and technical

### VS Code Marketplace (README.md in extension)
- Already updated with features, settings, commands
- Add the uniworld.world and grandbeta.world links (done)

### PowerShell Gallery
- Module description: "Correct Unicode text processing cmdlets: segmentation, normalization, display width, bidi analysis, line breaking. Backed by a Rust core passing 770K+ Unicode conformance tests."
- Tags: unicode, text, grapheme, emoji, bidi, normalization, display-width

### Social / Announcement (when ready)
- Lead with the universal problem: "Your code splits emoji. Your backspace orphans accents. Your terminal miscounts columns. Your strings that look identical don't match. These are Unicode problems and they happen in English too."
- Broaden to global: "For Arabic, Hebrew, Thai, Hindi, CJK, and dozens more scripts, the problems are worse. Bidi is broken. Line breaking is wrong. Word boundaries are guessed."
- Deliver the solution: "UniWorld: one library, every script, every Unicode algorithm. Rust core, Python/JS/C/Go bindings, VS Code extension, PowerShell module."
- End with the proof: full Unicode UCD conformance (run `cargo test --features conformance` for printed totals; data pinned to UCD 17.0.0).
- Link to uniworld.world

---

## Appendix: Design Mockup Notes for Developer

When building the full HTML/CSS site, use the existing `_publishing/site/index.html` and `style.css` as the starting point. The current page is a minimal stub; expand it section by section following the structure above. The existing CSS variables (colours, fonts, spacing) are a good base. Add sections as `<section>` elements with IDs for in-page navigation. Consider a sticky header with section links if the page gets long enough to warrant it.

The icon (icon.png or the full-resolution icon2.png) should be the hero image. For the favicon, resize to 32x32 or use an SVG if one is created later.

For code block syntax highlighting without a framework, consider embedding Prism.js (a single CSS + JS file, ~20KB) or using pre-formatted HTML with manual `<span>` colour classes. The latter is more in keeping with the "no dependencies" philosophy.
