# UniWorld Accounts, Tokens, and Domain Setup

Companion to the Publishing Roadmap. Use this to track what accounts you need, what you have, and the setup steps for each.

---

## Account Inventory

| Service | Purpose | Account/Username | Status |
|---------|---------|-----------------|--------|
| **GitHub** | Source code, CI, Pages | aguywithai | HAVE |
| **Domain registrar** | .world domains (uniworld, haimu, worldof) | Porkbun or Cloudflare preferred; GoDaddy OK year 1 then transfer | See DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md |
| **Email (sean@worldof.world)** | Personal mailbox for worldof.world | Zoho Mail Free or Hostinger Business Email | See DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md |
| **VS Code Marketplace** | Extension publishing | (Azure DevOps PAT) | NEED |
| **PowerShell Gallery** | Module publishing | (NuGet API key) | NEED |
| **crates.io** | Rust crate publishing | (GitHub login) | NEED (when ready) |
| **PyPI** | Python package publishing | (account) | NEED (when ready) |
| **npm** | JS/WASM package publishing | (account) | NEED (when ready) |

---

## 1. GitHub (HAVE)

**Username:** aguywithai

### Create the repository
```powershell
# From the project root
gh repo create aguywithai/uniworld --private --source=. --push
```
Or via github.com -> New Repository -> Name: uniworld -> Private -> Create.

