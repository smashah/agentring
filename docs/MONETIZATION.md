# Monetization — the gate on going public

Mohammed, 2026-08-10: *"i do want to make money on this so you need to find a way where i can make affiliate money on people buying the rings. so dont public-ize the repo till that is in place. eventually i want to take preorders for an alternative for vocci-ring priced at around $70."*

**The repo stays PRIVATE until an affiliate revenue path is in place and verified.** This file tracks that gate.

## Two revenue lines

1. **Affiliate on ring hardware (now).** Agent Ring is free software that makes a cheap ring useful. Every user needs a ring. So the app and the README point people at rings we earn commission on:
   - The WX02-class page-turner ring is sold on **Amazon** and **AliExpress** under many names. Both have affiliate programs — **Amazon Associates** and the **AliExpress Affiliate / Portals** program — that pay a percentage on referred hardware.
   - Deliverable: identify the exact ring SKUs that work with Agent Ring, enrol in the affiliate programs, and put tagged buy-links in the README, the first-run onboarding ("you'll need a ring — get one here"), and eventually the agentr.ing site. This is the mechanism that must exist before the repo goes public.

2. **Preorders for our own ring (later).** A ~$70 alternative to https://vocci.ai/products/vocci-ring. Agent Ring becomes the software that ships with our own hardware. Preorder capture (email + optional deposit) on agentr.ing. Not blocking v1 of the app; captured here so the architecture leaves room for it.

## Research still owed (see brief)
- Which affiliate program pays best on this hardware, and whether `.ing` / an EU/UK entity affects eligibility.
- The exact ring product(s) to recommend, verified to work with the WX02 profile.
- Whether a short link / storefront (e.g. a Linktree-style or a single Cloudflare Worker redirect with the affiliate tag) is cleaner than raw tagged links.

## Do-not-publish checklist
- [ ] Affiliate program(s) enrolled, tag/ID in hand
- [ ] Recommended ring SKU(s) confirmed working
- [ ] Tagged buy-links in README + onboarding
- [ ] Then, and only then: flip repo public
