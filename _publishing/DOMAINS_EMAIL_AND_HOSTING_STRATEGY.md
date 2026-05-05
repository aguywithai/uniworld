# Domains, Email, and Hosting Strategy

**Date**: 2026-02-10  
**For**: Final strategy on registrars, email (sean@worldof.world), worldof.world without a repo, HAIMU/UniWorld content split, and SEO. Port to web-properties repo with other publishing docs.

---

## 1. .world Domain Registrar Comparison

### GoDaddy (current assumption)
- **First year**: ~$2.99 (promo)
- **Renewal**: **$63.99/year** (high)
- **Includes**: Domain forwarding/redirect, DNS management
- **Email**: Not included with domain. Separate "Professional Email" or Microsoft 365 (~$2–6/mo per mailbox).

**Verdict**: Fine for year-one grab; renewal is expensive. Strong case to transfer out after the first year.

---

### Porkbun (recommended alternative)
- **First year**: ~$3.60 (sale)
- **Renewal**: **~$33.47/year** (about half of GoDaddy)
- **Includes**: Free URL forwarding (301/302, path preservation, masking), free email *forwarding* (up to 20 addresses), free WHOIS privacy, free SSL, DNS via Cloudflare
- **Transfer**: Transfer in after 60 days at current registrar; includes one-year renewal

**Verdict**: Best balance of price and features. Supports everything you need: redirect (haimu.world -> aguywithai.world/haimu), DNS for GitHub Pages (uniworld.world), and email forwarding if you only need receive-and-forward.

**Limitation**: "Email forwarding" means e.g. sean@worldof.world -> your Gmail. You receive at the custom address but reply from Gmail. For a full mailbox (send and receive as sean@worldof.world) you need a separate email provider (Zoho, Hostinger, etc.).

---

### Cloudflare Registrar
- **Pricing**: At-cost (no markup; you pay registry + ICANN only)
- **.world**: Supported; exact price varies by registry (check domains.cloudflare.com). Typically in the same ballpark as or slightly below Porkbun.
- **Includes**: Free DNS, free DNSSEC, WHOIS privacy, no upsells. No bundled "email forwarding" product; you use MX records with any provider (Zoho, Hostinger, etc.).
- **Redirects**: Domain DNS is on Cloudflare; use Dashboard -> Rules -> Redirect Rules (or Bulk Redirects) to send haimu.world -> aguywithai.world/haimu (301).

**Verdict**: Lowest ongoing cost and transparent pricing. Slightly more manual (redirect via dashboard, email via third party). Good if you want to consolidate domains and avoid renewal surprises.

---

### Pipeline Options

| Approach | When to use |
|----------|-------------|
| **Buy at GoDaddy year 1, then transfer** | If you want the $2.99 first-year price and are okay doing a transfer at ~60 days to Porkbun or Cloudflare. After transfer, renewals at the new registrar (~$33 or at-cost). |
| **Buy at Porkbun from the start** | Simplest: one registrar, ~$33/yr renewal, free forwarding and email forwarding. |
| **Buy at Cloudflare from the start** | If you prefer at-cost and are fine setting redirects in the Cloudflare dashboard and email elsewhere. |
| **Gradually move off GoDaddy** | Transfer uniworld.world, haimu.world, worldof.world to Porkbun or Cloudflare as they come up for renewal (or earlier). No need to rush; do it when convenient. |

**Recommendation**: For *new* .world acquisitions, use **Porkbun** (or Cloudflare if you prefer at-cost). For domains already on GoDaddy, either transfer after 60 days to lock in lower renewal or leave until renewal and then transfer. No need for a repo for worldof.world (see below).

---

## 2. Email: sean@worldof.world

You want a single personal address for basic communication, ideally with decent storage. Options from cheapest to more capable:

### Option A: Zoho Mail Free (cheapest – $0)
- **What**: One custom domain, up to 25 users (you only need one: sean@worldof.world), 5 GB per user.
- **Send/receive**: Full mailbox; send and receive as sean@worldof.world.
- **Setup**: Add domain at Zoho, point MX (and optional SPF/DKIM) at Zoho. Domain can be registered anywhere (GoDaddy, Porkbun, Cloudflare).
- **Caveat**: Some users report occasional deliverability issues with certain providers; for personal/low-volume use it’s usually fine.
- **Cost**: **$0**.

### Option B: Hostinger Business Email Starter
- **What**: 1 mailbox, 10 GB, custom domain. You can attach worldof.world even if the domain is only used for email (or also as an addon on your existing Hostinger hosting).
- **Setup**: In Hostinger, add the domain for email (or add as addon domain); set MX for worldof.world to Hostinger. No repo required.
- **Cost**: ~**$1.59/month** (~$19/year) after promos.

### Option C: GoDaddy Professional Email
- **What**: 10 GB mailbox, custom domain. Separate from hosting; Economy hosting does **not** include mailboxes.
- **Cost**: ~**$2–3/month** depending on plan and region.

### Option D: Email forwarding only (e.g. Porkbun free)
- **What**: sean@worldof.world forwards to Gmail (or another address). You *receive* at sean@worldof.world; replies go out from Gmail unless you use "Send mail as" (which can be clunky).
- **Cost**: **$0** (included with Porkbun/registrar).
- **Verdict**: Fine if you only care about receiving at the custom address; not a true "sean@worldof.world" mailbox.

**Recommendation**: For a real mailbox with no ongoing cost, use **Zoho Mail Free**. If you prefer one less moving part and already use Hostinger, **Hostinger Business Email** for worldof.world is cheap and simple. No need for GoDaddy-specific email unless you want to keep everything at GoDaddy.

---

## 3. worldof.world: No Repo, One Simple Page + Email

