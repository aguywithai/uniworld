# GoDaddy to Porkbun Domain Transfer and Email Migration Guide

**Date**: 2026-02-10  
**Purpose**: Step-by-step guide for transferring .world (and other) domains from GoDaddy to Porkbun, keeping DNS/hosting intact, and migrating Grand Beta email from Hostinger to Proton. For use on a personal Cursor account or when coordinating business vs personal domains.

---

## 1. Overview and Lead Time

### Why Porkbun
- **Renewal**: ~$33/year for .world vs GoDaddy ~$64/year.
- **Included**: Free URL forwarding, email forwarding, WHOIS privacy, DNS (Cloudflare-backed).
- **Transfers**: Add one year to existing expiry; no loss of time.

### Lead Time and 60-Day Lock
- **ICANN rule**: After a domain is *registered* or *transferred*, it is locked for **60 days** at the *new* registrar (you cannot transfer it out again for 60 days). You can still change nameservers, DNS, and renew at the current registrar.
- **When to start transfer**: Start **2–4 weeks before expiry** so the transfer completes before you have to renew at GoDaddy. Transfers often complete in 1–2 days if the losing registrar approves quickly; allow up to **5–8 days** in practice.
- **Minimizing disruption**: If the domain already uses **Hostinger nameservers** (or another third-party DNS), those nameservers **transfer with the domain** to Porkbun. Site and email keep working with no change. If the domain uses **GoDaddy nameservers**, you must either (a) switch to Hostinger nameservers at GoDaddy *before* transferring (recommended), or (b) use Porkbun’s no-downtime method: add the domain’s DNS zone on Porkbun first, then point the domain to Porkbun’s nameservers at GoDaddy, wait 24–48 hours, then complete the transfer. Details in Section 4.

### Account Strategy
- **Personal (Cursor / open source)**: One Porkbun account for aguywithai.world, worldof.world, haimu.world, uniworld.world, and any other personal/OS domains.
- **Business**: A separate **Porkbun account** (e.g. grandbeta.world only, or business domains only) so business and personal are clearly separated. Porkbun does not have a special “business” product; you just create a second account and use it only for business domains.

---

## 2. Domain-by-Domain Plan

| Domain | When | Current setup | After transfer | Notes |
|--------|------|----------------|----------------|--------|
| **grandbeta.world** | Within 1 month (expires soon on GoDaddy) | GoDaddy; points to Hostinger (site + email) | Porkbun (business account); still points to Hostinger for site; email moves to Proton | Transfer domain first; then migrate email to Proton (Section 7). |
| **aguywithai.world** | Later | GoDaddy; may point to Simplecast redirect | Porkbun; DNS points to Hostinger so the *site* is on Hostinger with Simplecast embedded (Section 5) | Email stays on Hostinger for now; move with domain when you transfer. |
| **worldof.world** | August (a few months left on GoDaddy) | GoDaddy; will be Zoho + Hostinger addon | Porkbun; still Zoho MX + Hostinger nameservers (or A records) for the addon | Transfer when convenient before expiry. |
| **2 parked domains** | When renewal date approaches | GoDaddy; no site | Porkbun; no site, just renew there | No DNS to preserve; straightforward transfer. |

**New domains** (e.g. uniworld.world, haimu.world): Register at **Porkbun** from the start; no transfer needed.

---

## 3. Transferring a Domain from GoDaddy to Porkbun (General Steps)

### Before You Start
1. **Confirm** the domain is at least **60 days** old (from registration or last transfer).
2. **Confirm** you have access to the **registrant/administrative email** on the domain (GoDaddy and/or Porkbun will send approval emails).
3. **If the domain uses Hostinger for site or email**: Ensure the domain is already using **Hostinger nameservers** at GoDaddy (see Section 4). Then when it transfers to Porkbun, those nameservers come with it and nothing changes for visitors or email.
4. **If the domain uses GoDaddy nameservers**: Either switch to Hostinger nameservers at GoDaddy first (Section 4), or use Porkbun’s no-downtime method (Section 4, “Method B”).

