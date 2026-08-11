# Agent Ring affiliate plan — #66

**Research date:** 2026-08-11. Sources are numbered and listed at the end.

**Evidence tiers used in this document:**

- **Primary-verified** (live URL checked today): Amazon UK rate card [12], Amazon US rate card [1], Amazon attribution policy [2], Amazon OneLink and application-review guidance [14][15][16][17], the AliExpress agreement [7], Alibaba wholesale and affiliate-program pages [5][18][19][20], eBay Partner Network terms [21][22][23], Amazon product listings, and GOV.UK VAT thresholds [13]. These claims are load-bearing.
- **Third-party-sourced** (affiliate review sites, verified March 2026): AliExpress commission rates, cookie duration, review timing. These are **scenarios, not confirmed economics** — the exact rate is behind the AliExpress portal login and will not be verifiable until Mohammed enrols and sees the dashboard. Nothing in the recommendation depends on the third-party figures being exact.
- **Assumptions** (flagged inline, not from any source): air freight cost, customs duty, realistic referral volumes. These are clearly labelled and are NOT used as the basis for any recommendation.

---

## RECOMMENDATION (read this first)

**Use the newly created Amazon Associates UK account, `agentring-21`, as the first live tag.** It is the **preferred primary** zero-inventory model — not the only one (AliExpress Affiliate below is the same zero-cost model with different geographic fit). A UK Associate tag earns on purchases a buyer makes on **Amazon.co.uk** after clicking from an approved property; **US buyers require a separate Amazon US enrollment and tag** — a UK tag does not earn on Amazon.com purchases. The commission rate is **3%**, the UK "All Other Categories" rate on the public UK rate card [12]. The WX02 ring does not map to a named UK category, so it is **modelled as UK "All Other Categories" pending category confirmation**. At 3% on a ~$15–19 ring, the per-unit commission is small (~$0.45–0.57), but the effort is one enrollment and one tagged link.

**Do not enrol every Amazon locale now.** OneLink is geo-redirection across a defined set of stores, not a worldwide tag [14]. Add the US only when Agent Ring has a verified US SKU and audience. From the US OneLink dashboard, Canada and EU5 can use the simplified single-store-ID controls described by Amazon [15], but the UK documentation still says country accounts must be created separately [16]. Treat that as a documentation mismatch: enable or link Canada, Germany, France, Italy, and Spain only through controls proven in the live dashboard. Japan, Singapore, Netherlands, Saudi Arabia, Poland, Sweden, and Australia explicitly require separate local accounts before their Store IDs can be linked [15].

**Add AliExpress Affiliate after `agentr.ing` is live** for buyers where AliExpress is the preferred or cheaper marketplace. Same zero-cost model. The official agreement confirms that the live portal controls target countries and per-sale pricing [7]; it does not publish a primary-source cookie or WX02 commission. Review sites report 3% on electronics, up to 9% on accessories, and a 3-day cookie [3][4], but every one of those figures remains an **unverified scenario** until Mohammed sees the exact campaign in the portal.

**Add Alibaba.com Affiliate as the global wholesale CTA after the site is live.** One membership exposes affiliate products across 200+ supported countries [18], and the exact WX02 wholesale listing already exists [5]. Current primary rules publish a 60-day cookie and tiered new-buyer/existing-buyer commissions [20]. This is the best global affiliate option for teams, bulk orders, and future branding; it is separate from buying inventory and reselling it ourselves.

**Use eBay only as a conditional local fallback.** EPN has 14 participating affiliate sites [21], but no live listing has been physically verified as WX02 hardware. Add an eBay destination only when a compatible listing exists in a target country; do not model a numeric rate until that listing's category and regional rate card are known [23].

**Do NOT pursue Alibaba bulk resale now.** The unit economics are excellent on paper ($2.91/unit at 100 pcs vs $15–19 retail), but resale requires inventory, shipping, returns handling, customer support, product-liability cover, a storefront, and UK VAT registration once turnover exceeds £90,000 [13]. This is a business, not a link. Capture it as the Phase 2 path to Mohammed's ~$70 own-brand ring, not as the gate-opener.

**Sequenced path:**
1. Use the confirmed UK Store ID/tag `agentring-21`
2. Publish real Agent Ring content at `agentr.ing` before Amazon reviews the submitted properties
3. Identify and physically verify 1–2 UK SKU candidates; then generate direct tagged Amazon links
4. Enrol Alibaba.com Affiliate for the exact WX02 wholesale listing and AliExpress after its portal exposes the real campaign economics
5. Ship the one-page regional product router and tagged links in README + first-run onboarding
6. Confirm all four MONETIZATION checklist gates (see "Sequence that gates public release" below), then flip the repo public
7. Add US/OneLink locales only as verified SKU inventory and traffic justify them
8. *(Much later)* Evaluate bulk-resale / own-brand ring when there is real demand

