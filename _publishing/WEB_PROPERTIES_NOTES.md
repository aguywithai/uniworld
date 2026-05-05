# Web Properties: Page Purposes, Linking Architecture, and Development Notes

**Date**: 2026-02-10
**For**: Porting to the web-properties repo alongside WEB_PROPERTIES_RULES.mdc
**Author**: Sean MacNutt

This document details what each web property IS, what it DOES, how they link together,
and practical notes for building them. Use this alongside the rules document which
covers hosting constraints and directory structure.

---

## 1. uniworld.world

### Purpose
The public face of the UniWorld project. A single-page static site that explains what
UniWorld is, why it matters, how to get it, and what tools are available. This is the
URL printed in every README, every registry listing, every social post.

### Hosting
**GitHub Pages** from the `aguywithai/uniworld` repo (free). Custom domain configured
in repo Settings -> Pages -> Custom domain. CNAME file in the repo's docs/ or
publishing/site/ folder.

### Content Summary
- Hero with icon, tagline, summary paragraph, CTA buttons (GitHub, VS Code, PowerShell)
- Problem statement (affects everyone: emoji, accents, bidi, CJK, Thai)
- Feature grid (8 algorithms with conformance numbers)
- Install cards (Rust, Python, JS, C, Go, VS Code, PowerShell)
- Quick start code examples
- Scripts covered visual
- Footer with attribution and ecosystem links

### Source of Truth
Content plan: `_publishing/site/SITE_CONTENT.md` (in the UniWorld repo)
Design brief: `_publishing/MARKETING_AND_WEBSITE_DESIGN.md` (in the UniWorld repo)
The web-properties repo's `uniworld/` folder is a working copy for development; the
actual deployment goes through the UniWorld repo.

### Keep It Light; Spill to aguywithai
- uniworld.world stays **light**: hero, problem, features, install, quick start, scripts, footer. Single page.
- Content that **grows or needs frequent updates** (blog, deep docs, release notes) lives at **aguywithai.world/projects/uniworld**. Link from uniworld.world to that hub. This keeps Pages within limits and establishes the template for future projects (landing on .world, depth on aguywithai).

### Links OUT from this page
- GitHub repo: https://github.com/aguywithai/uniworld
- VS Code Marketplace: https://marketplace.visualstudio.com/items?itemName=aguywithai.uniworld
- PowerShell Gallery: https://www.powershellgallery.com/packages/UniWorld
- crates.io: https://crates.io/crates/uniworld
- PyPI: https://pypi.org/project/uniworld/
- npm: https://www.npmjs.com/package/uniworld
- A Guy With AI: https://aguywithai.world (publisher, podcast)
- HAIMU: https://haimu.world (methodology)
- Grand Beta: https://grandbeta.world (funding)
- All projects: https://worldof.world
- Integration guides: (relative links within GitHub repo)
- Unicode Showcase: (relative link within GitHub repo)

### Links IN to this page
- Every UniWorld README (root, VS Code, PowerShell, integration docs)
- Every registry listing (Marketplace, Gallery, crates.io, PyPI, npm)
- aguywithai.world/projects (project showcase)
- aguywithai.world/haimu (case study reference)
- worldof.world (project listing)
- Social posts, press releases, blog posts
- grandbeta.world portfolio

### Design Constraints (GitHub Pages)
- Static HTML/CSS/vanilla JS only
- < 1 GB total site size (will be well under)
- No forms, no server-side, no analytics
- Prism.js OK for syntax highlighting (~20KB)
- CNAME file required for custom domain
- System font stack (covers all scripts natively)

### Development Notes
- Build from SITE_CONTENT.md section by section
- Use existing style.css as base (dark navy, off-white, muted blue, warm orange)
- Single-page scrolling layout with section IDs for anchor navigation
- Mobile-responsive (media queries for feature grid)
- Hero image: icon.png (tech globe with orbiting scripts)
- Code blocks: dark background, monospace, optional Prism.js
- Test locally with `python -m http.server` in the site folder
- Deploy: copy files to uniworld repo's docs/ folder, push, verify at uniworld.world

---

## 2. aguywithai.world

### Purpose
The central hub for Sean MacNutt's open-source development work and podcast. This is
the "A Guy With AI" brand landing page. It hosts the podcast landing/embed, project
showcases, the HAIMU methodology page (at /haimu), and mailing list signups.

This is the CONTENT HOST for anything that outgrows a simple landing page. Other
domains (haimu.world, potentially others) redirect here for their detailed content.

### Hosting
**Hostinger** (already owned, good limits). Deploy via Git auto-deploy or FTP/SFTP.

### Content Summary

