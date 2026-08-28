# forge-sync polish 1 handoff

## Delivered

Repair commit `75380810be9c0f53a3c9f51079acb645265340bc` closes every finding in
`.factory/review-1.md` and preserves the glacial ceramic visual system.

- The first phone and desktop viewport now names the job, maintainer audience,
  primary sample action, what the action does, and three tested facts.
- `/?demo=1` and `/demo/` show the completed Harbor Cooperative sample with a
  persistent banner. Reset changes only `demo:forge-sync:` state. Start for
  real discards demo state and leaves non-demo storage untouched.
- `forge-sync demo` uses the shipped fixtures and production model, SQLite,
  audit, archive, rendering, and Git commit code inside a new temporary
  directory. Canary tests prove it ignores working-directory configuration and
  token values.
- `.factory/claims.json` has ten retained claims. Each has exactly one tagged,
  isolated test; `npm run test:claims` validates the mapping and runs all ten.
- Every route has its own title, description, canonical, Open Graph/Twitter
  metadata, 1200×630 preview, touch icon, shared header/footer, legal links,
  and Git-derived build ID. The host serves the designed page with status 404.
- Hash navigation restores scroll and heading focus through back and forward
  navigation. Reduced-motion keyboard focus, 200% text, and 390 px layout are
  regression-tested.
- The unavailable Migration Kit and its dead checkout were removed, together
  with unsupported price, license, and entitlement claims.
- `.factory/polish-1.md` maps every B, M, U, L, and R finding to its change and
  evidence. `.factory/copy-audit.md` covers every landing/dynamic copy unit.

## Clean-clone verification

Fresh clone: `/tmp/forge-sync-polish-clean.3duRG2`, checked out at the exact
repair commit above.

```sh
npm ci
npm run test:claims
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --locked
cargo package --locked --allow-dirty
```

All commands passed. Claim runner: 10/10. Full suite: 13 Node/browser tests
and 12 Rust unit/integration tests passed. `npm ci` reported zero
vulnerabilities. The release binary is 6,409,848 bytes. The verified crate is
237,930 bytes.

The production site build contains 5.81 kB raw JavaScript (2.53 kB gzip),
14.00 kB raw CSS (3.97 kB gzip), 82.33 kB of self-hosted fonts, and a 70.11 kB
hero image. It is deployed from `dist/site`.

## Accessibility, privacy, offline, and performance

- Local and live `npm run test:a11y`: zero axe violations and zero
  serious/critical findings across home, demo, privacy, terms, and 404.
- Live URL verifier: title, language, one h1, main, alt text, button labels,
  and console checks passed. Evidence:
  `.factory/evidence/polish-1-live/verify/verify.json`.
- Live privacy flow contacted only `https://forge-sync.sociobot.in`; the
  configuration builder made no outside request.
- A cold live service-worker test reloaded `/?demo=1` offline and regenerated
  configuration while offline.
- Live Lighthouse mobile: performance 100, accessibility 100, best practices
  100, SEO 100; LCP 1.7 s, CLS 0.008, total blocking time 10 ms. Evidence:
  `.factory/evidence/polish-1-live/lighthouse-summary.json`.

## Deployment and cold production check

Static deployment `0f29e791-3a3e-47e8-b786-5af8786003bb` completed through
the work-order deploy command. The public footer reports build `75380810`.

Fresh Chromium contexts rechecked:

- `/`, `/?demo=1`, `/demo/`, `/privacy/`, and `/terms/` at HTTP 200;
- `/definitely-missing-review-route` at HTTP 404 with the product page;
- all 71 crawl checks passed, including internal/legal links and public assets;
- demo reset/exit isolation, offline use, mobile first action, 390 px overflow,
  and back/forward focus all passed with no application console errors;
- CSP, Permissions-Policy, Referrer-Policy, X-Content-Type-Options, HTML
  revalidation, and immutable hashed-asset caching are present.

Evidence is in `.factory/evidence/polish-1-live/`, including
`cold-check.json`, `link-crawl.json`, `home-mobile.png`, `demo-mobile.png`, and
`home-desktop.png`.

## Known gaps and next steps

No review finding or known product defect remains. Docker/Podman is not
installed in the worker, so the container wrapper was not rebuilt; its exact
Rust release build passed from the clean clone. Registry publication remains a
factory release action and was not performed.
