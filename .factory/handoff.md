# forge-sync polish 4 handoff

## Outcome

**PASS.** Repair commit `d7460b7d9416c422eb336a381cb1f31847db00a2` is pushed to
`main` and deployed to <https://forge-sync.sociobot.in>.

This round closes every finding in `.factory/review-1.md` through
`.factory/review-4.md` and every prior polish record. The live footer reports
build `d7460b7d`.

## What changed

- Demo storage now clears before every ordinary same-tab exit from either demo
  URL, including Home, Privacy, Terms, and the external source link. A
  `pageshow` reconciliation clears an old demo session when Back or Forward
  lands on a non-demo document, while reloads and demo-to-demo navigation keep
  the current session.
- The browser-isolation claim now proves query/path entry, reload, reset,
  demo-to-demo navigation, legal/home/external exits, and Back/Forward. It
  preserves a non-demo sentinel throughout.
- The configuration status now correctly tells people to set the named token
  environment variables. Its browser claim checks the exact message.
- Removed the untested Docker install/run instructions from the README and
  added a regression test that rejects undocumented container paths.
- Updated the claims sandbox description, demo documentation, copy audit, and
  verb-first catalog description.

## Exact verification

Clean clone: `/tmp/forge-sync-polish4-clean.A0kfRB/repo` at `d7460b7`.

```sh
npm ci
npm run test:claims
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --locked --allow-dirty
```

- All 21 declared claim commands passed from the clean clone.
- The scale claim completed 50 repositories and 5,000 issues in 26.676 s for
  the initial pass and 2.842 s for the unchanged pass (limits: 1,440 s and
  96 s).
- `npm test` passed 17 Node/browser/static tests and all Rust unit,
  integration, and documentation tests.
- The production build wrote `dist/site`; JavaScript is 6.47 kB raw / 2.78 kB
  gzip and CSS is 14.93 kB raw / 4.14 kB gzip.
- `cargo fmt --check`, Clippy with warnings denied, and package verification
  passed.
- Local accessibility: `npm run test:a11y -- http://127.0.0.1:4174` reported
  0 axe violations, including 0 serious/critical. URL verification reported a
  title, `lang=en`, one h1, main, complete alt text, labeled buttons, and no
  console errors. Evidence:
  `.factory/evidence/polish-4-local/verify/verify.json`.
- Live accessibility: `npm run test:a11y -- https://forge-sync.sociobot.in`
  reported 0 axe violations; `/opt/fleet/lib/verify-url.sh` reported no console
  errors. Evidence: `.factory/evidence/polish-4-live/verify/verify.json`.
- Live mobile Lighthouse: performance 100, accessibility 100, LCP 1.253 s,
  CLS 0.008. Evidence: `.factory/evidence/polish-4-live/lighthouse.json`.
- Cold live checks covered `/`, `/?demo=1`, `/demo/`, `/privacy/`, `/terms/`,
  and `/round-four-missing` (HTTP 404). They checked the first-screen bounds,
  demo storage, Back/Forward, path demo exit, offline reload, titles, one h1,
  main, canonical metadata, and no cross-origin product requests. Evidence:
  `.factory/evidence/polish-4-live/cold-check.json` and matching screenshots.
- The live link crawl returned 200 for every HTTP destination on all product
  routes (plus explicit mailto and in-page links). Live security headers
  include CSP, HSTS, referrer policy, nosniff, frame denial, permissions
  policy, and immutable hashed-asset caching.

## Deployment

The static work-order build command was `npm ci && npm run build:site`; the
published `dist/site` was deployed with
`/opt/fleet/lib/deploy-static.sh forge-sync dist/site`. Azure deployment ID:
`4c90af37-fe10-425a-9248-5349541a6fc0`.

## Known gaps and next steps

None. The Dockerfile remains available to maintainers, but no public container
installation promise remains until an isolated image claim can run in every
claims environment. A ready-to-publish CLI package can be made with
`cargo package --locked` (already verified); do not publish from this worker.