#### Root (aguywithai.world/)
- Landing page: "A Guy With AI" brand introduction
- Sean MacNutt, open-source developer and AI methodology practitioner
- Navigation to: Podcast, Projects, HAIMU, About
- Links to worldof.world, haimu.world, grandbeta.world

#### /podcast (aguywithai.world/podcast)
- **Your content landing** for the podcast: show description, episode highlights,
  subscribe links (Apple, Spotify, etc.), mailing list signup, about the host.
  This is NOT Simplecast; this is your own page with full editorial control.
- **Simplecast embed or link**: The actual player/feed lives at
  **aguywithai.world/podcast/listen** (or /podcast/episodes). This can be:
  - A Simplecast embedded player (`<iframe>` embed code from your Simplecast dashboard)
  - Or a link/redirect to the Simplecast-hosted page if embedding is limited
- **Routing**: Currently aguywithai.world redirects to Simplecast entirely. Replace
  this with: aguywithai.world -> your landing page; /podcast -> your podcast landing;
  /podcast/listen -> Simplecast embed or redirect. You keep ownership of the landing
  and SEO while Simplecast handles audio hosting and distribution.
- **Simplecast setup**: In Simplecast dashboard, your show's "custom website URL"
  can stay pointed at aguywithai.world/podcast. For embedding, get the embed code
  from Simplecast -> Episodes -> Share/Embed. Place it in an `<iframe>` on the
  /podcast/listen page. Simplecast does not need to "know" about the routing; it
  just provides the embed widget.

#### /projects (aguywithai.world/projects)
- Showcase of open-source projects
- UniWorld card linking to uniworld.world
- Future projects get cards here as they launch
- Each card: project name, one-line description, icon, link to project's .world domain

#### /projects/uniworld (dedicated page -- NOT a redirect)
- **A Guy With AI's perspective on UniWorld**: the development story, HAIMU case study,
  deeper dev notes. This is the "spill" location for content that grows beyond
  uniworld.world's light GitHub Pages landing.
- **Content plan**:
  - How UniWorld was conceived: HAIMU prompted for the highest-ROI neglected technical
    benefit projects; correct Unicode handling emerged as the clear winner.
  - Development timeline: library largely built within 14 hours of idea generation.
    "Move fast and fix things." -- Sean MacNutt.
  - HAIMU in practice: how the methodology guided code, architecture, testing, and
    publishing preparation. Link to haimu.world for the methodology itself.
  - Release notes, blog-style updates, and deeper technical discussion as the project
    evolves. This is where ongoing content lives.
- **Links**: uniworld.world (project landing), haimu.world (methodology),
  GitHub repo, registries.
- **Template**: This pattern (landing on .world, depth on aguywithai.world/projects/name)
  is reusable for future A Guy With AI projects.

#### /haimu (aguywithai.world/haimu)
- HAIMU methodology page (haimu.world 301 redirects here). **Official space** for HAIMU; haimu.world is the brand URL only.
- What is HAIMU: Human-AI Mutual Understandability
- Sean MacNutt's AI-assisted development methodology
- Key principles, process description
- Case studies: UniWorld (library built in 14 hours from idea generation; full ecosystem in days), future projects
- Links to projects that used HAIMU
- **Free HAIMU content** here; link to **grandbeta.world** (or /consulting) for consulting/paid services. Close linkage between HAIMU and A Guy With AI is intentional.
- **SEO**: Use `<link rel="canonical" href="https://aguywithai.world/haimu">` on this page so search engines consolidate on this URL. 301 from haimu.world passes link equity.

#### /about
- About Sean MacNutt
- Background, methodology, philosophy
- Links to worldof.world, grandbeta.world

### Links OUT from this site
- uniworld.world (project)
- haimu.world (methodology -- even though it redirects here, link the clean URL)
- worldof.world (all projects)
- grandbeta.world (business)
- Simplecast (podcast hosting)
- Substack (newsletter, if added)
- GitHub: https://github.com/aguywithai
- Social media profiles (if any)

### Links IN to this site
- haimu.world (301 redirect to /haimu)
- uniworld.world footer ("By Sean MacNutt / A Guy With AI")
- worldof.world (project listing)
- grandbeta.world (portfolio)
- Every UniWorld README footer

### Development Notes
- Hostinger has full capability but keep it simple: HTML/CSS/vanilla JS
- Current redirect to Simplecast should be replaced with a proper landing page
  that embeds the podcast player
- Git auto-deploy: connect Hostinger to the web-properties repo, deploy from
  the aguywithai/ folder
- Subfolders (/haimu, /projects, /podcast) are real directories with index.html files
- Shared header/footer components can be included via vanilla JS or build step
- No framework needed; this is a handful of static pages

---

## 3. haimu.world

### Purpose
Clean, memorable URL for the HAIMU methodology. In practice, this domain FORWARDS
to aguywithai.world/haimu where the actual content lives.