### Step-by-Step Transfer
1. **At GoDaddy**  
   - Log in → My Products → Domains → select the domain.  
   - Turn **off** “Transfer Lock” / “Domain Lock” (so the domain can be transferred).  
   - Get the **Authorization (EPP) code**: Domain → Transfer domain away from GoDaddy → Send auth code by email (or copy from the page).  
   - **Optional but recommended**: If you have private registration, turn it off temporarily so the transfer completes without delay (you can re-enable at Porkbun after).

2. **At Porkbun**  
   - Log in to the account you want (personal or business).  
   - Go to **Transfer** (porkbun.com/transfer).  
   - Enter the domain name and the authorization code.  
   - Complete checkout (you pay for one year’s renewal; it’s added to your existing time).  
   - If Porkbun asks you to confirm nameservers, choose “Keep existing nameservers” if the domain already points to Hostinger (or your host).  

3. **Approve the transfer**  
   - Check the registrant email for a message from GoDaddy (and/or Porkbun).  
   - Approve/accept the transfer as instructed.  
   - GoDaddy may allow you to approve in the GoDaddy dashboard as well (faster).

4. **After transfer**  
   - Domain appears in Porkbun → Domain Management.  
   - **DNS**: If you kept Hostinger nameservers, nothing to do. If you moved DNS to Porkbun (no-downtime method), manage DNS at Porkbun (see Section 4).  
   - Re-enable **WHOIS privacy** at Porkbun if you use it (free).  
   - **Renewal**: Set auto-renew at Porkbun if you want; otherwise note the new expiry date.

### If Something Fails
- **Wrong auth code**: Get a new one from GoDaddy and re-submit at Porkbun (Transfers → gear icon → resubmit with correct code).  
- **Transfer rejected**: Check that the domain is unlocked and that the registrant email is correct. Contact GoDaddy support to release the domain if needed.  
- **DNS broke**: If you were on GoDaddy nameservers and didn’t pre-build DNS on Porkbun, add the domain at Porkbun and recreate the same A, CNAME, MX records you had at GoDaddy (or point nameservers back to Hostinger and manage DNS there).

---

## 4. Keeping Everything Pointing to the Same Place (No Downtime)

### Method A: Domain Already Uses Hostinger Nameservers (Easiest)
- In GoDaddy, the domain’s nameservers are set to Hostinger’s (e.g. from Hostinger hPanel → your domain → Nameservers: **ns1.dns-parking.com** / **ns2.dns-parking.com** — or whatever your Hostinger plan shows; get the exact values from **hPanel → Websites → your site → Plan details / DNS**).
- When you transfer to Porkbun, **choose “Keep existing nameservers”**. Porkbun will import those; the domain keeps pointing to Hostinger. No downtime for site or email.

### Method B: Domain Still on GoDaddy Nameservers (or You Want DNS at Porkbun)
Use Porkbun’s no-downtime process so DNS is already correct before the transfer completes:

1. **At GoDaddy**: Leave the domain **locked** for now.  
2. **At Porkbun**: Start the transfer (you can use a placeholder auth code if the domain is locked).  
3. **At Porkbun**: Open **Transfers** → find the domain → gear icon under “DNS” → **Manage DNS records**.  
4. **Rebuild the zone**: Copy every DNS record from GoDaddy (A, AAAA, CNAME, MX, TXT, etc.) into Porkbun. If GoDaddy has a “Export zone file” or similar, you can use that; otherwise add records manually.  
   - For **Hostinger site + email**: You need the same A records (or CNAME) and MX records that Hostinger gives you. You can get these from Hostinger’s DNS zone editor or from their “Point domain to Hostinger” instructions.  
5. **At GoDaddy**: Change the domain’s **nameservers** to Porkbun’s:
   - `salvador.ns.porkbun.com`
   - `maceio.ns.porkbun.com`
   - `fortaleza.ns.porkbun.com`
   - `curitiba.ns.porkbun.com`
6. Wait **24–48 hours** for propagation.  
7. **At GoDaddy**: Unlock the domain and get the auth code; at Porkbun, resubmit the transfer with the real auth code.  
8. After the transfer, Porkbun is the registrar and already has the correct DNS; site and email stay up.

### Where to Get Hostinger Nameservers / Records
- **Hostinger**: Log in → **hPanel** → **Websites** → select the site → **Plan details** or **DNS / Nameservers**. Use the nameservers shown there (they can be per-account). For DNS records only (if using Method B), use **Domains** → **DNS Zone** (or equivalent) and copy A, MX, CNAME, TXT as needed.

