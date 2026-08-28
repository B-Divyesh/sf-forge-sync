# forge-sync adversarial review 3 handoff

## Delivered

- Added `.factory/review-3.md` with a full cold-read, copy, demo, claim,
  sandbox, history, routing, accessibility, link, visual-identity, and missed-
  leverage review.
- Verdict: **FAIL** with three blocking, two major, and four minor findings.
- No product source or deployment was changed.

## Main blockers

1. The documented Rust 1.85 minimum is false for the locked dependency graph;
   `cargo +1.85.0 build --locked` exits 101 because resolved ICU crates require
   Rust 1.88.
2. Review 1 B2 is only partly repaired: the live CLI demo uses hand-written
   terminal blocks rather than a self-hosted recording of the real command,
   and the displayed transcript differs from actual output.
3. The README promise that `status` and `sync` emit JSON remains absent from
   `.factory/claims.json`, reopening U20/R06.

The review also records unlisted production record-link/run-history behavior,
the missing scale benchmark from the brief, a query-demo heading mismatch, a
desktop first-screen fact below the fold, and two plain-word copy issues.

## Verification performed

Clean clone: `/tmp/forge-sync-review3-clean.rdSq7R/repo` at
`18ab53872546650039facbff88a6f04050a301ca`.

```sh
npm ci
npm run test:claims   # PASS: all 16 declared claims
npm test              # PASS: 13 Node/browser tests and all Rust tests
npm run build         # PASS: dist/site produced
npm run test:a11y -- https://forge-sync.sociobot.in
                       # PASS: 0 axe violations
```

Additional checks:

- Fresh Chromium at 390 × 844 and 1440 × 900.
- Live one-click demo entry, Reset, exit, Back, Forward, focus, clipboard, and
  configuration download.
- Browser storage sentinel and full request interception; no non-demo storage
  changed and no cross-origin request occurred.
- Real `forge-sync demo --json` from an empty temporary directory with canary
  token variables; output was isolated and contained SQLite state, target
  records, JSON archive files, and a Git commit.
- Live route metadata and heading inventory, designed HTTP-404 response,
  security headers, internal/external link crawl, robots, and sitemap.
- `/opt/fleet/lib/verify-url.sh https://forge-sync.sociobot.in
  /tmp/forge-sync-review3-verify`: PASS with no console errors or missing alt
  text.
- `cargo +1.85.0 build --locked`: **FAIL**, exit 101, as documented in F-3-1.

## Next steps

Resolve every finding in `.factory/review-3.md`, add the missing claims and
tests, regenerate the CLI demo surface from real command output, and repeat the
entire adversarial review from a clean clone. The current release is not ready
for a PASS verdict.
