# Dev notes — 2025-02-10: Repo setup and Cursor rules

## Done this session

- **Cursor rules**
  - Rewrote `.cursor/rules/coding-practices.mdc` for UniWorld (Rust core + bindings, no DoI/water content). Codified: dev notes in `_development/dev_notes/`, scripts in `_development/scripts/`, tests at root, published docs in `docs/` or root; habit of creating dev notes before development.
  - Added `.cursor/rules/PROJECT_ROADMAP.mdc` (alwaysApply) with checklisted Phase 0–4 and document-generation steps; documents _development substructure and .gitignore observance.

- **Repo**
  - Added `.gitignore` (venv, Rust target, IDE, OS, local env; optional conformance data; build outputs).
  - Added `README.md` at root, `LICENSE` (MIT), `docs/README.md`, `tests/README.md`.
  - Added `requirements.txt` (pytest, pytest-cov for dev and future Python bindings).
  - Venv and `pip install -r requirements.txt` started in background (monitor for completion).

## Venv resolution (later session)

- **What worked**: On this machine `python -m venv .venv` fails because **ensurepip** returns non-zero (pip not installed in venv). Fix: (1) Remove existing `.venv` if present. (2) Create venv without pip: `python -m venv .venv --without-pip`. (3) Download get-pip.py: `Invoke-WebRequest -Uri https://bootstrap.pypa.io/get-pip.py -OutFile get-pip.py -UseBasicParsing`. (4) Bootstrap pip: `.\.venv\Scripts\python.exe get-pip.py`. (5) Install deps: `.\.venv\Scripts\pip.exe install -r requirements.txt`. (6) Remove get-pip.py. Note: get-pip and pip install need full permissions (sandbox blocks Temp writes and can close HTTPS). Use PowerShell `;` not `&&` for chaining. Venv is now ready (pip 26.x, pytest, pytest-cov).

## Roadmap vs project doc

- PROJECT_ROADMAP.mdc is aligned with `_development/docs/UniWorld_PROJECT.md`: Phase 1 = UAX #29, #15, #9, #14; Phase 2 = composite ops + bindings; Phase 3 = testing + docs; Phase 4 = community. Tools (ucd_gen, conformance runners, benchmark) are in the project doc and will be added when we add UCD-dependent code and conformance runs.

## Conventions to follow

- Create/update a dev note in `_development/dev_notes/` before starting development work.
- Put auxiliary scripts in `_development/scripts/`.
- Keep published docs in `docs/` or root; working docs in `_development/docs/`.
- Respect `.gitignore` when generating or adding files.

## Next steps (from roadmap)

- Phase 0: CONTRIBUTING added; Phase 0 checklist completed.
- Phase 1: Rust crate scaffolded; implement UAX #29 grapheme cluster boundaries next, then word/sentence and conformance.
