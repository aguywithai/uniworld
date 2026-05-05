# Stable UCD update policy (solo maintainer)

## Unicode standard

- **Use only shipped releases** (e.g. `17.0.0` under `https://unicode.org/Public/<version>/`), not alpha/beta UCD trees.
- **Rationale:** Beta data changes; conformance counts and property files move. A side project cannot track every draft. Users expect the published standard. When a new version ships (usually September), run the version checklist and release a minor SemVer bump on the library.
- **Exception:** If the Consortium publishes a **corrigendum** or errata for a release you already ship, treat that as a patch or data refresh for the same major Unicode version, not as a "beta chase."

## ICU / dictionary data (Thai, Lao, Khmer, Myanmar)

- Dictionaries are **independent** of the annual UCD drop. They come from ICU `*dict.txt` files copied into `_development/data/dictionaries/`.
- **When to refresh:** When you intentionally pull new ICU source (e.g. after a new ICU major that you care about), or when line-break / word tests for those scripts start failing in a way that points at dictionary gaps.
- **When to skip:** A pure UCD 16 -> 17 table regeneration does **not** require a dictionary refresh unless you choose to align with a specific ICU release for those four languages.
- **How to refresh:** Obtain the four ICU dictionary sources, replace the `.txt` files under `_development/data/dictionaries/`, then run `python _development/scripts/generate_dictionary_data.py` to regenerate `src/data/dictionaries/*.dict`.

## Script

`download_ucd_tests.ps1` defaults to `-UcdVersion "17.0.0"` and writes a flat `_development/data/ucd/` tree so existing Python generators keep the same input paths.