---

## 5. aguywithai.world: Point DNS to Hostinger (Not Simplecast) Before or After Transfer

**Goal**: The domain should serve your **Hostinger site** (with podcast embedded), not redirect straight to Simplecast.

### Option 1: Point Domain to Hostinger via Nameservers (Recommended)
1. **At GoDaddy** (or later at Porkbun): Open the domain → **Manage DNS** / **Nameservers**.  
2. Change from “Default” or “GoDaddy nameservers” to **Custom** and enter Hostinger’s nameservers (from Hostinger hPanel for the aguywithai.world site).  
3. **At Hostinger**: Ensure the site for aguywithai.world is set up (landing page, /podcast with Simplecast embed, etc.).  
4. **DNS propagation**: Up to 24–48 hours. After that, aguywithai.world loads the Hostinger site; you can then remove any “forwarding to Simplecast” rule so the domain no longer redirects to Simplecast. The podcast is reached via your own page (e.g. aguywithai.world/podcast) with the player embedded.

### Option 2: Keep Registrar DNS, Use A/CNAME Only
- Leave nameservers at GoDaddy (or Porkbun). In the DNS zone, set:
  - **A** record for `@` (and optionally `www`) to Hostinger’s IP (from Hostinger docs or hPanel).
  - Or **CNAME** for `www` to the Hostinger hostname they provide.
- Same result: domain resolves to Hostinger; you build the site and embed Simplecast there.

### When to Do This
- You can do it **before** transferring aguywithai.world to Porkbun (at GoDaddy) or **after** (at Porkbun). Doing it before transfer means when you “keep existing nameservers,” Hostinger is already the source of truth. Doing it after is fine too: transfer first (keeping GoDaddy nameservers if needed to avoid downtime), then at Porkbun switch nameservers to Hostinger or recreate the same A/MX records pointing to Hostinger.

### Email (aguywithai.world)
- You’re keeping Hostinger email for aguywithai for now. When you transfer aguywithai.world to Porkbun, keep Hostinger nameservers (or the same MX records) so email keeps working. When you’re ready to move that email too (e.g. to Proton or leave on Hostinger), you can do it in a separate step; the guide below for Grand Beta email (Section 7) is the same idea.

---

## 6. worldof.world (August) and Two Parked Domains

- **worldof.world**: Before transfer, set it up as you want (Zoho MX for email, Hostinger addon for the one-page site). Use Hostinger nameservers for worldof.world at GoDaddy so that when you transfer to Porkbun you “keep existing nameservers” and nothing changes. Transfer in August (or whenever before expiry).  
- **Two parked domains**: No site, no special DNS. At GoDaddy: unlock, get auth code. At Porkbun: transfer, keep default Porkbun nameservers (or leave as-is). No lead time needed except “before expiry.”

---

## 7. Grand Beta: Moving Hostinger Email to Proton

**Goal**: Replace Hostinger email for **grandbeta.world** with Proton (business) for better security, and get file storage + sync (Proton Drive) similar to Google Drive.

### Proton Options (Researched)

- **Proton Mail – business**
  - **Mail Essentials**: About **$6.99/user/month** (annual). 15 GB per user, 10 addresses per user, custom domain (@grandbeta.world), calendar, mobile apps (iOS/Android), admin console.  
  - **Mail Professional**: Higher tier (e.g. ~$9.99/user/month); more features.  
- **Proton Drive**
  - **Desktop**: Folder sync (like Google Drive) on Windows/macOS; select folders to sync, files sync to the cloud.  
  - **Mobile**: Proton Drive apps for iOS and Android; access and sync files from phone.  
  - **Business**: “Drive Professional” – 1 TB per user, min 2 users (pricing on proton.me/business/drive). For a single user, **Proton Unlimited** (personal) includes 500 GB Drive; for business custom domain you’d look at **Mail Essentials** (email only) or a **Business Suite**–type bundle if Proton offers Mail + Drive for one user.  