---

## The three models at a glance

| | Amazon Associates | AliExpress Affiliate | Alibaba bulk + resale |
|---|---|---|---|
| **What it is** | Tagged link to Amazon; commission on sale | Tagged link to AliExpress; commission on sale | Buy rings wholesale, brand them, resell at retail |
| **Commission / margin** | **3% modelled** (UK "All Other Categories" rate [12]; WX02 category pending confirmation) | **3–9% modelled** (third-party-sourced [3][4]; unverified until portal enrollment) | Buy $2.91, sell $15–19 → ~$10–15 gross/unit [5] |
| **Cookie / attribution** | 24-hour session (US policy) [2] | **Reported** 3 days [3] (unverified) | N/A (you own the inventory) |
| **Upfront cost** | £0 | $0 | $291+ (100 units) + shipping + VAT |
| **Inventory risk** | None | None | 100% — unsold stock is your loss |
| **Support burden** | None (Amazon handles) | None (AliExpress/seller handles) | Full: returns, warranty, complaints |
| **Eligibility blocker** | None for UK entity [6] | None for global enrolment [7] | UK VAT reg if turnover > £90k |
| **Time to live** | Low effort — enroll, get tag, paste link | Low effort + portal review wait (manual approval) | High effort — samples, branding, storefront, shipping setup |
| **Clout / shareability** | Low — it is just a buy link | Low | High — "the official Agent Ring" |
| **Preorder fit** | No | No | Yes — this IS the path to the $70 ring |

---

## Model 1 — Amazon Associates

### Commission rate

The WX02 Bluetooth ring does not map cleanly to any single Amazon UK product category. It is **modelled as UK "All Other Categories" at 3.00%** standard commission — the rate that covers anything not explicitly named on the public UK rate card [12]. **This category assignment is pending confirmation after enrollment**, when Mohammed can see exactly which category the WX02 listing falls into; a named category may carry a different rate.

The 3.00% figure is from the Amazon Associates UK Standard Commission Income Statement [12]. The UK program (affiliate-program.amazon.co.uk) operates under Amazon Europe Core S.à r.l. [6]. The revenue figures in this document use 3.00% as the working model; if enrollment reveals a different category for the WX02, they should be recalculated.

For reference, the US rate card [1] lists "All Other Categories" at 4.00%, but the US rate does not apply to Amazon.co.uk purchases and is **not** the basis for the UK model.

### Cookie / attribution window

**24-hour session** [2]. The clock starts on click-through and ends when the customer places an order, clicks another associate's link, or 24 hours elapse — whichever comes first.

If the customer adds an item to their cart within the 24-hour window, the commission holds as long as the order is placed before the cart expires (~90 days) [2].

As of April 14 2026, a new rule requires that products be shipped, streamed/downloaded, and paid for within **180 days** of the qualifying click to earn commission [1b].

### Payout terms

Payment is issued approximately **60 days after the end of each calendar month** in which commission was earned [6]. UK thresholds:

| Method | Minimum |
|---|---|
| Direct deposit | £25 |
| Gift card | £25 |
| Check | £50 |

Source: Amazon.co.uk Associates Program Policies [6].

### UK / EU eligibility

Confirmed: United Kingdom is listed in Schedule 1 of the UK Associates Operating Agreement, contracting entity Amazon Europe Core S.à r.l., site amazon.co.uk [6]. A UK business entity (or individual) can enrol. The `.ing` domain is not restricted — Amazon accepts any website, social media, or app as the "Site" for the program [6].

The program is also available across EU markets (amazon.de, amazon.fr, amazon.it, amazon.es, amazon.nl, amazon.se, amazon.pl, amazon.com.be, amazon.ie) under the same Amazon Europe entity [6]. **However, a single UK enrollment does NOT automatically cover EU storefronts.** Locale-specific programs and store tags require separate confirmation or enrollment — Mohammed should verify which storefronts his UK Associate tag works with before relying on EU commission.

### Worldwide Amazon coverage: OneLink is not a worldwide tag

Amazon's current OneLink-supported stores are the United States, United Kingdom, Canada, Italy, France, Spain, Germany, Japan, Singapore, Netherlands, Saudi Arabia, Poland, Sweden, and Australia [14]. OneLink attempts an exact product match first and may use a close match or search when an identical product is unavailable. Each destination locale applies its own commission rate, accrues earnings in its own programme, and has its own payout threshold [14]. The UK 3% model in this document must not be extrapolated to another storefront.

