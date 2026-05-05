# UniWorld API reference plan

UniWorld’s public Rust API is already documented with `///` doc comments referencing the Unicode algorithms it implements (UAX #9, #14, #29, #15, etc.). This document outlines how to surface that API in a user-friendly way as part of Phase 3.

## 1. Rust API (crate-level)

Primary tools:

- **rustdoc**: generate HTML API docs from the Rust source.
- **docs.rs**: once the crate is published, docs.rs will host the rustdoc output.

Recommended commands during development:

```bash
cargo doc --no-deps --open
```

This builds local API documentation for:

- Core modules: `bidi`, `linebreak`, `segment`, `normalize`, `casemap`.
- Composite modules: `cursor`, `width`, `truncate`.
- Data modules: `data::*` (for contributors who need to understand generated tables).

## 2. Binding-level API summaries

Binding APIs are intentionally thin wrappers over the Rust core. The integration guides in `docs/integration/` serve as human-friendly entry points:

- `docs/integration/python.md`
- `docs/integration/javascript-wasm.md`
- `docs/integration/c.md`
- `docs/integration/go.md`

Each guide documents:

- How to build/install the binding for that language.
- The main functions exposed (segmentation, normalization, bidi, line breaking, width, truncation, cursor, case mapping).
- Short code samples.

## 3. Relationship to Unicode specs

API docs should consistently point from functions to the relevant Unicode specifications:

- Bidi: UAX #9
- Line breaking: UAX #14
- Segmentation: UAX #29
- Normalization: UAX #15

This is already reflected in many doc comments; Phase 3 can expand and standardize those references so that rustdoc output clearly answers “which UAX does this implement?” for each function.

## 4. Future: mdBook or similar

If a more narrative “book-style” reference is desired later, a future step could be:

- Add an `mdBook` in `_development/docs/book/` describing:
  - Conceptual overview of each algorithm.
  - How UniWorld’s implementation maps to the UAX rules.
  - Cross-references into the Rust API (module/function names).

For now, the combination of:

- `cargo doc` (Rust API)
- `docs/integration/*` (per-language usage)
- `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md` (rich test/presentation file)

is sufficient for Phase 3’s API reference goals.

