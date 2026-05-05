# Dev notes -- 2025-02-11: Rename unicore -> uniworld

## Reason

The name "unicore" was already taken on PyPI by the Praekelt Foundation's "Universal
Core" project (with multiple sub-packages). Crowding an existing namespace is both a
collision risk and a bad look for an open-source project. The origin chat recommended
checking availability and switching before the name embedded deeper.

## Registry availability (verified 2025-02-11)

| Registry  | "unicore"              | "uniworld"      |
|-----------|------------------------|-----------------|
| PyPI      | TAKEN (Praekelt)       | CLEAR (404)     |
| npm       | not checked            | CLEAR (404)     |
| crates.io | not confirmed (SSL)    | CLEAR (no hits) |

## Why "uniworld"

- Brand coherence with .world domains
- Communicates the project's international/all-scripts mission
- Clear on all three registries
- Distinctive -- no confusion with existing projects
- `pip install uniworld`, `use uniworld::bidi` read well
- "UniWorld: Correct Unicode for Every Script" is a four-word pitch

## Files changed

Global rename across all source, config, docs, and tests:

- `Cargo.toml`: package name and lib name
- `src/lib.rs`: crate doc comment
- All 6 test files: `use unicore::` -> `use uniworld::`
- `.cursor/rules/PROJECT_ROADMAP.mdc` and `coding-practices.mdc`
- `README.md`, `CONTRIBUTING.md`, `docs/README.md`, `tests/README.md`
- `LICENSE`, `requirements.txt`
- `_development/docs/UniCore_PROJECT.md` -> renamed to `UniWorld_PROJECT.md`
  (content updated, file renamed)
- `_development/scripts/download_ucd_tests.ps1`
- All dev notes referencing the old name
- `_development/dev_notes/*.md` (3 files with straggling references)

## Verification

Full test suite passes after rename:
- BidiTest.txt: 770,241/770,241
- BidiCharacterTest.txt: 91,707/91,707
- NormalizationTest.txt: 20,034/20,034
- WordBreakTest.txt: 1,944/1,944
- SentenceBreakTest.txt: 512/512
- GraphemeBreakTest.txt: 766/766
- 20 unit/integration tests

Total: 884,204 conformance cases + 20 unit tests, all passing.
Binary compiles as `uniworld v0.1.0`.
