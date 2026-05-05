# Dev notes — 2025-02-10: Phase 0 complete, Phase 1 Rust scaffold

## Venv (documented)

Venv is resolved; procedure documented in `2025-02-10-setup-and-rules.md` (create with `--without-pip`, bootstrap via get-pip.py, install requirements; use PowerShell `;` and full permissions if needed).

## Roadmap alignment

Confirmed PROJECT_ROADMAP.mdc encompasses all development in `_development/docs/UniWorld_PROJECT.md`: Phase 1 (UAX #29, #15, #9, #14), Phase 2 (composite + bindings), Phase 3 (testing + docs), Phase 4 (community). Tools (ucd_gen, conformance, benchmark) noted in project doc; to be added when UCD and conformance are needed.

## Done this session

- **Phase 0 completed**
  - CONTRIBUTING.md added at root (build, test, submit test cases/dictionaries).
  - docs/integration/README.md and docs/contributing/README.md added.
  - PROJECT_ROADMAP.mdc Phase 0 and Document Generation checklists ticked.

- **Rust crate scaffolded (Phase 1 start)**
  - Cargo.toml at root (package `uniworld`, lib only, conformance feature).
  - src/ layout per project doc: segment (grapheme, word, sentence), normalize, bidi, linebreak, casemap, cursor, width, truncate, data.
  - UAX #29 segment stubs: `grapheme_boundaries()`, `GraphemeClusterBoundaries` iterator (currently one codepoint per cluster until full table); `word_boundaries()`, `sentence_boundaries()` return [0].
  - normalize: nfc/nfd/nfkc/nfkd stubs (return copy).
  - tests/segment_tests.rs: empty string, ASCII boundaries, iterator.
  - README build section updated (Rust toolchain, cargo build/test).
  - Roadmap Phase 1 first bullet updated to note scaffold and stub in place.

- **Build note**
  - `cargo` was not in PATH in the automation environment; crate is structured to build. Run `cargo build` and `cargo test` locally with Rust installed.

## Next steps

- Implement full UAX #29 grapheme cluster rules (Grapheme_Cluster_Break property table or equivalent); add conformance test runner for GraphemeBreakTest.txt.
- Then word and sentence boundaries; then UAX #15 normalization, UAX #9 bidi, UAX #14 line break per roadmap.

## Conventions

Dev notes before development; scripts in _development/scripts/; tests at root; published docs in docs/ or root. Observe .gitignore.
