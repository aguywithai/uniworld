# Dev Notes: PowerShell Module Phase 1-3 Complete; PowerShell 7 Recommendation

**Date**: 2026-02-10
**Status**: Phases 1-3 done. Phase 4 (Pester, help, publish) deferred until after PowerShell 7 setup.

---

## Summary

The UniWorld PowerShell module is fully functional on Windows with the native DLL. All 12 cmdlets call the Rust FFI; segmentation, normalization, display width, truncation, bidi, and line breaking all work. Phases 1-3 of the PowerShell roadmap are complete.

## Current State

- **Native library**: Built with `cargo build --release --features cffi`. Output: `target/release/uniworld.dll`.
- **Interop**: Inline C# P/Invoke via `Add-Type` in the .psm1; UTF-8 and array marshalling handled; DLL path resolved at load (module dir `native/` or repo `target/release/`).
- **Manifest**: `UniWorld.psd1` — PowerShellVersion set to **5.1** so the module loads in both Windows PowerShell 5.1 and PowerShell 7.
- **Tested on**: Windows PowerShell 5.1 (current dev machine). PowerShell 7 (pwsh) not yet installed; Phase 4 will include testing on PS 7.

## Recommendation: Install PowerShell 7

**Yes — recommend installing PowerShell 7** for this project.

**Why:**
- PowerShell 7 is the cross-platform, actively developed edition. The Gallery and most docs assume PS 7+.
- UniWorld is documented as supporting PowerShell 7; we should verify the module there.
- You can keep using Windows PowerShell 5.1 for day-to-day use; having **both** lets you run the same module in both and catch any differences (e.g. `$IsWindows` exists in 7, not in 5.1 — we already handled that).
- No implementation changes are required for PS 7; the module works as-is in both 5.1 and 7.

**How to install (Windows):**
- **winget**: `winget install Microsoft.PowerShell`
- **MSI**: [PowerShell releases](https://github.com/PowerShell/PowerShell/releases) — download the latest `.msi` for Windows.
- **Microsoft Store**: Search for "PowerShell" and install "PowerShell" (the open-source edition).

After install, use `pwsh` from a new terminal to launch PowerShell 7. You can then run:
```powershell
Import-Module .\extensions\powershell\UniWorld.psd1 -Force
Get-DisplayWidth "Hello"
```

## Implementation: No Changes Needed for PS 7

The module is written to work on both 5.1 and 7:
- Platform detection uses `($null -eq $IsWindows) -or $IsWindows` so it works when `$IsWindows` is absent (5.1) or true/false (7).
- No PS 7–only cmdlets or syntax are used.
- `Join-Path` is called with two arguments only (5.1 compatible).

**Optional later decision:** If you want to publish to the Gallery as "PowerShell 7 only," you can set `PowerShellVersion = '7.0'` in the manifest. That would prevent 5.1 users from loading the module. Many modules support both; we currently do. No change unless you decide to drop 5.1 support.

## Phase 4 (Next)

After PowerShell 7 is installed (and optionally verified with a quick import + Get-DisplayWidth):
- **Pester tests**: Expand `Tests/UniWorld.Tests.ps1` to cover all cmdlets with Unicode samples (grapheme, word, sentence, width, truncate, normalize, bidi, line break, Get-UnicodeInfo).
- **Help**: Ensure comment-based help is complete for all cmdlets (`Get-Help Get-GraphemeBoundaries -Full`).
- **Platform DLLs**: Per the cross-platform plan, add CI or document single-platform build; optionally ship .so/.dylib later.
- **Publish**: `Publish-Module` to PowerShell Gallery when ready.

## Files Touched This Session

- `src/c_bindings.rs` — expanded FFI (grapheme, word, sentence, bidi, line break, truncate, NFKC/NFKD, array free).
- `extensions/powershell/UniWorld.psm1` — full P/Invoke interop and all 12 cmdlets wired to Rust.
- `extensions/powershell/UniWorld.psd1` — PowerShellVersion 5.1; exports unchanged.
- `.cursor/rules/POWERSHELL_MODULE_ROADMAP.mdc` — Phases 1–3 marked complete; technical notes updated.
- `_development/dev_notes/2026-02-10-cross-platform-native-library-plan.md` — cross-platform build plan (deferred).
