# Dev Notes: Sequencing, Tools, GitHub-First, and Final Steps to Publish

**Date**: 2026-02-10
**Purpose**: Order of work (Phase 4 vs workflow), what tools you use, whether GitHub comes first, and how final steps lead into first publishing steps.

---

## Do You Need to Publish on GitHub First?

**Yes.** GitHub Actions run only when the repository exists on GitHub and you push (or open a PR, or create a release). So the sequence is:

1. **Create the GitHub repo** (e.g. github.com/aguywithai/uniworld), if not already done.
2. **Push your local repo** (or clone from GitHub and push your code).
3. **Add the workflow file** (`.github/workflows/build-native.yml`) and push again — the workflow will run on the next push to `main`/`master` or on release.

You cannot "run GitHub Actions from your CLI" in the sense of executing the workflow on your machine. The workflow runs on GitHub’s runners when triggered by events (push, PR, release). From your CLI you:

- **Trigger** the workflow: `git push origin main` or `git push origin v0.1.0` (tag).
- **View** runs: GitHub → Actions tab, or `gh run list` / `gh run view` if you use the GitHub CLI (`gh`).

Optional: **act** (https://github.com/nektos/act) runs GitHub Actions locally in Docker. Useful to debug a workflow before pushing; not required.

---

## Tools You’ll Use for CI and Publishing

| Task | Tool | Where |
|------|------|--------|
| Trigger workflows | `git push`, or create release/tag on GitHub | Your machine (git) or GitHub web |
| View workflow runs | GitHub Actions tab, or `gh run list` | Browser or `gh` CLI |
| (Optional) Run workflows locally | `act` | Your machine (Docker required) |
| Publish VS Code extension | `vsce publish` (or `npm run package` then upload .vsix manually) | Your machine, from `extensions/vscode/` |
| Publish PowerShell module | `Publish-Module -Path . -NuGetApiKey $env:PSGALLERY_API_KEY` | Your machine, from `extensions/powershell/` |
| Publish Rust crate | `cargo publish` | Your machine |
| Publish Python package | `maturin publish` or `twine upload` | Your machine (or CI) |
| Publish npm package | `npm publish` | Your machine (or CI) |

For the **native build workflow** specifically: you only need **git** and **GitHub**. Push the branch; the workflow runs on GitHub. No local Rust/macOS/Linux needed for the cross-platform builds.

---

## Recommended Order: Phase 4 First, Then Rely on the Workflow

1. **Phase 4 (PowerShell) first**  
   - Expand Pester tests so they cover all cmdlets and meaningful Unicode cases (grapheme, word, sentence, width, truncate, normalize, bidi, line break, Get-UnicodeInfo).  
   - Tests live in the repo; CI can run them (the workflow already has a `test-windows` job that runs Pester).  
   - Complete comment-based help for all cmdlets.  
   - This gives you a solid baseline before depending on CI.

2. **Then push to GitHub and use the workflow**  
   - Repo on GitHub → push `.github/workflows/build-native.yml` (and the updated module that looks in `native/win-x64` etc.).  
   - Workflow runs on push to main and on release; it builds .dll/.so/.dylib and produces the `uniworld-native-all` artifact.  
   - You (or a collaborator) can download that artifact and test on Windows/Linux/macOS, or add later steps that attach it to a release.

3. **Linux/macOS testing**  
   - **Linux**: Use WSL on your machine (see cross-platform dev note for exact commands).  
   - **macOS**: Either get someone with a Mac to run the same tests, or rely on the workflow’s macOS job (build already validates it compiles; running Pester on macOS would require adding a macos test job that installs PowerShell and runs the tests).

Preparing for Linux/macOS up front means: (a) Pester tests in the repo that anyone can run, and (b) clear instructions (e.g. in the cross-platform dev note) for WSL and for a Mac tester. The workflow’s Linux/macOS jobs validate the native build; full module testing on those OSes can be manual or a later CI job.

---

## Align READMEs and Website Before / As You Near Completion

Before or as you approach “done”:

- **Root README**: Reflect current state (UniWorld library, VS Code extension, PowerShell module, bindings, link to uniworld.world).
- **Extension README** (`extensions/vscode/README.md`): Already updated; ensure uniworld.world and grandbeta.world links are correct.
- **PowerShell README** (`extensions/powershell/README.md`): Update for Phase 4 (how to run tests, that CI builds native binaries, install from Gallery when published).
- **Website (uniworld.world / GitHub Pages)**: Per `_publishing/MARKETING_AND_WEBSITE_DESIGN.md` and `_publishing/site/SITE_CONTENT.md` — build the full HTML/CSS site when ready; can go live after repo is public and first packages are published.

You don’t have to publish packages before creating the GitHub repo. Create the repo, push code (and workflow), align READMEs and docs, then do the first publish steps (VS Code, PowerShell, crates.io, etc.) when you’re ready.

---

## Final Steps and How They Lead Into First Publishing Steps

### Truly final steps (before “first publish”)

| Step | What | Leads to |
|------|------|----------|
| Phase 4 PowerShell | Pester coverage, help, any last fixes | Module ready to publish; CI can run tests |
| Workflow in repo | `.github/workflows/build-native.yml` pushed | Builds and artifacts available on GitHub |
| Repo on GitHub | Create repo, push code + workflow | Actions run; you can create releases and attach artifacts |
| README/docs alignment | Root, VS Code, PowerShell, publishing docs | Clear story for users and for Gallery/Marketplace |
| (Optional) Website | Build from SITE_CONTENT / marketing doc | uniworld.world or GitHub Pages as landing |

### First publishing steps (once the above are in place)

| Step | What | Tool / place |
|------|------|----------------|
| 1. Create GitHub repo | Create uniworld (or chosen name) under your account | GitHub web |
| 2. Push code | Add remote, push main (and tags if you use them) | `git remote add origin ... ; git push -u origin main` |
| 3. Confirm workflow | Check Actions tab for build-native (and test-windows) | GitHub Actions |
| 4. Publish VS Code extension | Package and publish to Marketplace | `cd extensions/vscode ; vsce publish` (need VSCE_PAT or `vsce login`) |
| 5. Publish PowerShell module | Option A: use `uniworld-native-all` artifact and pack module with native/ folder; Option B: build DLL locally and pack. Then publish. | `Publish-Module -Path extensions/powershell -NuGetApiKey $env:PSGALLERY_API_KEY` (need API key) |
| 6. (Optional) Publish library | crates.io, PyPI, npm per `_publishing/CHECKLIST.md` | `cargo publish`, `maturin publish`, `npm publish` |
| 7. Enable GitHub Pages | If using Pages for site, enable in repo Settings | GitHub → Settings → Pages |
| 8. Tag release | e.g. `v0.1.0` | `git tag v0.1.0 ; git push origin v0.1.0` |

So: **final steps** = Phase 4 + workflow + repo on GitHub + README/website prep. **First publishing steps** = push, confirm CI, then publish extension and module (and optionally library and site). The workflow doesn’t publish for you; it produces artifacts. You (or a later workflow step) use those artifacts when packaging the PowerShell module for the Gallery.
