import { createFileRoute } from '@tanstack/react-router';
import brandLogoUrl from '../../../../assets/logo.png?url';
import ringProductUrl from '../../../../assets/logo_menubar.png?url';

export const Route = createFileRoute('/')({
  component: Landing,
});

const gestures = [
  ['Tap', 'Enter'],
  ['Swipe up', 'Spotlight'],
  ['Swipe down', 'Mission Control'],
  ['Swipe left', 'Previous'],
  ['Swipe right', 'Next'],
  ['Long press', 'Anything'],
] as const;

function BrandLockup({ compact = false }: { compact?: boolean }) {
  return (
    <span className={compact ? 'brand-lockup brand-lockup-compact' : 'brand-lockup'}>
      <img src={brandLogoUrl} alt="" aria-hidden="true" />
      <span>Agent Ring</span>
    </span>
  );
}

function Landing() {
  return (
    <>
      <a className="skip-link" href="#main">
        Skip to content
      </a>

      <header className="site-header">
        <div className="shell header-inner">
          <a className="brand-link" href="#main" aria-label="Agent Ring home">
            <BrandLockup compact />
          </a>
          <nav aria-label="Page sections">
            <a href="#how-it-works">How it works</a>
            <a href="#release">Release</a>
            <a href="#hardware">Get a ring</a>
          </nav>
        </div>
      </header>

      <main id="main">
        <section className="hero" aria-labelledby="hero-title">
          <div className="hero-glow" aria-hidden="true">
            <img src={brandLogoUrl} alt="" />
          </div>
          <div className="shell hero-inner">
            <div className="hero-mark">
              <img
                src={brandLogoUrl}
                alt="Agent Ring — a ring, cursor and key"
                width="1024"
                height="1024"
                fetchPriority="high"
              />
            </div>
            <p className="eyebrow">The shortcut remote already on your finger</p>
            <h1 id="hero-title">Your ring. Any shortcut.</h1>
            <p className="hero-lede">
              Agent Ring turns the taps, swipes and long presses of a WX02-class
              Bluetooth ring into the keyboard shortcuts you choose.
            </p>
            <div className="hero-actions">
              <a className="primary-action" href="#release">
                Check the macOS release
              </a>
              <a className="text-action" href="#how-it-works">
                See how it works <span aria-hidden="true">↓</span>
              </a>
            </div>
            <p className="hero-proof">
              <span>Internal macOS build working</span>
              <span className="proof-divider" aria-hidden="true">
                ·
              </span>
              <span>Public download pending signing and notarization</span>
            </p>
          </div>
        </section>

        <section className="product-proof" aria-labelledby="product-title">
          <div className="shell product-layout">
            <div className="product-stage">
              <img
                src={ringProductUrl}
                alt="A black WX02-class Bluetooth finger ring"
                width="2048"
                height="2048"
                loading="lazy"
              />
              <p>WX02-class Bluetooth ring</p>
            </div>
            <div className="product-copy">
              <p className="section-label">Six inputs. Yours to map.</p>
              <h2 id="product-title">A page turner becomes a computer remote.</h2>
              <p>
                The ring was made to scroll a phone. Agent Ring reads the raw
                Bluetooth HID reports that macOS normally throws away, recognises
                each gesture, then fires the shortcut you assigned.
              </p>
              <dl className="gesture-list">
                {gestures.map(([gesture, action]) => (
                  <div key={gesture}>
                    <dt>{gesture}</dt>
                    <dd>{action}</dd>
                  </div>
                ))}
              </dl>
              <p className="mapping-note">Examples only. Every gesture is configurable.</p>
            </div>
          </div>
        </section>

        <section id="how-it-works" className="how-it-works" aria-labelledby="how-title">
          <div className="shell">
            <p className="section-label">From gesture to action</p>
            <h2 id="how-title">Connect. Map. Use.</h2>
            <ol className="step-rail">
              <li>
                <span className="step-number">01</span>
                <h3>Connect</h3>
                <p>
                  Pair a compatible WX02-class ring over Bluetooth. Agent Ring
                  identifies the product and listens to its raw input.
                </p>
              </li>
              <li>
                <span className="step-number">02</span>
                <h3>Map</h3>
                <p>
                  Press the ring, watch the gesture highlight, then assign a key
                  combination, media key or no action.
                </p>
              </li>
              <li>
                <span className="step-number">03</span>
                <h3>Use</h3>
                <p>
                  Leave the lightweight native app in the menu bar. Your mappings
                  work wherever ordinary keyboard shortcuts work.
                </p>
              </li>
            </ol>
          </div>
        </section>

        <section id="release" className="release" aria-labelledby="release-title">
          <div className="shell release-layout">
            <div>
              <p className="section-label">Release status</p>
              <h2 id="release-title">Built on macOS. Not publicly released yet.</h2>
            </div>
            <div className="release-copy">
              <p>
                The strict WX02 capture, shortcut injection, permission onboarding,
                editable mappings and menu-bar app are built and installed in
                internal testing. The app stays out of the Dock, as a menu-bar
                utility should.
              </p>
              <div className="release-state">
                <span className="state-dot state-dot-live" aria-hidden="true" />
                <p>
                  <strong>macOS</strong>
                  <span>Internal build working</span>
                </p>
              </div>
              <div className="release-state">
                <span className="state-dot" aria-hidden="true" />
                <p>
                  <strong>Public macOS download</strong>
                  <span>Waiting for Developer ID signing and notarization</span>
                </p>
              </div>
              <div className="release-state">
                <span className="state-dot" aria-hidden="true" />
                <p>
                  <strong>Windows 11</strong>
                  <span>Planned; no Windows build exists yet</span>
                </p>
              </div>
            </div>
          </div>
        </section>

        <section id="hardware" className="hardware" aria-labelledby="hardware-title">
          <div className="shell hardware-layout">
            <div>
              <p className="section-label">Compatible hardware</p>
              <h2 id="hardware-title">Buy links come after the ring passes the test.</h2>
              <p className="hardware-intro">
                The Amazon UK account is active, but no retail listing gets an
                affiliate recommendation until its ring is physically verified
                with Agent Ring. That keeps a cheap purchase from becoming the
                wrong purchase.
              </p>
            </div>
            <div className="hardware-links">
              <a
                href="https://www.amazon.co.uk/s?k=bluetooth+scrolling+ring+page+turner"
                className="market-link"
              >
                <span>
                  <strong>Amazon UK</strong>
                  <small>Browse untagged results while compatibility is tested</small>
                </span>
                <span aria-hidden="true">↗</span>
              </a>
              <a
                href="https://www.alibaba.com/product-detail/Wholesale-Tiktok-Remote-BT-Ring-Remote_1601870411652.html"
                className="market-link"
              >
                <span>
                  <strong>Alibaba.com</strong>
                  <small>View the exact WX02 wholesale listing</small>
                </span>
                <span aria-hidden="true">↗</span>
              </a>
              <p className="affiliate-note">
                Amazon Associates UK account created: <strong>agentring-21</strong>.
                AliExpress and Alibaba.com affiliate enrollment are pending. Links
                remain untagged until their portal and product checks are complete.
              </p>
              <p className="disclosure">
                Agent Ring may earn a commission from future qualifying purchases.
              </p>
            </div>
          </div>
        </section>
      </main>

      <footer className="site-footer">
        <div className="shell footer-inner">
          <BrandLockup compact />
          <p>Make a small ring do more.</p>
          <nav aria-label="Related projects">
            <a href="https://petrol.now">petrol.now</a>
            <a href="https://openwa.dev">openwa.dev</a>
          </nav>
        </div>
      </footer>
    </>
  );
}