The current US OneLink guide describes simplified activation for Canada and EU5 (UK, Spain, Germany, France, and Italy) from a US store ID [15]. The UK guide still instructs Associates to create accounts for the countries they want to monetise, including the US and EU5 [16]. This mismatch matters: from the UK home account, do not promise automatic Canada/EU5 provisioning until the live OneLink dashboard exposes and successfully links those controls.

Amazon explicitly requires separate local Associate accounts for Japan, Singapore, Netherlands, Saudi Arabia, Poland, Sweden, and Australia before their Store IDs can be linked and assigned a default tracking ID [15]. Australia is the first later separate-account priority, followed by Japan and Singapore. Netherlands, Sweden, Poland, and Saudi Arabia wait for verified local inventory and traffic. India, UAE, Brazil, Mexico, Belgium, Ireland, Turkey, and Egypt appear as direct Associates programmes but are not on the current OneLink-supported list [14]; they require direct local-program links if demand ever justifies them.

Do not create all of these accounts speculatively. Every separate account carries local tax/payment setup, payout thresholds, and an independent application-review clock. Amazon reviews submitted properties after three qualifying sales within 180 days and expects each property to be public, original, and active; its rule of thumb is about ten posts, generally with recent content, while accepted social pages normally need an established organic following [17].

### Enrollment URL

**https://affiliate-program.amazon.co.uk** — Amazon's confirmation page shows Store ID/tag `agentring-21` with **Success**. Amazon says it reviews the submitted Sites after three qualified sales [17]. The application listed `petrol.now`, `openwa.dev`, and `agentr.ing`; keep `agentr.ing` as the primary promotional property once it is live, and remove `petrol.now` and `openwa.dev` from the Site list before review unless Agent Ring tagged links genuinely appear there. Amazon reviews every submitted Site for public, original, active content, and tagged links must appear only on declared Sites [17]; it does not require every declared Site to discuss Agent Ring. Do not force unrelated cross-site content.

### Effort

- Setup: low — account creation, site verification
- Ongoing: near-zero — generate a tagged link, paste it in the README
- The main task is identifying the right SKU(s) to link to (see below)

### Inventory / fulfilment / returns / support

**All handled by Amazon and the seller.** Zero inventory, zero shipping, zero returns processing, zero customer support. This is the core advantage.

### Compatibility confidence

**Medium.** Several Bluetooth scrolling/page-turner rings are sold on Amazon under multiple brand names (e.g., B0CLDVJPM8, B0CZ487JJG, B0FHW4HXYQ — all listed at $14.99–$18.99) [8][9][10]. **These are compatibility candidates, not verified WX02 hardware** — no SKU is confirmed as the exact same chipset until one is physically tested against the Agent Ring HID profile. The risk is that Amazon delists a specific ASIN and the tagged link breaks — mitigated by linking to a search results page or maintaining 2–3 ASIN links.

### Clout / shareability

Low. A tagged Amazon link is invisible to the buyer — it is just a normal Amazon purchase. There is no "Agent Ring brand" moment. This is a revenue mechanism, not a marketing one.

---

## Model 2 — AliExpress Affiliate / Portals

### Commission rate

AliExpress sets commissions by product category. Rates are published on the AliExpress Portal dashboard (portals.aliexpress.com) after enrollment, not on a public rate card page. Third-party verification (checked March 2026) reports:

| Category | Commission [3][4] |
|---|---|
| Electronics | 3% |
| Accessories | up to 9% |
| Home & Garden | 9% |
| Clothing | 9% |
| Other categories | 7% |

A separate "Hot Products" tier can offer rates up to 90%, capped at $50 commission per order [3]. The WX02 ring is classified as electronics or accessories on AliExpress, so the **reported third-party scenario is 3–9%, unverified until portal** enrollment, depending on the exact listing category.

**Source caveat:** These rates are from third-party affiliate-program review sites (affiliateprogramsguru.com, diggitymarketing.com), verified March 2026 [3][4]. The official AliExpress Affiliate Program Service Agreement [7] states that rates are "prescribed in AliExpress Affiliate Program Advertising Rules and Policies" on the portal, which requires login to view. Mohammed should confirm the exact rate for the specific ring listing after enrollment.

### Cookie / attribution window

**Reported 3 days** [3][4] — third-party-sourced and unverified until portal enrollment. If accurate, this is longer than Amazon's 24-hour session, so a buyer who clicks Monday and buys Wednesday would still attribute.

### Payout terms

- Minimum withdrawal: **$15 USD** (per official agreement: "provided such remitting balance exceeds USD15") [7]
- Payment frequency: **monthly**, with a Net 60 validation period before funds are withdrawable [3]
- Payment method: bank transfer to the participant's designated account (bank handling fees apply per withdrawal) [7]
- Also available via affiliate networks (Awin, CJ Affiliate, Impact, Admitad, Rakuten Advertising) which offer PayPal/direct deposit options [3]

