# Auxiliary scripts

Helper scripts for development. Not shipped as part of the library.

| Script | Purpose |
|--------|---------|
| `download_ucd_tests.ps1` | Download pinned UCD + conformance files into `_development/data/ucd/` |
| `generate_*.py` | Regenerate `src/data/*.rs` from UCD inputs |
| `generate_dictionary_data.py` | Regenerate `src/data/dictionaries/*.dict` from ICU `*_dict.txt` sources |
| `STABLE_UCD_UPDATE.md` | Policy: stable releases only; dictionaries vs UCD |

Unicode updates: see repo root `VERSION_UPDATE_CHECKLIST.md` (gitignored) and `STABLE_UCD_UPDATE.md` here.
