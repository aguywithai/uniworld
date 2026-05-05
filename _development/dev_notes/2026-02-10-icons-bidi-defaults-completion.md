# Dev Notes: Icon Variants, Bidi Visualization, Defaults, Completion Readiness

**Date**: 2026-02-10
**Focus**: Icon assets, bidi colour highlighting, common-sense defaults, dev complete checklist

## Summary

- Produced square-aspect icon variants combining tech-globe style, orbiting multi-script text (including Arabic RTL), and Asia/Africa view. User to select one and save as `icon.png`.
- Shipped togglable bidi colour highlighting (`uniworld.showBidiVisualization`).
- Switched to common-sense default profile: grapheme cursor and grapheme delete **on by default**.
- Documented completion status and next steps (version bump, final test, publish).

## 1. Icon Assets (Square, Tech + Orbiting Scripts, Asia/Africa)

**Requirement**: Tech feel (blue wireframe globe, neon green continents, circuit-board vibe), orbiting "Hello"-style text in multiple scripts (Latin, Arabic, Chinese, Japanese), planet orientation showing Asia and Africa, **square aspect ratio** for extension icon (no cropping).

**Generated variants** (in `assets/`):

- `uniworld-icon-tech-scripts-1.png` - Tech globe, Asia/Africa view, orbiting scripts (Hello, Marhaba, Nihao, Konnichiwa).
- `uniworld-icon-tech-scripts-2.png` - Same brief; alternate layout/emphasis.
- `uniworld-icon-tech-scripts-3.png` - Same brief; third variant.

User to choose one, copy to final location, and name as `icon.png` for extension and/or website. See `_publishing/CHECKLIST.md` for where to place icon in the extension package.

## 2. Bidi Visualization (Shipped)

- **Setting**: `uniworld.showBidiVisualization` (boolean, default `false`).
- **Behaviour**: When enabled, LTR runs get a light blue background, RTL runs light orange. Uses WASM `bidi_levels()` per visible line; run boundaries derived from consecutive same-level code points; level 255 (X9-removed) treated as LTR.
- **Performance**: Only visible lines processed; refreshes on config change, document edit, editor change, visible range change.
- **Roadmap**: Phase 3 "Bidi visualization" marked complete; settings table and technical notes updated in `VSCODE_EXTENSION_ROADMAP.mdc`.

## 3. Common-Sense Default Profile

**Rationale**: Most users expect one arrow key = one visible "character" (grapheme) and one Backspace/Delete = remove one grapheme. Defaulting these on reduces emoji/Indic/combining-mark surprises with no downside for plain ASCII.

**Defaults changed** (in `extensions/vscode/package.json`):

| Setting | Old default | New default |
|---------|-------------|-------------|
| `uniworld.enableGraphemeCursor` | `false` | `true` |
| `uniworld.enableGraphemeDelete` | `false` | `true` |

**Left unchanged (off by default)**:

- `uniworld.enableBidiVisualCursor` - Only relevant for RTL; LTR-only users unchanged.
- `uniworld.enableGraphemeWordSelect` - Changes double-click behaviour globally; keep opt-in.
- `uniworld.showLineBreakOpportunities` / `uniworld.showBidiVisualization` - Visual overlays; opt-in.

Roadmap settings table updated to reflect new defaults and the "common-sense" wording.

## 4. Development Completeness

**Phase 1-3**: All checklist items complete (foundation, core features, advanced features including bidi visualization).

**Phase 4 (Polish and Publish)**:

- Settings UI: toggles documented and grouped under UniWorld (done in package.json descriptions).
- Icon and branding: icon variants produced; user to select and place; marketplace description/screenshots as needed.
- Package and test: `vsce package`, test on multiple file types and scripts - ready for user to run.
- Publish: push `.vsix` to marketplace when ready.

**Optional future enhancements** (not required for "fully complete"):

- Command-palette toggle for bidi visualization (in addition to setting).
- Optional "strict RTL" or extra bidi display options.
- Additional icon size variants (e.g. 128, 256) if marketplace requires.

## 5. Next Steps for User

1. Pick preferred icon from the three tech-scripts variants; copy/rename to `icon.png` and place per publishing checklist.
2. Version bump (e.g. to 0.0.11 or 0.1.0) when ready for release.
3. Run final tests (grapheme cursor/delete, bidi cursor, bidi visualization, line breaks, word select, hover, normalization) per `docs/UniWorld_Extension_Test_Guide.md`.
4. Package with `vsce package` and publish to VS Code Marketplace when satisfied.
