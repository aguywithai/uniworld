# Dev Notes: Reflections on the VS Code Extension at Completion

**Date**: 2026-02-10
**Context**: VS Code extension 0.1.0 receiving finishing touches; transitioning to PowerShell module.

---

## What We Built

A VS Code extension that solves real, daily Unicode problems using a Rust/WASM core that passes every official conformance test suite. The extension went from scaffold to feature-complete in a single concentrated development arc:

- **Phase 1**: WASM integration, status bar, lazy loading. The foundation that everything else rests on.
- **Phase 2**: Grapheme cursor, grapheme delete, hover inspector, inspect command, truncation, normalization. This is the "just works" layer -- the features that English-only users benefit from without knowing they need them.
- **Phase 3**: Visual bidi cursor, line break decorations, word selection, bidi run highlighting. The features that make UniWorld relevant to every script on earth.

## What Went Well

**The Rust core was the right investment.** Every extension feature is a thin TypeScript wrapper around a WASM call. The hover inspector, the bidi visualization, the line break decorations -- they all call into the same conformance-tested algorithms. There's no duplication, no drift, no "JS reimplementation" of Unicode rules. When the bidi visualization needed `bidi_levels()`, it was already there, already correct, already tested against 770,241 BidiTest cases. This is the compound return on building the core library properly.

**Settings-gated keybindings.** The decision to use `when` clauses in package.json keybindings instead of runtime key interception was important. It means the extension has zero overhead when features are disabled, and users never have to manually edit keybindings.json. The `enableBidiVisualCursor` taking precedence over `enableGraphemeCursor` via `!config.uniworld.enableBidiVisualCursor` in the when clause was a clean solution to the priority problem.

**Common-sense defaults.** Shipping grapheme cursor and delete as on-by-default was the right call. The extension should improve the experience from the moment it's installed. The advanced features (bidi cursor, visualizations, word select) being opt-in respects that not everyone needs them.

**Visible-lines-only processing.** For decorations (line breaks, bidi highlighting), only processing visible lines keeps the extension fast regardless of file size. The refresh-on-scroll pattern is simple and effective.

## What Was Harder Than Expected

**Bidi cursor movement.** The visual cursor was the most complex feature. The initial approach of mapping logical direction to visual direction based on embedding level was correct in principle, but the edge cases at run boundaries, line crossings, and mixed-direction text required multiple iterations. The final solution using pre-computed visual cursor stops from Rust (with deduplication) and a pure index-based TypeScript driver was much cleaner than the intermediate versions.

**Decoration wiring.** The bidi visualization bug -- decorations not appearing -- was a straightforward omission (missing event listener wiring and missing startup initialization), but it's the kind of bug that's easy to introduce when building features incrementally. The pattern of "create decoration types, set up config listener, wire into event handlers, initialize on startup" has four separate points of failure. The line break decorations got this right first time because they were the template; bidi visualization was added later and missed steps. Lesson: when adding a new decoration feature, copy the full pattern including all wiring, not just the core logic.

**Contrast tuning.** Finding the right opacity for bidi highlighting took iteration. Too subtle and it's invisible; too heavy and it interferes with reading. The final values (15-18% background, 28-30% border) strike a balance that's clearly visible without being distracting, but this is inherently subjective and theme-dependent. A future enhancement could use theme-aware colours.

## Architecture Decisions Worth Preserving

1. **WASM is loaded lazily** via `loadWasm()` with a `wasmLoadFailed` flag. This means the extension activates fast and degrades gracefully if WASM fails to load.

2. **One source of truth for Unicode data.** The extension has zero Unicode tables of its own. Everything comes from the WASM module, which is built from the Rust core, which is generated from official Unicode 17.0 data files. This chain of trust matters.

3. **Decoration lifecycle.** Decorations are created on activate and disposed on deactivate. They're not recreated on every refresh -- only the ranges change. This is the correct VS Code pattern for performance.

4. **No native dependencies.** The extension is pure TypeScript + WASM. It works on every platform VS Code runs on without compilation, dynamic linking, or platform-specific binaries.

## What's Left (Known Limitations)

- **RTL mouse click positioning**: VS Code maps mouse clicks to logical positions before the extension can intervene. A pragmatic correction for pure-RTL lines is feasible but would cause cursor flicker and wouldn't solve click-drag selection. Deferred.

- **Theme-aware decoration colours**: The bidi highlighting uses fixed RGBA values. A more polished version could read theme colours or offer configurable colours via settings.

- **No bidi panel**: The roadmap originally mentioned "show logical vs visual order in a panel." The inline highlighting was implemented instead, which is more immediately useful. A side panel showing reorder mapping could be a future feature.

- **WASM size**: The .wasm file is ~4.5 MB (compressed to ~1 MB in the .vsix). This includes all Unicode data tables. It's acceptable for an extension but larger than minimal. If size becomes an issue, the data tables could be split or loaded on demand.

## On the Broader Project

The VS Code extension is one surface of a larger system. The same core library will power the PowerShell module, the Python and JavaScript packages, and the C and Go bindings. The extension demonstrates that the core is solid and the binding layer works. Every feature in the extension -- grapheme movement, bidi analysis, line breaking, normalization -- is a function call into the same Rust crate that other consumers will use.

The decision to keep everything in one monorepo is paying off. The VS Code extension's WASM is built from the same `src/` as the library. When we move to the PowerShell module, it will use the same core (likely via C FFI or a .NET wrapper calling into the compiled Rust). One set of conformance tests, one set of Unicode data tables, multiple output surfaces.

## For the Record

770,241 bidi test cases. 19,338 line break test cases. 766 grapheme tests. 1,944 word boundary tests. 512 sentence boundary tests. All passing. The extension is built on that foundation, and it shows in the reliability of every feature.

The extension is ready to ship. The remaining polish (mouse click, theme colours, panel) is genuinely optional -- nothing in the current feature set is broken or incomplete. Version 0.1.0 is a legitimate first stable release.
