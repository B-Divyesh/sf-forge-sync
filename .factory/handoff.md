# forge-sync adversarial review 4 handoff

## Outcome

Adversarial review 4 is complete against live
<https://forge-sync.sociobot.in> and base commit
`943e558a37d4d0db5bf316b33a3fd7d9f28df6a1`.

Verdict: **FAIL**. No product code was changed.

The complete report is `.factory/review-4.md`. It records:

- **F-4-1 / B2 (blocking):** leaving an active demo through Privacy keeps
  `demo:forge-sync:session` after the banner disappears. The existing claim
  test covers only the dedicated leave link.
- **F-4-2 (major):** README documents a Docker build/run path with no
  `claims.json` entry or test.
- **F-4-3 (minor):** the builder tells users to add environment-variable names
  at run time even though the generated configuration already contains them.

## Verification completed

Clean clone: `/tmp/forge-sync-review4-clean.Ud98Fz/repo`.

```sh
npm ci
npm run test:claims
npm test
npm run build
npm run test:a11y -- https://forge-sync.sociobot.in
```

- All 21 declared claim commands returned success.
- Rust 1.88.0 build and tests passed.
- The scale claim measured 24.347 seconds initial and 2.408 seconds
  incremental for 50 repositories and 5,000 issues.
- The general suite passed 16 Node/browser/static tests and all Rust tests.
- The production build generated `dist/site`; JavaScript is 6.18 kB raw and
  2.66 kB gzip.
- Live axe: 0 violations, including 0 serious/critical.
- The URL verifier found the title, language, one h1, main, complete alt and
  button labels, and no console errors.
- Live route/metadata/header checks, full link crawl, offline reload, security
  headers, first-screen bounds, and 404 checks otherwise passed.
- The live home HTML is byte-identical to the clean production build.

The real CLI demo was also run from a new temporary working directory with
canary token values. It created an isolated `/tmp/forge-sync-demo-*` tree with
the expected refs, metadata, SQLite record, dated history, and committed JSON
archive. Neither canary appeared in the output.

## Required next steps

1. Clear demo-prefixed storage for every transition from demo to a non-demo
   URL, and extend the claim test to Privacy, Terms, wordmark, external links,
   Back/Forward, and both demo entry forms.
2. Add an isolated container build/configured-sync claim or remove the
   unverified container instructions.
3. Rewrite the builder status to tell users to set the named token environment
   variables, then rerun the full review.
