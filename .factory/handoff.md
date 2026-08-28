# forge-sync repair handoff — perfection loop 1

## Delivered

Product repair commit: `7cc585aa2def1a1842a23d6a257a9643b265f0c8`.

- Replaced the vague first screen with the requested maintainer-focused
  headline, one primary sample action, visible explanation, and three plain
  facts. At 390 px the copy and action now precede the ceramic art.
- Added `forge-sync demo`, shipped Harbor Cooperative fixture JSON, and an
  inspectable temporary output tree with target records, branches, tags,
  pull-request review data, mappings, audit events, and a committed archive.
- Added `/?demo=1` and `/demo/`, persistent demo banner, Reset demo, Start for
  real, and a `demo:forge-sync:` browser-storage namespace. The paid kit was
  removed because its only checkout route was dead.
- Added claims, isolated tests, demo documentation, plain-words audit, catalog
  description, product 404 page, route metadata, OG artwork, touch icon,
  robots/sitemap, and Static Web Apps 404 response override.
- Made every route use the same header/footer. Hash navigation now manages
  History API state, scroll position, focus, and polite route announcements.

The glacial ceramic visual system is retained. `og-image.webp` and the touch
icon are crops derived from the existing product-owned ceramic illustration;
its provenance remains in `.factory/design.md`.

## Verification evidence

Clean clone: `/tmp/forge-sync-clean.SDFm0e` from commit `7cc585a`.

```sh
npm ci
npm run test:claim:demo-cli
npm run test:claim:demo-browser
npm run test:claim:configuration
npm run test:claim:offline
npm run test:claim:license
npm test
npm run build
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

All commands passed. `npm test` passed 9 Node/browser tests and 9 Rust tests.
Every entry in `.factory/claims.json` passed from the clean clone.

Local production-site checks also passed:

- `npm run test:a11y -- http://127.0.0.1:4174` — 0 axe violations; 0
  serious/critical across home, demo, privacy, terms, and 404.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4174/ ...` — title, `lang`,
  one h1, main, image alt coverage, and console checks passed. Evidence:
  `.factory/evidence/verify-url-round1/verify.json`.
- Lighthouse mobile local production build: performance **99**, accessibility
  **100**. Evidence: `.factory/evidence/lighthouse-round1.json`.
- Production build: JS 4,916 bytes raw / 2.23 KB gzip; CSS 13,777 bytes raw /
  3.90 KB gzip; hero 70 KB; derived OG image 31,882 bytes.

## Run and deploy

Build the static deployment with `npm run build`; deploy `dist/site` using the
work-order static deployment configuration. The committed
`site/public/staticwebapp.config.json` carries CSP, cache headers, and the
product 404 response override.

The repair branch and deployment handoff were pushed to `origin/main` at
`4c4ef14535ec74f3cd11c88d5d98eb023bce2e14`. During the final host poll at
10:14 UTC, `https://forge-sync.sociobot.in/` still served the preceding
revision and `/demo/` returned 404. Static-host propagation is factory-managed;
the committed build and deployment configuration are ready for that release.

No known blocking findings remain. The only intentionally omitted surface is
the unsupported paid Migration Kit; no checkout or license claim is shipped.
