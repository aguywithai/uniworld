# Dev Notes: Cross-Platform Native Library Build Plan (.so / .dylib)

**Date**: 2026-02-10
**Status**: Workflow added. CI builds on push to main and on release; module updated to use `native/<rid>/` layout.
**Context**: PowerShell module uses Rust cdylib via C FFI. Windows .dll used during development; CI produces .so and .dylib.

---

## What We Have Now

- `Cargo.toml` has `crate-type = ["lib", "cdylib"]`. C FFI is behind `--features cffi`; **CI and local builds must use** `cargo build --release --features cffi`.
- **GitHub Actions**: `.github/workflows/build-native.yml` builds on Windows, Ubuntu, and macOS; uploads per-OS artifacts and an assembled `uniworld-native-all` artifact (layout below). Also runs Pester on Windows.
- **Module**: `Initialize-UniWorldNative` looks for the library in: `native/<rid>/<libName>`, then `native/<libName>`, then `target/release/<libName>`. RID is `win-x64`, `linux-x64`, or `osx-arm64`.

## What Cross-Platform Enables

- PowerShell 7+ on Linux and macOS can load the module when `native/linux-x64/libuniworld.so` or `native/osx-arm64/libuniworld.dylib` is present (e.g. from the CI artifact).
- Required for PowerShell Gallery if the module is listed as cross-platform.
- Useful for CI and DevOps on Linux.

## CI Workflow (Option A — Implemented)

1. **Workflow**: `.github/workflows/build-native.yml`
   - Trigger: push to `main`/`master`, or release events.
   - Matrix: `windows-latest`, `ubuntu-latest`, `macos-latest`; each runs `cargo build --release --features cffi`.
   - Artifacts: `uniworld-windows`, `uniworld-linux`, `uniworld-macos`; then **assemble** job produces `uniworld-native-all` with:
     ```
     extensions/powershell/native/
       win-x64/uniworld.dll
       linux-x64/libuniworld.so
       osx-arm64/libuniworld.dylib
     ```
   - **test-windows** job runs Pester on Windows (needs repo on GitHub and push to trigger).

### Option B: Cross-Compile from Windows (Alternative)

- Install Rust cross-compilation targets:
  ```
  rustup target add x86_64-unknown-linux-gnu
  rustup target add aarch64-apple-darwin
  ```
- Requires a linker for each target. On Windows this means either:
  - **cross** (Docker-based): `cargo install cross` then `cross build --release --target x86_64-unknown-linux-gnu`. Requires Docker Desktop.
  - **zig as linker**: `cargo install cargo-zigbuild` then `cargo zigbuild --release --target x86_64-unknown-linux-gnu`. Requires zig installed.
- Apple targets additionally require the macOS SDK (available via osxcross or similar); this is fiddly from Windows.
- **Verdict**: Option A (CI) is more reliable and maintainable. Option B is useful for local testing but not recommended as the primary build path.

## Difficulty Estimate

| Task | Effort | Notes |
|------|--------|-------|
| Write GitHub Actions workflow | 1-2 hours | Standard matrix build; maturin-action is a template |
| Update module native path logic | 15 minutes | Small change to `Initialize-UniWorldNative` |
| Test on Linux | 30 minutes | Use WSL or Docker |
| Test on macOS | 30 minutes | Requires macOS machine or CI only |
| Package for PowerShell Gallery | 1 hour | Include all three binaries in the module zip |
| **Total** | **~3-4 hours** | Mostly CI setup and testing |

## Testing on Windows Without Other Machines

### WSL (Windows Subsystem for Linux) — Recommended

You already have WSL (distribution exists). Use it to build and test the Linux .so and the module.

**1. Enter WSL and go to your project (clone or use existing folder):**
```bash
wsl
cd /mnt/c/Users/seanm/Documents/projects/unicode
# Or clone: git clone https://github.com/aguywithai/uniworld.git && cd uniworld
```

**2. Install Rust (if not already):**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

**3. Build the native library (C FFI required):**
```bash
cargo build --release --features cffi
```
This produces `target/release/libuniworld.so`.

**4. Install PowerShell 7 (optional; for full module test):**
```bash
sudo apt-get update && sudo apt-get install -y wget apt-transport-https software-properties-common
wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb
sudo dpkg -i packages-microsoft-prod.deb
sudo apt-get update && sudo apt-get install -y powershell
```

**5. Run the module (from repo root):**
```bash
pwsh -NoProfile -Command "Import-Module ./extensions/powershell/UniWorld.psd1 -Force; Get-DisplayWidth 'Hello'; Get-GraphemeBoundaries 'Hello'"
```
The module will find `target/release/libuniworld.so` via the path `extensions/powershell/../../target/release/` (relative to the module directory).

**6. Run Pester tests (if Pester is installed in pwsh):**
```bash
pwsh -NoProfile -Command "Invoke-Pester -Path extensions/powershell/Tests/ -Output Normal"
```

### Docker
- Run a Linux container: `docker run -it -v ${PWD}:/work rust:latest bash`
- Build inside the container; copy the .so out.
- Can also run PowerShell in the container (`mcr.microsoft.com/powershell`).
- More setup than WSL but fully isolated.

### macOS Testing
- No native macOS emulator on Windows. Options:
  - GitHub Actions (free for public repos): push a test branch, CI builds and runs tests on macOS.
  - A macOS VM (requires macOS hardware or cloud service like MacStadium / GitHub Codespaces with macOS).
  - For PowerShell module purposes, the .dylib build is structurally identical to .so; if Linux works, macOS almost certainly will too. CI-only testing for macOS is acceptable.

## Decision

Ship Windows-only for initial development and testing. Add cross-platform via GitHub Actions CI when setting up the publish pipeline (Stage 2 of the publishing checklist). The module code already handles all three platforms; only the native binaries are missing.