- **Most affordable business setup with email + file access**  
  - **Option A**: **Mail Essentials** (~$6.99/user/mo) for @grandbeta.world email only; use **Proton Drive Plus** or **Unlimited** on the same account for storage/sync (check current bundles at proton.me/business and proton.me/drive/pricing).  
  - **Option B**: **Proton Business Suite** (or equivalent bundle) if it includes Mail + Drive for one user and fits budget (check proton.me/business/plans).  
  - **Storage**: Mail Essentials gives 15 GB for email. For file storage/sync, add a Drive plan; Proton Drive has desktop sync and mobile apps so you get “computer file access and folder sync” and phone access.  
- **Practical suggestion**: Sign up at **proton.me/business**, start with **Mail Essentials** for grandbeta.world (custom domain, 15 GB, mobile apps). Add **Drive** (Plus or business) if you need more storage and folder sync; confirm current single-user options on the pricing page.

### Migration Steps (High Level)

1. **Create Proton (business) account** and add custom domain **grandbeta.world** (Proton will show MX and optional SPF/DKIM/DMARC).  
2. **At Porkbun (or current DNS)** for grandbeta.world: Replace Hostinger MX records with Proton’s MX (and any TXT they give).  
3. **Migrate existing mail** (optional): Use Proton’s import tools or forward from Hostinger to Proton for a transition period; then remove Hostinger mailboxes.  
4. **Configure devices**: Install Proton Mail and Proton Drive apps on phone and desktop; sign in with @grandbeta.world.  
5. **When domain is on Porkbun**: MX for grandbeta.world stays pointing to Proton; no change needed for email when you transfer the domain, as long as you keep the same MX records (or use Hostinger nameservers only for the *website*, with MX overridden at Porkbun if you manage DNS there).

### Order of Operations for Grand Beta

1. **Transfer grandbeta.world** from GoDaddy to Porkbun (business account), keeping Hostinger nameservers so the site and current Hostinger email keep working.  
2. **Subscribe to Proton** (Mail Essentials or bundle) and add grandbeta.world.  
3. **Update MX** for grandbeta.world (at Hostinger DNS if nameservers are Hostinger, or at Porkbun if you switch DNS to Porkbun) to Proton’s values.  
4. **Migrate/forward** old mail as needed; then remove or stop using Hostinger mailboxes for @grandbeta.world.

---

## 8. Checklist Summary

- [ ] **Porkbun**: Create personal account (and, if desired, separate business account for grandbeta).  
- [ ] **grandbeta.world**: Before expiry — unlock at GoDaddy, get auth code; transfer to Porkbun (business); keep Hostinger nameservers. Then set up Proton, change MX to Proton, migrate mail.  
- [ ] **aguywithai.world**: When ready — (optional) point DNS at GoDaddy to Hostinger so site + Simplecast are on your site; then unlock, get auth code, transfer to Porkbun; keep Hostinger nameservers. Email stays on Hostinger until you decide to move it.  
- [ ] **worldof.world**: Before August expiry — ensure Zoho + Hostinger addon and Hostinger nameservers; transfer to Porkbun; keep existing nameservers.  
- [ ] **2 parked domains**: Before their renewal — unlock, get auth codes, transfer to Porkbun.  
- [ ] **New domains** (e.g. uniworld.world, haimu.world): Register at Porkbun only.  
- [ ] **Proton**: Choose Mail Essentials (or bundle with Drive); add grandbeta.world; update MX; install Mail + Drive apps; migrate Grand Beta email from Hostinger.

---

## 9. References

- Porkbun transfer: [porkbun.com/transfer](https://porkbun.com/transfer)  
- Porkbun no-downtime transfer: [kb.porkbun.com/article/89](https://kb.porkbun.com/article/89-how-to-transfer-a-domain-to-porkbun-with-no-downtime)  
- Porkbun nameservers import: [kb.porkbun.com/article/117](https://kb.porkbun.com/article/117-will-my-nameservers-be-imported-during-a-transfer)  
- Hostinger nameservers/DNS: hPanel → Websites / Domains  
- Proton for Business: [proton.me/business](https://proton.me/business)  
- Proton Drive: [proton.me/drive](https://proton.me/drive) (desktop sync, mobile apps)

---

*This guide is part of the web/publishing docs; port it to the web-properties repo or keep it in the UniWorld repo for reference when coordinating personal vs business Cursor accounts and domain/email moves.*
