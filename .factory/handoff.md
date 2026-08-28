# forge-sync adversarial review 1 handoff — FAIL

Reviewed the deployed product and clean repository at
`b3d507fde6d9e89a9002027087aff72b35bbb889`. The full evidence-backed result is
in `.factory/review-1.md`.

## What was done

- Opened the live home page in fresh 390 × 844 and 1440 × 900 Chromium
  contexts before scrolling.
- Audited every landing/README sentence and visible copy unit for length,
  jargon, marketing language, terminology, heading context, and action naming.
- Tried `/demo`, `?demo=1`, and `forge-sync demo` from an empty temporary
  directory; no demo exists.
- Checked for `.factory/claims.json` and `@claim:` tests; both are absent, then
  inventoried every unlisted live/README claim.
- Exercised same-origin network behavior and an offline reload, crawled links,
  inspected metadata/routes/404/history/focus/header/footer, and reviewed the
  product's visual identity.
- Changed no product code.

## How to verify

```sh
npm ci
npm test
npm run build
npm run test:a11y -- https://forge-sync.sociobot.in
mkdir -p .factory/evidence/verify-url
VERIFY_NODE_MODULES="$PWD/node_modules" /opt/fleet/lib/verify-url.sh \
  https://forge-sync.sociobot.in .factory/evidence/verify-url
```

The clean-clone test/build, live axe scan, and URL smoke verifier passed. The
review still fails on four blockers: unclear first screen, no sandbox demo,
missing claims contract/tests, and generic broken deep-route handling.

## Known gaps / next steps

Implement B1–B4 first. Then repair the dead checkout, metadata/crawl files,
focus/history handling, and shared header/footer. Apply the exact copy rewrites
and claim-test requirements in `.factory/review-1.md`, rerun from a clean clone,
and repeat this cold review.