### UK / EU eligibility

The agreement is between the participant and Alibaba.com Singapore E-Commerce Private Limited (AliExpress Global) [7]. It requires the applicant to control the submitted website, app, or social-media property; AliExpress may reject an application. A UK entity or individual can apply, but the live portal determines target countries and campaign availability rather than a public country matrix [7].

### Enrollment URL

**https://portals.aliexpress.com** — accept the Alibaba.com Free Membership Agreement and portal advertising rules, complete account registration, and submit the applicant's name, country, controlled site/media, and contact details [7]. Apply after `agentr.ing` serves real Agent Ring content. `petrol.now` and `openwa.dev` qualify only if they genuinely publish Agent Ring promotion; they must not be unrelated placeholders. Approval timing remains **third-party-reported and unverified** until the portal shows the application state.

### Effort

- Setup: low — application form + portal manual review wait
- Ongoing: near-zero — generate tagged links from the portal's link builder
- API keys are available for deeper integration (dropshipping/affiliate developer API) [3a], but not needed for simple tagged links

### Inventory / fulfilment / returns / support

**All handled by AliExpress and the seller.** Same zero-burden model as Amazon Associates.

### Compatibility confidence

**Medium-high (but unverified).** The WX02 ring originated as an AliExpress/Alibaba product, so finding a listing that looks compatible is straightforward. However, **these are compatibility candidates, not verified WX02 hardware** — the same ring appears under dozens of brand names and product titles, and a specific listing may go offline or use a different chipset revision. No SKU is confirmed until physically tested. Mitigated by linking to a search query or maintaining multiple listings.

### Clout / shareability

Low. Same as Amazon — it is a buy link, not a brand moment.

---

## Model 3 — Alibaba WX02 bulk purchase + branded resale

### The wholesale listing

**Supplier:** Shenzhen Dongxin Trading Co., Ltd. (Guangdong, China). 7-year Alibaba member, 4.5/5 store rating (1,005 reviews), ≥100% on-time dispatch rate, 14% reorder rate. Main markets: Spain, Brazil, Chile, Armenia, United States. [5]

**Listing:** "Wholesale Tiktok Remote BT Ring Remote Video Scrolling Remote Long Battery Life Rechargeable Remote" [5]

**URL:** https://www.alibaba.com/product-detail/Wholesale-Tiktok-Remote-BT-Ring-Remote_1601870411652.html

### Tiered pricing (checked 2026-08-11) [5]

| Quantity | Unit price (USD) |
|---|---|
| 2–99 pcs | $3.14 |
| 100–999 pcs | $2.91 |
| 1,000–9,999 pcs | $2.76 |
| ≥10,000 pcs | $2.45 |

### Specifications (from listing) [5]

- CE Certified
- Battery life: 8 hours
- Material: Plastic
- Charging: Type-C
- Brand name: OEM (supplier accepts custom branding)
- Private mold: Yes (custom tooling available)
- Package: 7 × 7 × 3 cm, 0.100 kg gross weight
- Lead time: 14 days (1–1,000 pcs), 20 days (1,001–5,000), 30 days (5,001–10,000)

### Unit economics at explicit volume tiers

The resale price benchmark is the current Amazon retail price for equivalent Bluetooth scrolling rings: **$14.99–$18.99** [8][9][10]. For the model below I use a conservative $15.00 resale price.

**Assumptions (clearly flagged — not from primary sources):**
- Air freight China→UK: ~$1.50/unit (based on 0.1 kg/package, small-parcel air rates)
- UK import VAT: 20% on (goods + shipping value) — the 20% standard rate is well-established, but the exact application depends on the consignment value threshold (£135) and the seller's setup
- UK customs duty: assumed 0% (consumer electronics/accessories often fall under 0% duty, but this needs verification against the actual HS code for a Bluetooth remote — likely HS 8543.70 — at www.gov.uk/trade-tariff)
- Payment processing: assume ~3% + $0.30 per transaction pending a provider choice
- These assumptions should be verified before committing capital

| Tier | Unit cost | Est. landed cost/unit | Resale @ $15 | Gross margin/unit | Gross margin % |
|---|---|---|---|---|---|
| **10 pcs** | $3.14 | ~$5.50 | $15.00 | ~$9.50 | ~63% |
| **50 pcs** | $3.14 | ~$5.00 | $15.00 | ~$10.00 | ~67% |
| **100 pcs** | $2.91 | ~$4.99 | $15.00 | ~$10.01 | ~67% |
| **1,000 pcs** | $2.76 | ~$4.50 | $15.00 | ~$10.50 | ~70% |

After payment processing (~$0.75/transaction at $15), contribution margin drops to ~$9.25/unit at the 100-pcs tier.

