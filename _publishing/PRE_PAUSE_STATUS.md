# Where We Are: Pre-Pause Checklist

**Updated**: After READMEs (and HAIMU) are complete. Use this to pick up the thread.

---

## Done

- **Stage 2: README alignment and cross-linking** — Complete. Root, VS Code, PowerShell, integration docs updated with HAIMU AI development methodology, dev story, uniworld.world, and cross-links. READMEs are declared complete (with the right to edit if developments occur before publishing).

---

## Next: Signups and Procurement (Stage 3)

**Yes — signups are logically next.**

Do these before or in parallel with pushing to GitHub. When you return after business tasks, you want accounts and tokens ready so you can publish without delay.

| Item | Where to do it | Notes |
|------|----------------|--------|
| **uniworld.world** | Porkbun or GoDaddy (see DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md) | Purchase and hold; do not set DNS yet (Pages in Phase B). |
| **haimu.world, worldof.world** | Same registrar if desired | Optional to buy now; strategy doc has the plan. |
| **VS Code Marketplace** | Azure DevOps | Create publisher "aguywithai"; get PAT for `vsce publish`. Store PAT in password manager. |
| **PowerShell Gallery** | powershellgallery.com | Account + API key for UniWorld. Store key securely. |
| **crates.io** | crates.io (GitHub login) | API token for `cargo publish`. |
| **PyPI / npm** | Optional for now | When Python/WASM packages are published. |
| **All tokens** | Password manager | Note expiry dates. |

Details: `_publishing/ACCOUNTS_AND_TOKENS.md`. Domain strategy (Porkbun vs GoDaddy, email): `_publishing/DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md`.

---

## After Signups: What’s Left Before You Leave This Repo

In order:

1. **Stage 1 (optional but recommended): Final local build and test**  
   One full pass so nothing is broken before push: `cargo test`, `cargo build --release --features cffi`, VS Code compile + package, Pester 68 tests, version check (0.1.0 everywhere). You can do this right before Stage 4.

2. **Stage 4: GitHub repository (private)**  
   Create `aguywithai/uniworld` (private), push, verify Actions: `build-native.yml` runs, Windows/Linux/macOS build, Pester (test-windows), artifact `uniworld-native-all`. Fix any CI failures.

3. **Stage 5: Cross-platform testing with CI artifacts**  
   Download `uniworld-native-all`, drop DLLs/so/dylib into `extensions/powershell/native/`, run Pester in WSL (and macOS if available), final VS Code .vsix test. All green.

4. **Pause**  
   Repo is on GitHub (private), CI green, accounts/tokens/domains acquired. You leave for business tasks (incorporation, patents, etc.) and/or website development.

---

## What Happens Where

- **This repo (UniWorld)**: Stages 1–5 above. No website code here; uniworld.world is built from content in `_publishing/site/` and deployed via GitHub Pages in Phase B (or when you’re ready).
- **Business tasks**: Separate (incorporation, patent filing). Not in this repo.
- **Websites**: Separate repo (e.g. web-properties). aguywithai.world, worldof.world, haimu, uniworld landing, etc. Use `_publishing/WEB_PROPERTIES_RULES.mdc`, `WEB_PROPERTIES_NOTES.md`, and `GODADDY_TO_PORKBUN_TRANSFER_AND_EMAIL_GUIDE.md` (port those into the web repo). You can develop sites in parallel with business tasks; they don’t block UniWorld publishing.
- **Simplecast → aguywithai.world/podcast**: Documented in the transfer guide and web notes: point aguywithai.world DNS to Hostinger, build landing at `/podcast`, embed or link to Simplecast at e.g. `/podcast/listen`. No change in this repo.

---

## Order Summary

1. **Signups (Stage 3)** — next.  
2. **Optional: Stage 1** — final local build/test run.  
3. **Stage 4** — push to GitHub private, verify CI.  
4. **Stage 5** — CI artifacts, cross-platform test.  
5. **Pause** — business tasks + website work elsewhere.  
6. **Phase B** (later): Go public, registries, uniworld.world Pages, outreach.

---

## If You Only Do One Thing Next

Do **Stage 3 (signups and procurement)**. Get domains (at least uniworld.world), VS Code publisher + PAT, PowerShell Gallery key, crates.io token. Then you can push to GitHub (Stage 4) and run CI (Stage 5) whenever you’re ready; when you come back after business, you’re set to publish.
