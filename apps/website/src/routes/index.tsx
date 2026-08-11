import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Landing,
});

function Landing() {
  return (
    <>
      <a className="skip-link" href="#main">
        Skip to content
      </a>
      <header className="site-header">
        <div className="wrap header-inner">
          <p className="wordmark">
            <span className="wordmark-dot" aria-hidden="true" />
            Agent Ring
          </p>
          <nav aria-label="Page sections">
            <a href="#how-it-works">How it works</a>
            <a href="#gestures">Gestures</a>
            <a href="#status">Release status</a>
            <a href="#support">Support</a>
          </nav>
        </div>
      </header>

      <main id="main">
        {/* Hero */}
        <section className="hero" aria-labelledby="hero-title">
          <div className="wrap hero-grid">
            <div className="hero-copy">
              <p className="eyebrow">A cross-platform ring remapper</p>
              <h1 id="hero-title">
                Remap a Bluetooth finger-ring into any keyboard shortcut.
              </h1>
              <p className="lede">
                Agent Ring is a lightweight native app for macOS, with Windows 11
                planned. It listens to the raw HID reports of a WX02-class ring,
                turns each tap, swipe, and long-press into a gesture, and fires the
                keystroke or media key you mapped — replacing per-device Karabiner
                rules and stuck page-turner behaviour with one menu-bar app.
              </p>
              <div className="status-pills" aria-label="Current release status">
                <span className="pill">
                  macOS · developer build
                  <span className="pill-note">public notarized download pending</span>
                </span>
                <span className="pill pill-muted">Windows 11 · planned</span>
              </div>
              <p className="hero-links">
                <a className="btn" href="#how-it-works">
                  How the gesture pipeline works
                </a>
                <a className="link" href="#status">
                  Release status and roadmap
                </a>
              </p>
            </div>

            {/* CSS-only ring emblem, decorative */}
            <div className="ring-emblem" aria-hidden="true">
              <div className="ring-orbit">
                <div className="ring-band" />
                <div className="ring-core" />
                <span className="gesture-chip chip-up">↑</span>
                <span className="gesture-chip chip-right">→</span>
                <span className="gesture-chip chip-down">↓</span>
                <span className="gesture-chip chip-left">←</span>
                <span className="gesture-chip chip-tap">tap</span>
              </div>
            </div>
          </div>
        </section>

        {/* Why */}
        <section className="why" aria-labelledby="why-title">
          <div className="wrap">
            <h2 id="why-title">Why a dedicated app</h2>
            <p>
              The WX02 ring was built to turn pages on a phone. On a computer it is
              almost useless on its own: quick presses emit synthetic digitizer
              touch swipes (HID usage page 0x0D/0x05) that macOS silently discards
              and that Karabiner cannot see, while long presses emit consumer keys
              such as volume up, volume down, and power.
            </p>
            <p>
              Agent Ring captures the raw HID stream before the operating system
              throws the interesting part away. Mouse-like swipes become shortcuts,
              and the volume keys become ordinary, remappable inputs — so a long
              press no longer changes your system volume behind your back.
            </p>
          </div>
        </section>

        {/* How it works */}
        <section id="how-it-works" className="steps" aria-labelledby="steps-title">
          <div className="wrap">
            <h2 id="steps-title">How gesture-to-shortcut works</h2>
            <p className="section-intro">
              The pipeline is capture, classify, inject. Two working Python
              prototypes proved every stage; the Rust app is the production
              implementation of the same pipeline.
            </p>
            <ol className="step-list">
              <li className="step-card">
                <span className="step-number" aria-hidden="true">
                  1
                </span>
                <h3>Capture</h3>
                <p>
                  Read raw HID reports from the ring over Bluetooth, including the
                  digitizer reports that macOS discards. The app never matches on
                  vendor or product ID alone — the ring spoofs Apple&apos;s IDs — so
                  profiles match the product string, the Bluetooth transport, and
                  the HID usage pages.
                </p>
              </li>
              <li className="step-card">
                <span className="step-number" aria-hidden="true">
                  2
                </span>
                <h3>Classify</h3>
                <p>
                  Turn the raw stream into gestures: tap, swipe up, down, left, or
                  right, plus long-press. Classification follows the proven
                  prototype — tip up and down cycles with a movement threshold
                  decide whether a press was a tap or a swipe.
                </p>
              </li>
              <li className="step-card">
                <span className="step-number" aria-hidden="true">
                  3
                </span>
                <h3>Inject</h3>
                <p>
                  Each gesture fires the action you mapped: any keyboard combo, a
                  media key, or nothing. Injection works across platforms, with
                  Input Monitoring and Accessibility only asked for when macOS
                  actually needs them.
                </p>
              </li>
            </ol>
          </div>
        </section>

        {/* Gestures */}
        <section id="gestures" className="gestures" aria-labelledby="gestures-title">
          <div className="wrap">
            <h2 id="gestures-title">Gestures you can map</h2>
            <p className="section-intro">
              Every gesture is configurable in the settings window: press the ring
              button, the gesture highlights, then pick an action. Example mappings
              below; nothing is hard-coded.
            </p>
            <div className="table-scroll">
              <table>
                <thead>
                  <tr>
                    <th scope="col">Gesture</th>
                    <th scope="col">Example mapping</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>
                      <kbd>Tap</kbd>
                    </td>
                    <td>Enter, page down, or a paste shortcut — one tap, one action</td>
                  </tr>
                  <tr>
                    <td>
                      <kbd>Swipe up</kbd>
                    </td>
                    <td>Option + Space to open Spotlight or Launchpad</td>
                  </tr>
                  <tr>
                    <td>
                      <kbd>Swipe down</kbd>
                    </td>
                    <td>Mission Control or a window snap</td>
                  </tr>
                  <tr>
                    <td>
                      <kbd>Swipe left / right</kbd>
                    </td>
                    <td>Media previous / media next while a player is focused</td>
                  </tr>
                  <tr>
                    <td>
                      <kbd>Long press</kbd>
                    </td>
                    <td>
                      The ring&apos;s volume keys become first-class inputs — remap
                      them to anything without changing system volume
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </section>

        {/* Status */}
        <section id="status" className="status" aria-labelledby="status-title">
          <div className="wrap">
            <h2 id="status-title">Release status</h2>
            <div className="status-grid">
              <article className="status-card">
                <h3>macOS</h3>
                <p>
                  The core engine, tray menu, and native settings window (no
                  webview) are working in the private development chain, and a
                  developer build is running in internal testing. A public,
                  notarized download is not available yet: release builds need a
                  stable Developer ID signing identity and notarization before
                  anything is published.
                </p>
                <p className="status-line">
                  Today: installed developer build — no public download link yet.
                </p>
              </article>
              <article className="status-card">
                <h3>Windows 11</h3>
                <p>
                  Planned, not yet shipping. The open question is how Windows&apos;s
                  raw HID input treats the ring&apos;s touch reports, and that spike
                  needs a physical Windows machine with Bluetooth. Windows support
                  is gated on that check; worst case, Windows v1 ships
                  long-press-only input.
                </p>
                <p className="status-line">Today: no Windows build exists.</p>
              </article>
            </div>
            <ol className="roadmap" aria-label="Release roadmap">
              <li className="roadmap-item">
                <strong>M0</strong>
                <span>Core engine and gesture classifier</span>
                <span className="roadmap-state">done</span>
              </li>
              <li className="roadmap-item">
                <strong>M1</strong>
                <span>macOS backend and tray menu</span>
                <span className="roadmap-state">done</span>
              </li>
              <li className="roadmap-item">
                <strong>M2</strong>
                <span>Settings window with learn mode</span>
                <span className="roadmap-state">done</span>
              </li>
              <li className="roadmap-item">
                <strong>M3</strong>
                <span>Windows spike, then Windows backend</span>
                <span className="roadmap-state">planned</span>
              </li>
              <li className="roadmap-item">
                <strong>M4</strong>
                <span>Signed, notarized releases published</span>
                <span className="roadmap-state">pending</span>
              </li>
            </ol>
          </div>
        </section>

        {/* Support */}
        <section id="support" className="support" aria-labelledby="support-title">
          <div className="wrap">
            <h2 id="support-title">Support Agent Ring</h2>
            <p className="section-intro">
              Agent Ring is being built as a free app. It remaps a WX02-class
              Bluetooth ring — an inexpensive, off-the-shelf product from any of
              the usual stores. The cards below show where each program stands.
              When a tag and a physically verified SKU are both ready, that card
              carries a tagged buy link; until then the Amazon links point straight
              at the store listings so you can see the hardware yourself.
            </p>

            <div className="affiliate-grid">
              <article className="affiliate-card">
                <h3>Amazon UK</h3>
                <p>
                  The recommended first model: Amazon Associates UK handles
                  checkout, shipping, and returns. A UK tag only earns on
                  Amazon.co.uk purchases.
                </p>
                <a
                  className="link card-link"
                  href="https://www.amazon.co.uk/s?k=bluetooth+scrolling+ring+page+turner"
                >
                  Browse Amazon UK listings
                </a>
                <p className="affiliate-state">
                  Account created: agentring-21. Amazon reviews the application
                  after three qualified sales; a tagged link appears here after a
                  compatible UK SKU is physically verified.
                </p>
              </article>
              <article className="affiliate-card">
                <h3>Amazon US &amp; global</h3>
                <p>
                  The US program is a separate enrollment with its own tag; a UK
                  tag does not earn on Amazon.com. Add it only after a compatible
                  US SKU and a real US audience are verified.
                </p>
                <a
                  className="link card-link"
                  href="https://www.amazon.com/s?k=bluetooth+scrolling+ring+page+turner"
                >
                  Browse Amazon US listings
                </a>
                <p className="affiliate-state">Not live yet — no US tag exists.</p>
              </article>
              <article className="affiliate-card">
                <h3>AliExpress</h3>
                <p>
                  A zero-inventory referral program for buyers who prefer
                  AliExpress, run through a separate portal with manual approval.
                </p>
                <p className="affiliate-state">
                  Apply after agentr.ing is live; publish a link only after the
                  portal confirms the exact WX02 campaign and link. Commissions and
                  the cookie window are unverified third-party scenarios.
                </p>
              </article>
              <article className="affiliate-card">
                <h3>Alibaba.com Affiliate</h3>
                <p>
                  The Alibaba.com Affiliate program covers the exact WX02 wholesale
                  listing as a zero-inventory referral — a future global
                  bulk-order and team CTA. Separate from any later branded resale
                  of Agent Ring hardware.
                </p>
                <a
                  className="link card-link"
                  href="https://www.alibaba.com/product-detail/Wholesale-Tiktok-Remote-BT-Ring-Remote_1601870411652.html"
                >
                  View the WX02 wholesale listing
                </a>
                <p className="affiliate-state">
                  Portal enrollment pending — no affiliate link yet.
                </p>
              </article>
            </div>

            <p className="disclosure">
              Agent Ring may earn a commission from purchases made through links on
              this page.
            </p>
          </div>
        </section>
      </main>

      <footer className="site-footer">
        <div className="wrap footer-inner">
          <p className="footer-brand">
            <span className="wordmark-dot" aria-hidden="true" />
            Agent Ring — agentr.ing
          </p>
          <nav aria-label="Footer">
            <a href="https://petrol.now">petrol.now</a>
            <a href="https://openwa.dev">openwa.dev</a>
          </nav>
        </div>
      </footer>
    </>
  );
}