**Total capital required at 100 pcs:** ~$500 (goods + shipping + VAT). Revenue at sell-through: ~$1,500. Gross profit if all sell: ~$1,000.

### Effort

- Setup: high — request samples, evaluate quality, design branding/packaging, set up a storefront (Shopify, Gumroad, or a page on agentr.ing), configure payment processing
- Ongoing: high — order management, shipping, customer support, returns, warranty claims
- Regulatory: UK VAT registration becomes mandatory if turnover exceeds £90,000/year [13]

### Inventory / fulfilment / returns / support risk

**This is the main drawback.** Unlike the affiliate models (zero inventory), bulk resale means:

- **Inventory risk:** 100 unsold rings = ~$300 of stranded capital. The ring is cheap, but the risk is real if demand doesn't materialise.
- **Fulfilment:** You ship every order. Either manually (envelope + Royal Mail) or via a fulfilment service.
- **Returns:** EU/UK distance-selling regulations give buyers 14-day return rights. You absorb return shipping and refund costs.
- **Warranty/product liability:** If a ring malfunctions (battery, Bluetooth pairing), the buyer comes to you, not the Shenzhen supplier. The listing is CE Certified [5], which is the minimum for UK/EU sale, but product-liability insurance is advisable.
- **Customer support:** "My ring won't pair" emails are now your problem.

### UK / EU eligibility

No enrollment needed — you are the seller, not an affiliate. But:
- UK VAT registration if turnover exceeds the threshold
- CE marking is present on the listing [5], required for UK/EU sale
- UKCA marking may be required for UK sale (post-Brexit transition) — this needs verification
- Product-liability insurance is advisable

### Compatibility confidence

**Highest (but still a candidate, not verified).** This listing is the strongest compatibility candidate — the product title, specs, and form factor match the WX02 profile. The supplier lists "private mold: Yes" and "OEM" brand name [5], meaning Mohammed could brand these as "Agent Ring" with custom packaging. **However, no unit from this supplier has been physically tested against the Agent Ring app.** The clout/shareability play is real, but compatibility must be confirmed with a sample order before committing capital.

### Clout / shareability

**Highest.** This is the only model where Mohammed owns the product. "The official Agent Ring" — branded hardware that ships with the software. This is the stepping stone to the ~$70 own-brand ring from `docs/MONETIZATION.md`. An affiliate link cannot do this; a branded resale product can.

### Preorder relationship

This model IS the bridge to the preorder line from `docs/MONETIZATION.md`. The sequence:
1. Start with affiliate links (Model 1/2) to validate demand
2. If demand exists, order 100 branded rings (Model 3) and sell at $15–25
3. Use the sales data and customer list to justify a custom-mold ring at ~$70 (the preorder line)
4. Preorder capture (email + optional deposit) on agentr.ing for the $70 ring

---

## Global affiliate extensions — same #66 decision

These channels extend the three core business models; they do not create a fourth product strategy. Amazon and AliExpress remain zero-inventory retail referral paths, Alibaba.com Affiliate adds a zero-inventory wholesale referral path, and eBay is a conditional marketplace fallback.

### Alibaba.com Affiliate — global wholesale CTA

Alibaba.com Affiliate advertises CPS, CPI, and KOL campaigns, 200M+ affiliate products, and support across 200+ countries from one membership [18]. Registration supports companies and individuals: a company supplies its registered name, company ID, and tax ID; an individual supplies the name matching their ID. Both require email verification, a completed profile, and agreement approval. A supplier portal is needed for withdrawal, not for promotion [19].

The commission rules effective 2026-07-01 publish these order-based rates [20]:

| Buyer/order | Under $5,000 | $5,000–$10,000 | Over $10,000 |
|---|---:|---:|---:|
| **New buyer** | 8% | $300 | $400 |
| **Existing buyer** | 3% | $100 | $200 |

The current cookie is 60 days, and a qualifying order must reach `Trade Completed` [20]. Traffic and orders from Mainland China, India, Russia, Nigeria, Cuba, Iran, North Korea, Syria, and Ukraine are excluded [20]. These published economics apply to the Alibaba.com Affiliate programme, not to AliExpress and not to branded resale.

Use this programme for the exact WX02 wholesale listing [5] and team, event, agency, or own-brand intent. It is not the primary single-ring retail CTA. Enrolment can begin with a real individual or company identity and controlled media, but `agentr.ing` should be live first so the reviewed property matches the promotion.

### eBay Partner Network — conditional local fallback

EPN supports 14 participating affiliate sites: Australia, Austria, Belgium, Canada, France, Germany, Ireland, Italy, Netherlands, Poland, Spain, Switzerland, United Kingdom, and United States [21]. eBay's wider marketplace reaches more countries, but that is not the same as affiliate-storefront coverage.

