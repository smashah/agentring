# macOS public release

Agent Ring has two deliberately separate macOS paths:

- `scripts/install-macos.sh` replaces Mohammed's internal `/Applications/Agent Ring.app` using the stable local identity that preserves the machine's existing TCC grants.
- `scripts/package-macos-release.sh` builds a distribution copy under `dist/`, signs it with Developer ID Application, notarizes and staples it, and never touches `/Applications`.

## Account Holder certificate gate

The prepared CSR is outside Git at:

`/Users/Mohammed/.config/agentring/signing/AgentRing-Developer-ID.certSigningRequest`

Its SHA-256 is:

`698818ce60a816346bc389846b54c2e52842f084045241c104a1a9eaddc34d5e`

The paired private key is already preserved in the login Keychain under `Agent Ring Developer ID Application`. The Account Holder must open <https://developer.apple.com/account/resources/certificates/add>, choose **Developer ID**, then **Developer ID Application**, upload that CSR, download the resulting `.cer`, and open the certificate once to import it into the login Keychain. Do not create another CSR, revoke a certificate, or replace the installed app.

Verify the import with:

```sh
security find-identity -v -p codesigning | grep 'Developer ID Application'
```

## Package without changing the installed app

Before the certificate exists, the assembly path can be checked without signing:

```sh
scripts/package-macos-release.sh --prepare-only
```

The validated `agentring-notary` Keychain profile is already configured from the existing App Store Connect API key. After the certificate is imported, run:

```sh
scripts/package-macos-release.sh
```

Set `AGENTRING_NOTARY_PROFILE` only to use a differently named Keychain profile. The script also accepts `APPLE_API_KEY_PATH` plus `APPLE_API_KEY_ID` and, for a team key, `APPLE_API_ISSUER_ID`.

The script refuses to continue unless exactly one usable Developer ID Application identity exists. It signs a separate `.app` with hardened runtime and timestamping, waits for Apple notarization, staples and validates the ticket, runs Gatekeeper assessment, and writes a versioned ZIP plus SHA-256 file beneath a unique `dist/agentring-public-*` directory.

`Resources/AppIcon.icns` is derived deterministically from the same canonical physical-ring asset used by the website and menu-bar source. Regenerate it with `scripts/build-app-icon.sh`; do not reintroduce the retired cursor/mouse artwork or create an independent logo.

Publishing the ZIP and changing the landing page's pending-download state happen only after the script reaches `public artifact ready`.