### Personal Access Token (PAT) for Actions
- GitHub Actions work automatically on push (no extra token needed for the repo's own workflows)
- If you need a PAT for other tools: Settings -> Developer settings -> Personal access tokens -> Fine-grained tokens -> Generate
- Scope: `repo` (full control of private repositories)

---

## 2. Domain Registration and DNS

**Registrar choice**: GoDaddy .world renewal is ~$64/yr. **Porkbun** (~$33/yr) and **Cloudflare** (at-cost) are cheaper and support redirects + DNS. See **DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md** for full comparison. Steps below use "registrar" generically; substitute GoDaddy, Porkbun, or Cloudflare as chosen.

### Purchase uniworld.world (and haimu.world, worldof.world as needed)
1. At your chosen registrar, search for `uniworld.world` (and `haimu.world`, `worldof.world`)
2. Purchase. Porkbun/Cloudflare: lower renewal; GoDaddy: often cheap first year, then consider transfer before renewal
3. Complete checkout

### Point Domain to GitHub Pages
After the repo is public and GitHub Pages is enabled:

1. **In GitHub**: Go to repo Settings -> Pages -> Custom domain -> Enter `uniworld.world` -> Save
   - GitHub will create a `CNAME` file in your repo (or you can add it manually to the Pages source folder)
   - Check "Enforce HTTPS"

2. **In your registrar**: Go to DNS management for `uniworld.world` (e.g. GoDaddy: My Products -> DNS -> Manage; Porkbun/Cloudflare: domain -> DNS)

3. **Add DNS records**:

   | Type | Name | Value | TTL |
   |------|------|-------|-----|
   | A | @ | 185.199.108.153 | 600 |
   | A | @ | 185.199.109.153 | 600 |
   | A | @ | 185.199.110.153 | 600 |
   | A | @ | 185.199.111.153 | 600 |
   | CNAME | www | aguywithai.github.io | 600 |

   These are GitHub Pages IP addresses (verify current IPs at [docs.github.com/pages/custom-domains](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site/managing-a-custom-domain-for-your-github-pages-site)).

4. **Wait for DNS propagation** (can take 15 minutes to 48 hours; usually under 1 hour)

5. **Verify**: Visit `https://uniworld.world` -- should show your GitHub Pages site

### Troubleshooting DNS
```powershell
# Check DNS resolution
nslookup uniworld.world
# Should show GitHub Pages IPs (185.199.108-111.153)

# Check from browser
# Ctrl+Shift+J -> Network tab -> verify no redirect loops
```

---

## 3. VS Code Marketplace (Azure DevOps PAT)

Publishing VS Code extensions requires a **Personal Access Token** from Azure DevOps.

### Steps
1. Go to [dev.azure.com](https://dev.azure.com/)
2. Sign in with your Microsoft account (or create one)
3. Create an organization if you don't have one (any name, e.g. "aguywithai")
4. Click your profile icon (top right) -> Personal access tokens
5. **New Token**:
   - Name: `vsce-publish`
   - Organization: All accessible organizations
   - Expiration: 1 year (max)
   - Scopes: Custom defined -> **Marketplace** -> check **Manage**
6. Click Create, **copy the token immediately** (you won't see it again)

### Create a publisher
```powershell
# Install vsce globally if not already done
npm install -g @vscode/vsce

# Create publisher (one-time)
vsce create-publisher aguywithai
# It will ask for the PAT you just created

# Or login to existing publisher
vsce login aguywithai
```

### Store the token
```powershell
# Option 1: Environment variable (session only)
$env:VSCE_PAT = "your-token-here"

# Option 2: vsce login stores it in your keychain
vsce login aguywithai
```

### Publish
```powershell
cd extensions/vscode
vsce publish
```

---

## 4. PowerShell Gallery (NuGet API Key)

### Steps
1. Go to [powershellgallery.com](https://www.powershellgallery.com/)
2. Sign in with your Microsoft account
3. Click your username (top right) -> API Keys
4. **Create**:
   - Key Name: `uniworld-publish`
   - Expires: 365 days
   - Glob Pattern: `UniWorld` (restricts key to this package)
5. Click Create, **copy the key**

### Store the key
```powershell
# Option 1: Environment variable
$env:PSGALLERY_API_KEY = "your-key-here"

# Option 2: Use directly in Publish-Module
```

### Publish
```powershell
cd extensions/powershell
# Ensure native DLLs are in place (native/win-x64/, etc.)
Publish-Module -Path . -NuGetApiKey $env:PSGALLERY_API_KEY
```

---

## 5. crates.io (Rust - When Ready)

### Steps
1. Go to [crates.io](https://crates.io/)
2. Click "Log in with GitHub" (uses your aguywithai account)
3. Go to Account Settings -> API Tokens -> New Token
4. Name: `uniworld-publish`, copy the token

### Login and publish
```powershell
# Set cargo path if needed
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

cargo login your-token-here
cargo publish
```

---

## 6. PyPI (Python - When Ready)

### Steps
1. Go to [pypi.org](https://pypi.org/) -> Register
2. Enable 2FA (required for new accounts)
3. Account settings -> API tokens -> Add API token
   - Token name: `uniworld`
   - Scope: Entire account (first time) or project-specific after first upload

### Publish
```powershell
# Using maturin (already in requirements)
maturin publish
# It will prompt for your PyPI token
```

---

## 7. npm (JavaScript/WASM - When Ready)

### Steps
1. Go to [npmjs.com](https://www.npmjs.com/) -> Sign Up
2. Username: aguywithai (or preferred)
3. Enable 2FA

### Login and publish
```powershell
npm login
# Follow prompts (opens browser for 2FA)

cd extensions/wasm  # or wherever the npm package is
npm publish
```

---

## Token Security Reminders

- **NEVER commit tokens** to the repository
- Store tokens in environment variables or credential managers
- Use the minimum scope/permissions needed
- Set expiry dates and rotate annually
- For CI, use GitHub repository secrets (Settings -> Secrets and variables -> Actions)

---

## Quick Reference: What to Do When

| When | Accounts needed |
|------|----------------|
| Push to GitHub (private) | GitHub (HAVE) |
| Run CI (Actions) | GitHub (HAVE, automatic) |
| Deploy website | GitHub Pages + GoDaddy (domain) |
| Publish VS Code extension | Azure DevOps PAT (Marketplace) |
| Publish PowerShell module | PowerShell Gallery (NuGet key) |
| Publish Rust crate | crates.io (GitHub login) |
| Publish Python package | PyPI |
| Publish npm package | npm |