Signup requires an eBay account, company information, payout currency, and the web, mobile, or social properties used for promotion [22]. Buy It Now attribution uses a 24-hour window, while commission depends on the destination region and product-category rate card [23]. No numeric WX02 rate is verified, so this plan does not model one. Add an eBay destination only after a live target-market listing has been physically verified as compatible WX02-class hardware.

---

## Revenue comparison at realistic sales volumes

**Assumption:** "Realistic sales volumes" for a free, open-source macOS/Windows utility app in its first year are modest. A reasonable range is 50–500 ring referrals/year, driven by GitHub stars, README links, and first-run onboarding.

### Amazon Associates (3% on $15 ring — UK rate card [12])

| Referrals/year | Commission/year |
|---|---|
| 50 | $22.50 |
| 100 | $45.00 |
| 250 | $112.50 |
| 500 | $225.00 |

### AliExpress Affiliate (reported 3% on ~$8 AliExpress listing — third-party scenario, unverified until portal)

| Referrals/year | Commission/year |
|---|---|
| 50 | $12 |
| 100 | $24 |
| 250 | $60 |
| 500 | $120 |

*Note: AliExpress listing prices for the same ring are lower than Amazon (~$5–10), so the absolute commission is lower despite the similar percentage. The 9% accessories rate, if it applies, would yield $36–$360 at the same volumes.*

### Alibaba bulk resale (100 units at $2.91, sell at $15)

| Sell-through | Revenue | Gross profit |
|---|---|---|
| 50 units | $750 | ~$500 |
| 100 units | $1,500 | ~$1,000 |
| 500 units | $7,500 | ~$5,000 |

**The base scenarios pay roughly $12–$225 yearly (AliExpress at the reported 3% electronics rate is the floor; Amazon at 3% is the ceiling), with the AliExpress 9% accessories scenario reaching $360 at 500 referrals — but these are reported third-party scenarios, unverified until portal enrollment. Bulk resale pays $500–$5,000 per batch — but requires capital, labour, and risk.**

The recommendation stands: start with affiliate (free, low-effort, one of four checklist gates), evaluate bulk resale when demand is proven. **Enrollment alone does not unblock public release** — all four MONETIZATION checklist gates must be satisfied (see below).

---

## One product page and regional link routing

### Recommendation: direct destination URLs, not an Amazon redirect

Use one Agent Ring product page with one visible primary **Buy** button and one optional **Bulk / branded** button. Keep the destination data in a server-side or deployment-time mapping keyed by product and country, with `program`, vendor URL or ASIN/listing ID, locale tag, `verifiedAt`, `enabled`, and `priority` fields. Resolve country from an explicit visitor override first and Cloudflare's `cf.country` second; the visitor should never have to navigate a visible country-by-country link matrix.

For Amazon, render the final locale-specific Amazon affiliate URL directly into the Buy anchor before the click. Amazon says OneLink works with full Amazon links and Amazon short links, while third-party shortened URLs do not redirect through OneLink [14]. Do not send an Amazon click through `agentr.ing/ring`, another third-party shortener, or a redirect Worker. Use OneLink only inside its 14-store boundary, verify the destination with Amazon's Check Matching Products tool, and remember that a close match can be a similar product or a search rather than the exact ring [14].

AliExpress and eBay destinations should likewise use the portal-generated tagged URL after the exact listing and campaign are verified. The separate Bulk / branded button should point directly to the Alibaba.com tracked WX02 listing. If no verified local SKU and affiliate link exist, show an unmonetised search or notify-me state rather than sending a visitor to a wrong product or locale tag.

Record click analytics separately with a beacon, then allow normal anchor navigation to the already-rendered vendor URL. This keeps measurement independent from destination routing and preserves the direct-link behaviour Amazon requires for OneLink. The app's onboarding should open the Agent Ring product page in the system browser; the page, not the compiled app, owns current regional link configuration.

| Buyer country | Launch routing rule |
|---|---|
| United Kingdom | Direct Amazon.co.uk link with the UK tag and a physically verified UK SKU |
| United States | Direct Amazon.com link only after a US account, tag, and verified US SKU exist |
| Canada and EU5 | Enable only through OneLink controls proven in the live dashboard, then verify the exact destination |
| JP, SG, NL, SA, PL, SE, AU | Direct local-program links only after separate enrollment, local stock, and traffic justify them |
| Other retail markets | Verified AliExpress campaign when available; otherwise an unmonetised search or notify-me state |
| Worldwide bulk intent | Direct Alibaba.com Affiliate URL for the exact WX02 wholesale listing |

Any page containing tagged links should carry a clear disclosure such as: *"Agent Ring may earn a commission from purchases made through links on this page."*

---

## Confirmed enrollment outcome and next human account action

