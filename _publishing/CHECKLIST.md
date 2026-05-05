# UniWorld publishing checklist and instructions

Single reference for taking UniWorld from "tested and ready" to published (crates.io, PyPI, npm) and optionally to a site/podcast link. Use this as your checklist and instruction set at every stage.

**Repo**: Personal GitHub **@aguywithai**. Publish and CLI work (including by the bot) use tokens you store locally; see "Where to add tokens" below.

---

## Where to add tokens (this repo, for local CLI and bot)

Tokens must **never** be committed. This repo is set up so you can add them in one place for the bot to use when running publish commands.

- [ ] **Create `.env` in the repo root** (same level as `Cargo.toml`). It is already in `.gitignore` and will not be committed.
- [ ] **Use the template**: Copy `.env.example` to `.env`:  
  `cp .env.example .env` (or create `.env` and paste the variable names from `.env.example`).
- [ ] **Fill in only the tokens you need**:
  - **crates.io**: Create a token at [crates.io/settings/tokens](https://crates.io/settings/tokens). In `.env` set `CARGO_REGISTRY_TOKEN=<your-token>`. The bot (or you) can run `cargo login --token $CARGO_REGISTRY_TOKEN` before `cargo publish`, or you run `cargo login` once on your machine and cargo will use `~/.cargo/credentials.toml` instead.
  - **PyPI**: Create a project-scoped token at [pypi.org/manage/account/token/](https://pypi.org/manage/account/token/). In `.env` set `PYPI_API_TOKEN=<your-token>`. `maturin publish` and `twine` use this when the env var is set.
  - **npm**: Create an Automation or Publish token at [npmjs.com/settings/~/tokens](https://www.npmjs.com/settings/~/tokens). In `.env` set `NPM_TOKEN=<your-token>`. For `npm publish`, you can run `npm config set //registry.npmjs.org/:_authToken $NPM_TOKEN` once in the session, or rely on `npm login` having been done (stores auth in `~/.npmrc`).
  - **VS Code Marketplace**: Create a PAT at [dev.azure.com](https://dev.azure.com) (scopes: Marketplace > Manage). In `.env` set `VSCE_PAT=<your-pat>`. `vsce publish` uses this. See the Azure DevOps / Marketplace setup in Stage 1 below.
  - **PowerShell Gallery**: Create an API key at [powershellgallery.com/account/apikeys](https://www.powershellgallery.com/account/apikeys). In `.env` set `PSGALLERY_API_KEY=<your-key>`. `Publish-Module` uses this.
- [ ] **CI (GitHub Actions)**: Do **not** put tokens in the repo. Add them as **repository secrets**: Settings → Secrets and variables → Actions → New repository secret. Use the same names (`CARGO_REGISTRY_TOKEN`, `PYPI_API_TOKEN`, `NPM_TOKEN`, `VSCE_PAT`, `PSGALLERY_API_KEY`) in the workflow so the same checklist applies.

**Summary**: For local and bot-driven publishes, put tokens in **`.env`** in the repo root. For CI, use **GitHub Actions secrets**. `.env.example` in the repo shows the variable names only (no values).

---

## Stage 0: Decisions (before accounts)

### GitHub: personal vs organization

- [x] **Decided**: **Personal** GitHub (**@aguywithai**). Repo lives under your account; good for profile, open source, and linking to podcast or business from the project.
- [ ] Ensure the repo is **public** and has a clear LICENSE file (UniWorld uses MIT).

### Domain and presence (optional but recommended)

- [ ] **Domain**: Acquire **uniworld.world** (or alternate) if you want a canonical landing page. Not required for publishing packages.
- [ ] **Landing**: Plan where the project "lives" publicly: GitHub README only, or GitHub Pages (see below) / uniworld.world linking to repo, podcast, and registries.

### GitHub Pages (if the URL points here)

You can **dress the site up as much as you like**; there is no single required style.

- [ ] **Enable GitHub Pages** for this repo (Settings → Pages): source can be "Deploy from a branch" (e.g. `main` or `gh-pages`) or "GitHub Actions."
- [ ] **Content options**:
  - **Minimal**: Use the repo README as the page (Jekyll default). No extra files.
  - **Custom page**: Add an `index.md` or `index.html` in the branch or in `/docs` (if using "Deploy from branch" with folder `/docs`). You can add intro text, a short blurb about UniWorld, and links to the repo, registries, and **your business site**.
  - **Themed**: Use a Jekyll theme (e.g. in `_config.yml`: `theme: jekyll-theme-minimal`) and optionally override CSS/HTML in `/assets/css/style.scss` and `_layouts/default.html` so the page matches your style.
  - **Fully custom**: Push a static site (HTML/CSS/JS) to the Pages branch; no Jekyll required if you use a static generator or hand-written files.
- [ ] **Add a link to your business site**: In the README or in the Pages landing content, add a line such as "By [Your Name](https://your-business-site.com)" or "More from [Business Name](https://...)." The project stays neutral and open source; the link is your choice.

### Narrative

- [ ] **Positioning**: Decide how you'll describe the project in one line (e.g. "Correct Unicode text handling for every script—bidi, line break, segmentation, normalization—one library, Rust core, Python/JS/C/Go bindings").
- [ ] **Links**: Optional: link from README or Pages to your AI-cohosted podcast or business; keep the project itself neutral and open source.

---

## Stage 1: Accounts and names (one-time)

Do these once; the bot can use tokens afterward for all publishes.

### Registries

| Registry   | Purpose        | Action |
|-----------|----------------|--------|
| **crates.io** | Rust crate     | Log in with **GitHub** (no separate signup). Create an API token at crates.io; add to `.env` as `CARGO_REGISTRY_TOKEN` or run `cargo login` once. |
| **PyPI**      | Python package | Create account at [pypi.org](https://pypi.org). Create a **project-scoped API token**; add to `.env` as `PYPI_API_TOKEN`. |
| **npm**       | JavaScript/WASM| Create account at [npmjs.com](https://www.npmjs.com). Create a token; add to `.env` as `NPM_TOKEN`, or run `npm login` once. |

- [ ] **crates.io**: GitHub linked, API token created; add to **`.env`** (see "Where to add tokens" above) or run `cargo login`.
- [ ] **PyPI**: Account created, project-scoped API token created; add to **`.env`** as `PYPI_API_TOKEN`.
- [ ] **npm**: Account created, token created; add to **`.env`** as `NPM_TOKEN` or run `npm login`.

### Extension marketplaces

| Marketplace | Purpose | Action |
|-------------|---------|--------|
| **VS Code Marketplace** | VS Code extension | Create a publisher at [marketplace.visualstudio.com/manage](https://marketplace.visualstudio.com/manage). You need an **Azure DevOps** account (free, sign up with GitHub or Microsoft account). Then create a **Personal Access Token (PAT)** at [dev.azure.com](https://dev.azure.com) with scope **Marketplace > Manage**. Add to `.env` as `VSCE_PAT`. Publish with `vsce publish`. |
| **PowerShell Gallery** | PowerShell module | Create an account at [powershellgallery.com](https://www.powershellgallery.com) (Microsoft account). Create an **API key** at [powershellgallery.com/account/apikeys](https://www.powershellgallery.com/account/apikeys). Add to `.env` as `PSGALLERY_API_KEY`. Publish with `Publish-Module -NuGetApiKey $env:PSGALLERY_API_KEY`. |

- [ ] **VS Code Marketplace**: Azure DevOps account created; publisher created (e.g. `aguywithai`); PAT created with Marketplace scope; add to **`.env`** as `VSCE_PAT`.
- [ ] **PowerShell Gallery**: Account created; API key created; add to **`.env`** as `PSGALLERY_API_KEY`.

### Name availability

- [ ] **Crate name**: Check [crates.io](https://crates.io) for `uniworld` (or chosen name). Reserve by publishing or by creating an empty crate if the registry allows.
- [ ] **PyPI**: Check [pypi.org](https://pypi.org) for `uniworld` (or chosen name).
- [ ] **npm**: Check [npmjs.com](https://www.npmjs.com) for `uniworld` (or chosen name; scoped e.g. `@yourname/uniworld` is an option).
- [ ] **VS Code Marketplace**: Check [marketplace.visualstudio.com](https://marketplace.visualstudio.com/search?term=uniworld) for `uniworld`.
- [ ] **PowerShell Gallery**: Check [powershellgallery.com](https://www.powershellgallery.com/packages?q=uniworld) for `UniWorld`.

---

## Stage 2: One-time repo and CI setup

### Package metadata

- [ ] **Cargo.toml**: `name`, `version`, `description`, `license`, `repository` (and optionally `homepage`, `documentation`) set for publish.
- [ ] **pyproject.toml** (or equivalent): Same for Python package name, version, description; `maturin` config if using PyO3.
- [ ] **package.json** (for WASM): Same for npm package name, version, description.

### Cross-platform wheels (Python)

Python users need pre-built wheels. You cannot build every platform from one machine.

- [x] **GitHub Actions**: `.github/workflows/release.yml` runs on **release tag** (`v*`) and uses **maturin-action** to publish wheels from **ubuntu-latest, windows-latest, macos-latest** (extend matrix later for Linux aarch64 / Apple Silicon if needed). It downloads UCD test data before `cargo test`, then publishes PyPI, crates.io, and npm (wasm-pack). Manual **workflow_dispatch** runs tests only (no publish).
- [ ] Confirm secrets in the repo: `PYPI_API_TOKEN`, `CARGO_REGISTRY_TOKEN`, `NPM_TOKEN` (same names as `.env.example`).
- [ ] **Token storage**: Store PyPI token as a GitHub Actions secret (e.g. `PYPI_API_TOKEN`); same for npm if publishing from CI. crates.io uses `cargo login` token; for CI you typically use `CARGO_REGISTRY_TOKEN` or the crates.io CI token.

### Test run (recommended)

- [ ] **Test PyPI**: Publish a pre-release or test version to [test.pypi.org](https://test.pypi.org) first (`maturin publish --repository testpypi` or equivalent). Install with `pip install --index-url https://test.pypi.org/simple/ uniworld` and smoke-test.
- [ ] **Tag and CI**: Create a test tag (e.g. `v0.1.0-rc.1`), push, and confirm CI builds and (if wired) publishes to test registries without errors.

---

## Stage 3: Publish day sequence

Human checkpoint: **you** decide when to publish. The bot (or you) can do the rest via CLI and CI.

1. - [ ] **Run full test suite** (e.g. `cargo test`, integration tests). All pass.
2. - [ ] **Bump version** in `Cargo.toml`, `pyproject.toml`, `package.json` (e.g. `0.1.0`).
3. - [ ] **Update CHANGELOG.md** with release notes for this version.
4. - [ ] **Commit and tag** (e.g. `git tag v0.1.0`). Push commits.
5. - [ ] **Human checkpoint**: Review tag and **push the tag** (`git push origin v0.1.0`). This is the point of no return if CI auto-publishes.
6. - [ ] **CI**: On tag push, workflow builds Rust crate, Python wheels, and (if configured) npm package, and publishes to crates.io, PyPI, and npm.
7. - [ ] **Verify**: Open each registry and confirm the new version is live and the README/metadata look correct.

### If you publish manually (no CI)

- [ ] `cargo publish` (from repo root; requires `cargo login`).
- [ ] `maturin publish` (or `twine upload dist/*`) for Python; ensure wheels are built for desired platforms or use CI for that.
- [ ] `npm publish` for JS/WASM (from the package directory).

---

## Stage 4: After first publish

- [ ] **Registry pages**: Spot-check crates.io, PyPI, npm, VS Code Marketplace, and PowerShell Gallery project pages; fix any description or link if needed.
- [ ] **README**: Ensure root README is the one you want on crates.io (crates.io shows the repo README by default).
- [ ] **Landing / website**: Update `_publishing/site/` with links to all registries, the VS Code extension, and the PowerShell module. See `_publishing/site/SITE_CONTENT.md` for the full content plan. Deploy to GitHub Pages or uniworld.world.
- [ ] **Announce**: Optional: link from podcast, blog, or business site; keep messaging consistent with "open source, correct Unicode for every script."

### Extension publishing

- [ ] **VS Code extension icon and thumbnails**: Two candidate images were generated for UniWorld (globe and planet themes) and saved under the project assets. To use one:
  1. Copy your chosen image (e.g. `assets/uniworld-icon-globe.png` or `uniworld-icon-planet.png`) into `extensions/vscode/` as `icon.png`. Marketplace expects 128x128 or 256x256 PNG.
  2. In `extensions/vscode/package.json` add `"icon": "icon.png"` at the top level (next to `"main"`). Include `icon.png` in the `"files"` array if not already covered.
  3. The same image can be used on the website (e.g. in `_publishing/site/` or GitHub Pages) as the project thumbnail or hero image.
- [ ] **VS Code extension**: From `extensions/vscode/`, run `vsce publish` (requires `VSCE_PAT` in `.env` or logged in via `vsce login`). Or publish via CI.
- [ ] **PowerShell module**: From `extensions/powershell/`, run `Publish-Module -Path . -NuGetApiKey $env:PSGALLERY_API_KEY`. Or publish via CI.

---

## Quick reference: what you do vs what can be automated

| You do | Bot / CI can do |
|--------|------------------|
| Create GitHub (and decide personal vs org) | — |
| Create crates.io / PyPI / npm accounts | — |
| **Add tokens**: put them in **`.env`** (repo root, gitignored) for local/CLI; or in **GitHub Actions secrets** for CI. See "Where to add tokens" above. | Run tests, bump version, update CHANGELOG, commit, tag |
| Decide "publish now" and push the release tag | Build wheels, run `cargo publish`, `maturin publish`, `npm publish` (using tokens from .env or secrets) |
| Verify packages on registries | — |
| Acquire domain, set up landing (optional) | — |

---

## File and doc references

- **Publishing overview**: `_publishing/publishing_overview.md` (registry details, bot vs human, CI summary).
- **Project spec and roadmap**: `_development/docs/UniWorld_PROJECT.md`, `.cursor/rules/PROJECT_ROADMAP.mdc`.
- **Showcase and “why UniWorld”**: `docs/UniWorld_Unicode_Showcase_TEST_OUTPUT.md`, root `README.md` (section "Why UniWorld").
- **Static site source**: `_publishing/site/` (HTML/CSS/JS for GitHub Pages).
- **Communications**: `_publishing/communications/` (press copy, social, podcast notes).
- **Extension roadmaps**: `.cursor/rules/VSCODE_EXTENSION_ROADMAP.mdc`, `.cursor/rules/POWERSHELL_MODULE_ROADMAP.mdc`.