You do **not** want a repo for this site. Options:

### Recommended: Hostinger addon domain
- **Domain**: Register worldof.world at Porkbun (or Cloudflare/GoDaddy). Point nameservers to **Hostinger**.
- **Hosting**: In Hostinger, add worldof.world as an **addon domain** on your existing account (same plan that has aguywithai.world). No extra hosting plan.
- **Site**: One folder (e.g. `worldof` or the addon’s document root) with a single `index.html` (and optional `style.css`, one image). No Git, no GitHub Pages, no repo.
- **Email**: Either Zoho Mail Free (MX to Zoho) or Hostinger Business Email for worldof.world (MX to Hostinger). Both work with the domain on Hostinger DNS/nameservers once you point the domain there.
- **Cost**: $0 extra for hosting (addon); only domain + optional email (Zoho $0 or Hostinger ~$1.59/mo).

### Alternative: Registrar + redirect only
- If worldof.world is only a redirect to another URL (e.g. aguywithai.world or a simple link page elsewhere), you only need the domain and a 301 redirect at the registrar (Porkbun/Cloudflare/GoDaddy). No hosting, no repo. You would not have a dedicated "worldof" page unless that target URL is a page on Hostinger (e.g. aguywithai.world/worldof) or elsewhere.

**Recommendation**: Use **Hostinger addon + one static page** so worldof.world is its own minimal hub (one image, links to all properties). No repo, no GitHub. Email: sean@worldof.world via Zoho free or Hostinger Business Email.

---

## 4. HAIMU: haimu.world as URL, aguywithai.world/haimu as Official Space

- **Acquire**: haimu.world (at Porkbun or Cloudflare; or GoDaddy year 1 then transfer).
- **Official content**: Lives at **aguywithai.world/haimu** (on Hostinger). No separate HAIMU repo required.
- **Redirect**: haimu.world -> **https://aguywithai.world/haimu** (301 permanent). Set at registrar (Porkbun “URL Forwarding”) or at Cloudflare (Redirect Rule). GoDaddy: Domain forwarding to that URL.
- **Content split**: Free HAIMU methodology, case studies, and links; consulting/paid services link to **grandbeta.world** (or grandbeta.world/consulting). Close association between HAIMU and A Guy With AI is intentional and acceptable.

### SEO
- **301 redirect**: Passes link equity to the destination; search engines treat the canonical destination as aguywithai.world/haimu. No need to maintain two versions.
- **Canonical**: On the /haimu page, use:
  ```html
  <link rel="canonical" href="https://aguywithai.world/haimu">
  ```
  So search engines consolidate on that URL even if someone links to haimu.world.
- **Internal links**: From aguywithai.world (nav, footer) link to **haimu.world** in text/URLs so users see the clean brand URL; the redirect does the rest. Same in READMEs and elsewhere: "HAIMU (haimu.world)" is fine and reinforces the brand.

---

## 5. UniWorld: Keep uniworld.world Light; Spill to aguywithai

- **uniworld.world**: Built from the UniWorld repo and deployed via **GitHub Pages**. Keep it **light**: hero, problem, feature grid, install cards, quick start, scripts, footer. Single-page, minimal ongoing updates.
- **Heavy or growing content**: Put it at **aguywithai.world/projects/uniworld** (blog posts, deep docs, release notes, case studies). Link from uniworld.world to “More on the project hub” or similar.
- **Source of truth**: Repo (e.g. `docs/` or `_publishing/site/`) for the Pages site; web-properties repo can hold a working copy. No need for a separate “uniworld” repo for the website; the code repo is the source.
- **Result**: uniworld.world stays within GitHub Pages limits and stays fast; anything that needs more space or frequent updates lives under A Guy With AI and keeps a clear template for future projects (landing on .world, depth on aguywithai.world/projects/name).

---

## 6. Summary Table

| Item | Recommendation | Cost (approx) |
|------|-----------------|---------------|
| **.world domains** | Porkbun (or Cloudflare) for new/transferred; avoid GoDaddy renewal | ~$33/yr per domain (Porkbun) or at-cost (Cloudflare) |
| **sean@worldof.world** | Zoho Mail Free, or Hostinger Business Email if you prefer | $0 or ~$19/yr |
| **worldof.world site** | Hostinger addon, one static page; no repo | $0 extra (addon) |
| **haimu.world** | Redirect 301 to aguywithai.world/haimu; content on Hostinger | Domain only |
| **uniworld.world** | GitHub Pages from repo; light content; spill to aguywithai.world/projects/uniworld | Free |
| **GoDaddy** | Use for year-one promo if desired; transfer to Porkbun/Cloudflare before renewal | Year 1 ~$3; then transfer |

---

## 7. What You Need from a Domain Provider

For your setup, the registrar only needs to support:

1. **haimu.world**: Redirect (301) to `https://aguywithai.world/haimu`.  
   → Porkbun, Cloudflare, and GoDaddy all do this (Porkbun: URL Forwarding; Cloudflare: Redirect Rules; GoDaddy: Domain Forwarding).

2. **uniworld.world**: DNS pointing to GitHub Pages (A records and/or CNAME for `www`).  
   → Any registrar with DNS (or use Cloudflare/Porkbun DNS) works.

3. **worldof.world**: Either DNS pointing to Hostinger (for addon + page + email), or just redirect if you later decide worldof is only a redirect.  
   → Same; no special requirement.

So you do **not** need a registrar that “does hosting” or “includes email” for the domains themselves. You need DNS + redirect capability. Email and hosting are separate (Zoho/Hostinger).

---

*This document can be merged or cross-referenced with ACCOUNTS_AND_TOKENS.md and WEB_PROPERTIES_NOTES.md when you finalize the strategy and port to the web repo.*