The confirmation surface proves that the Amazon Associates UK account was created successfully with Store ID/tag `agentring-21`. Amazon says its team reviews the submitted Sites after three qualified sales [17]. No additional EU locale was submitted.

The already-open tab is parked at Amazon's next account boundary: **Enter Payment and Tax Information**, with **Now** and **Later** choices. Mohammed must choose when to begin that financial/tax setup; an agent must not choose the timing or complete a tax interview on his behalf.

Keep `agentr.ing` as the primary declared promotional property once it is live. Remove `petrol.now` and `openwa.dev` from the Site list before review unless Agent Ring tagged links genuinely appear on them. Then generate a direct tagged link only for a physically verified UK ring SKU. Receiving the tag starts the review clock but does not complete the remaining product gates below.

---

## Sequence that gates public release

Per `docs/MONETIZATION.md`, the do-not-publish checklist:

- [x] Affiliate program enrolled, tag/ID in hand: `agentring-21` ← **ONE of four gates, not the only one**
- [ ] Recommended ring SKU(s) confirmed working ← **identify 1–2 ASIN candidates, then test**
- [ ] Tagged buy-links in README + onboarding ← **paste the tagged link**
- [ ] Then, and only then: flip repo public

### SKU candidates (Amazon.com, checked 2026-08-11)

These are all Bluetooth scrolling/page-turner rings that **appear to be** in the WX02 hardware class — **compatibility candidates, not verified WX02 hardware.** No SKU is confirmed until physically tested against the Agent Ring app:

| ASIN | Title | Price | Source |
|---|---|---|---|
| B0CLDVJPM8 | TikTok Scrolling Ring Remote, Bluetooth Page Turner | $14.99 | [8] |
| B0CZ487JJG | Bluetooth Scrolling Ring for TikTok, Page Turner | $18.99 | [9] |
| B0FHW4HXYQ | TikTok Scrolling Ring Remote, Bluetooth Page Turner | $14.99 | [10] |

**Note:** These are US Amazon listings. Mohammed needs to find the equivalent ASINs on Amazon.co.uk for the UK tagged link. The same products exist on Amazon UK (e.g., B0D1Y4Y4RN — TikTok Scrolling Ring, Bluetooth Remote Control [11]) but the price was not captured in this session. Mohammed should search "TikTok scrolling ring" on Amazon.co.uk and pick the highest-rated listing with Prime delivery.

### Compatibility caveat

These rings are **compatibility candidates** — they appear to be the same WX02-class hardware based on product photos, specs, and price point, but **no SKU has been physically tested against the Agent Ring app.** The HID reports (digitizer swipes on usage page 0x0D/0x05, consumer keys on 0x0C/0x01) are expected to be produced by the same chipset across these listings, but batch variation and chipset revisions are possible. The app should be tested against at least one specific ring before claiming compatibility. The current dev ring is one sample; it does not certify every listing.

---

## What this plan does NOT do

- Does not claim Amazon's application review is complete; account creation succeeded, but review follows three qualified sales
- Does not flip the repo public (gated on enrollment + tagged links)
- Does not modify the README or onboarding code (that is implementation, not research)
- Does not order any inventory (Model 3 is deferred)
- Does not tick any checklist items in `docs/MONETIZATION.md` that are not evidence-confirmed

---

## Sources

All URLs checked 2026-08-11.