### Hosting
**GoDaddy forwarding** (free with domain registration). 301 permanent redirect to
https://aguywithai.world/haimu.

If HAIMU grows into something that needs its own standalone site (e.g., a methodology
framework with its own documentation, certification, community), it can be promoted to:
- A GitHub Pages repo (free, own site)
- A Hostinger addon domain (uses existing hosting)

For now, forwarding is sufficient and costs nothing.

### Setup Steps
1. Purchase haimu.world on GoDaddy
2. In GoDaddy Domain Portfolio -> haimu.world -> DNS -> Forwarding
3. Add forwarding: destination = https://aguywithai.world/haimu
4. Type: 301 Permanent
5. Masking: optional (keeps haimu.world in address bar; may or may not work depending
   on Hostinger's response headers)
6. HTTPS: automatically applied

### Links OUT
- Not applicable (this is a redirect, not a page)
- The destination (aguywithai.world/haimu) handles all outbound links

### Links IN
- Every UniWorld README footer mentions HAIMU with this URL
- uniworld.world footer
- aguywithai.world (nav and footer link the clean URL)
- worldof.world
- Press releases, social posts about methodology

### Development Notes
- No development needed for the domain itself -- it's a redirect
- All content development happens at aguywithai.world/haimu
- If promoting to standalone later, create a `haimu` GitHub Pages repo and update
  GoDaddy DNS from forwarding to GitHub Pages IPs

---

## 4. worldof.world

### Purpose
Sean MacNutt's personal landing page for ALL .world projects, including business.
A simple, elegant link hub -- one image, brief intro, and links to every property.
Think of it as a Linktree-style page but self-hosted and branded.

### Hosting
**Option A (preferred)**: GoDaddy forwarding with masking to a GitHub Pages mini-site.
Create a small `worldof` GitHub Pages repo with a single index.html.

**Option B**: Hostinger addon domain pointing to a /worldof subfolder on aguywithai.world.

**Option C**: Just a GoDaddy redirect to aguywithai.world (simplest but loses the
personal hub identity).

Recommendation: **Option A** -- a tiny GitHub Pages site gives full control, costs
nothing, and establishes the worldof.world identity distinctly from aguywithai.world.

### Content Summary
- Minimal single page
- Sean MacNutt's name/photo or simple branding
- "My .world projects" or similar heading
- Link cards:
  - aguywithai.world -- Podcast and open-source development
  - haimu.world -- HAIMU methodology
  - uniworld.world -- UniWorld: Unicode text handling
  - grandbeta.world -- Grand Beta (business)
  - (future projects added as cards)
- Brief footer with name

### Email: sean@worldof.world
- **No repo** for this site. Hosting: Hostinger addon domain only; one static page.
- **Mailbox**: Zoho Mail Free (5 GB, $0) or Hostinger Business Email Starter (10 GB, ~$1.59/mo). Point MX for worldof.world to chosen provider. See DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md.

### Links OUT
- aguywithai.world
- haimu.world
- uniworld.world
- grandbeta.world
- Any future .world projects

### Links IN
- Every other property's footer ("All projects: worldof.world")
- Personal profiles, social media bios
- Business cards (if applicable)

### Design Constraints
- One page, one image, a handful of links
- Should load instantly
- GitHub Pages constraints apply (if Option A): static only, < 1 GB, no server-side
- Could literally be 50 lines of HTML and 30 lines of CSS

### Development Notes
- Build last or simultaneously with aguywithai -- it's the simplest page
- If using GitHub Pages: create repo `aguywithai/worldof`, enable Pages, add CNAME
- If using Hostinger addon: add worldof.world as addon domain in Hostinger panel,
  point to a /worldof directory
- The GoDaddy DNS for worldof.world would point to GitHub Pages IPs (if Option A)
  or Hostinger nameservers (if Option B)

---

## 5. grandbeta.world

### Purpose
Business entity landing page. Not managed from the personal web-properties repo.
Mentioned here for completeness and cross-linking reference.

### Hosting
Separate (business repo and hosting). Decision on hosting is a business matter.

### Subsidiary Pattern
grandbeta.world/subsidiaryname for subsidiary pages. Keeps the business domain as a
hub with child pages, same pattern as aguywithai.world/projectname for personal projects.

### Links IN from personal properties
- uniworld.world footer: "Funded by Grand Beta"
- aguywithai.world: reference in about/projects sections
- worldof.world: link card

---

## Linking Architecture Summary

```
                        worldof.world
                      (personal hub - links to all)
                     /        |         \          \
                    /         |          \          \
    aguywithai.world    haimu.world   uniworld.world   grandbeta.world
    (content host)      (redirect)    (project site)    (business)
         |                 |
         |       forwards to:
         |    aguywithai.world/haimu
         |
         +-- /podcast
         +-- /projects
         |     +-- /uniworld (redirect to uniworld.world)
         +-- /haimu (HAIMU content; haimu.world lands here)
         +-- /about
```

**Every page footer links to:** worldof.world, aguywithai.world, haimu.world, grandbeta.world

**uniworld.world additionally links to:** GitHub, VS Code Marketplace, PowerShell Gallery,
crates.io, PyPI, npm, integration docs

---

## Domain Acquisition Checklist

See **DOMAINS_EMAIL_AND_HOSTING_STRATEGY.md** for registrar comparison (.world: GoDaddy ~$64/yr renewal vs Porkbun ~$33/yr vs Cloudflare at-cost). Recommended: Porkbun or Cloudflare for new/transferred .world domains.

| Domain | Registrar (suggested) | Estimated Cost (1st year) | Renewal (~) | Action |
|--------|------------------------|---------------------------|-------------|--------|
| uniworld.world | Porkbun or Cloudflare | ~$3-4 (Porkbun sale) or at-cost | ~$33 or at-cost | Purchase, hold (DNS when Pages ready) |
| haimu.world | Porkbun or Cloudflare | ~$3-4 or at-cost | ~$33 or at-cost | Purchase, set 301 forwarding to aguywithai.world/haimu |
| worldof.world | Porkbun or Cloudflare | ~$3-4 or at-cost | ~$33 or at-cost | Purchase; point to Hostinger addon (no repo) |
| aguywithai.world | Already owned | -- | existing | Already on Hostinger |
| grandbeta.world | (business) | (business expense) | -- | Separate |

**Total new domain cost**: ~$10-15 first year if using Porkbun/Cloudflare; renewals ~$99/year for three .world domains. No extra hosting cost (Hostinger addon for worldof; GitHub Pages free; forwarding free).

---

## Development Sequencing

### Phase 1: Domain Acquisition (do now, before or during business prep)
1. Purchase uniworld.world, haimu.world, worldof.world on GoDaddy
2. Set up haimu.world forwarding to aguywithai.world/haimu (even though content isn't there yet -- can forward to aguywithai.world temporarily)
3. Do NOT configure DNS for uniworld.world or worldof.world yet (no sites to point to)

### Phase 2: Build uniworld.world (during UniWorld publishing roadmap Stage 7)
1. Build site from SITE_CONTENT.md in the web-properties repo's uniworld/ folder
2. Copy to the aguywithai/uniworld repo's docs/ folder
3. Enable GitHub Pages, configure custom domain, verify HTTPS
4. This is the template for future project sites

### Phase 3: Build aguywithai.world (can happen anytime)
1. Replace the Simplecast redirect with a proper landing page
2. Build /podcast, /projects, /haimu, /about subpages
3. Deploy to Hostinger
4. Update haimu.world forwarding destination if needed

### Phase 4: Build worldof.world (last priority)
1. Simple link hub page
2. Deploy to GitHub Pages or Hostinger addon
3. Configure DNS

### Future: As new projects launch
1. New project gets a .world domain
2. GitHub Pages for the project site (from its repo)
3. Card added to aguywithai.world/projects
4. Link added to worldof.world
5. Cross-links in all footers

---

## Notes for README Finalization

When finalizing UniWorld READMEs (publishing roadmap Stage 2), ensure:

- **uniworld.world** is prominent in every README with a brief blurb ("Visit
  uniworld.world for the full project, documentation, and ecosystem links")
- **aguywithai.world** appears in attribution ("By Sean MacNutt / A Guy With AI")
- **haimu.world** appears in methodology references
- **grandbeta.world** appears in funding attribution
- **worldof.world** appears in "All projects" links
- All URLs use https:// prefix
- Links to registries use placeholder text until published ("Coming soon" or
  "Available after public release")
- The README links should survive the transition from private to public GitHub --
  relative links for in-repo docs, absolute links for external sites

---

## Coordination with Publishing Roadmap

This document aligns with PUBLISHING_ROADMAP.mdc stages:

| Roadmap Stage | Web Properties Action |
|---------------|----------------------|
| Stage 2 (README alignment) | Ensure all .world URLs are in READMEs |
| Stage 3 (Signups/procurement) | Purchase domains, set up forwarding |
| Stage 7 (Website creation) | Build uniworld.world from SITE_CONTENT.md |
| Stage 8 (Screenshots/polish) | Verify all cross-links resolve |
| Stage 9 (Outreach/promotion) | All sites live, linking correctly |

Between stages 5 (pause for business) and 7 (website), aguywithai.world and
worldof.world can be built independently since they don't depend on UniWorld's
public status.