- **[1]** Amazon Associates US Standard Commission Income Statement: https://affiliate-program.amazon.com/help/node/topic/GRXPHT8U84RAYDXZ — "All Other Categories: 4.00%"
- **[1a]** Amazon Associates US Onsite Commission Income Statement: https://affiliate-program.amazon.com/help/node/topic/G4ARBJC7Z2NK48CA — onsite rates (lower than standard)
- **[1b]** Amazon Associates Operating Agreement changes (April 14 2026): https://affiliate-program.amazon.com/help/operating/compare — "Added 180-day time limit requirement"
- **[2]** Amazon Associates 24-hour session help: https://affiliate-program.amazon.com/help/node/topic/G9SMD8TQHFJ7728F — "within 24 hours of their arrival at Amazon.com via your Associates link"
- **[3]** AliExpress Affiliate Program review (affiliateprogramsguru.com, verified March 2026): https://affiliateprogramsguru.com/programs/aliexpress/ — "Commission: 0-9%... Cookie Duration: 3 days... Min. Payout: $16 USD"
- **[3a]** AliExpress Affiliate Program 2026 guide (freshstore.com, July 2026): https://blog.freshstore.com/aliexpress-affiliate-programme-2026/ — enrollment steps, API keys, tracking ID
- **[4]** AliExpress Affiliate Program review (diggitymarketing.com): https://diggitymarketing.com/best-affiliate-programs/aliexpress/ — "Electronics: 3%, Accessories: 9%"
- **[5]** Alibaba wholesale listing: https://www.alibaba.com/product-detail/Wholesale-Tiktok-Remote-BT-Ring-Remote_1601870411652.html — tiered pricing, specs, supplier details
- **[6]** Amazon.co.uk Associates Program Policies: https://affiliate-program.amazon.co.uk/help/operating/policies — UK payout minimums (£25 deposit), Schedule 1 entity (Amazon Europe Core S.à r.l.)
- **[7]** AliExpress Affiliate Program Service Agreement (effective April 1 2025): https://cdn.contract.alibaba.com/terms/b_platform_service_agreement/20250305142526766/20250305142526766.html — registration and controlled-media requirements, portal-defined campaign economics, "$15 USD" minimum withdrawal, and payment terms
- **[8]** Amazon.com listing B0CLDVJPM8: https://www.amazon.com/SSOBZELR-Trending-Bluetooth-Scrolling-Wireless/dp/B0CLDVJPM8 — $14.99
- **[9]** Amazon.com listing B0CZ487JJG: https://www.amazon.com/dp/B0CZ487JJG — $18.99
- **[10]** Amazon.com listing B0FHW4HXYQ: https://www.amazon.com/Scrolling-Bluetooth-Android-Hands%E2%80%91Free-Rechargeable/dp/B0FHW4HXYQ — $14.99
- **[11]** Amazon.co.uk listing B0D1Y4Y4RN: https://www.amazon.co.uk/Scrolling-Bluetooth-Control-Android-Scroller-Blue/dp/B0D1Y4Y4RN — TikTok Scrolling Ring (price not captured)
- **[12]** Amazon Associates UK Standard Commission Income Statement: https://affiliate-program.amazon.co.uk/help/node/topic/GRXPHT8U84RAYDXZ — UK "All Other Categories: 3.00%" (primary UK rate card, the basis for the 3% model in this document)
- **[13]** GOV.UK — How VAT works: VAT thresholds: https://www.gov.uk/how-vat-works/vat-thresholds — UK VAT registration threshold £90,000
- **[14]** Amazon Associates UK — OneLink overview and supported stores: https://affiliate-program.amazon.co.uk/help/node/topic/G8JHEWQ9GTDUN7EH — supported-store boundary, matching behaviour, direct-link requirements, locale-specific earnings
- **[15]** Amazon Associates US — OneLink setup guide: https://affiliate-program.amazon.com/help/node/topic/G2L3ZBRGXTS7EMEY — simplified Canada/EU5 controls and separate-account linkage for JP, SG, NL, SA, PL, SE, and AU
- **[16]** Amazon Associates UK — OneLink country-account setup: https://affiliate-program.amazon.co.uk/help/node/topic/GKHRXG4YEJBTCAFC — UK guidance to create accounts for each monetised country
- **[17]** Amazon Associates UK — application review process: https://affiliate-program.amazon.co.uk/help/node/topic/G8TW5AE9XL2VX9VM — three qualified sales in 180 days and submitted-site quality/activity guidance
- **[18]** Alibaba.com Affiliate — programme overview: https://ads.alibaba.com/welcome.htm — CPS/CPI/KOL, 200M+ affiliate products, 200+ supported countries
- **[19]** Alibaba.com Affiliate — registration guide: https://ads.alibaba.com/help/register.htm?id=1 — individual/company registration fields, verification, approval, and withdrawal setup
- **[20]** Alibaba.com Affiliate — commission rules effective 2026-07-01: https://ads.alibaba.com/help/commission.htm?id=2 — new/existing-buyer tiers, 60-day cookie, order completion, and excluded traffic
- **[21]** eBay Partner Network agreement and participating sites: https://partnernetwork.ebay.com/page/network-agreement — 14 affiliate storefronts
- **[22]** eBay Partner Network — joining guide: https://partnernetwork.ebay.com/solutions/joining-the-ebay-partner-network — account, business, currency, and promotional-property requirements
- **[23]** eBay Partner Network — rate card: https://partnernetwork.ebay.com/our-program/rate-card — regional/category commissions and 24-hour Buy It Now attribution

---

## Assumptions (flagged, not load-bearing)

These are clearly labelled in the text above and are NOT used as the basis for any recommendation:

- Air freight cost China→UK: ~$1.50/unit (assumption, not sourced)
- UK customs duty rate: assumed 0% pending HS code verification at www.gov.uk/trade-tariff
- UK VAT registration threshold: £90,000/year [13]
- UKCA marking requirement: needs verification for post-Brexit electronics sale
- Realistic annual referral volumes (50–500): assumption based on typical OSS app adoption, not market research
- Resale price of $15.00: based on observed Amazon listings [8][9][10], actual resale price is Mohammed's decision
