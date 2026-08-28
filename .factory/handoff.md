# forge-sync review 2 handoff

## Delivered

Created .factory/review-2.md and committed it. This was a review-only work
order; product source was not changed.

## Verification performed

- Opened the live site from fresh 390 × 844 and desktop browser contexts and
  completed the cold-read check before scrolling.
- Exercised the live one-click demo, reset and leave-demo storage isolation,
  same-origin request behavior, designed 404, metadata, and link crawl.
- Ran cargo run -- demo --json from a new /tmp directory with canary token
  environment variables; it created a separate /tmp/forge-sync-demo-* output.
- Created /tmp/forge-sync-review2-clean.BBroWE with git clone --no-local, then
  ran npm ci, npm run test:claims, npm test, npm run build, and live npm run
  test:a11y -- https://forge-sync.sociobot.in. All passed; axe had zero
  violations.

## Remaining findings

The verdict is **FAIL**. README production promises about real state/archive
output, daemon passes, metadata records, pull-request history, attribution, and
optional archives do not have matching declared claims/tests. See F-2-1a through
F-2-1f in .factory/review-2.md. The review also records route-focus, copy, and
generic-action-label findings F-2-2 through F-2-5.

## Next step

Implement the review fixes, add exact claim tests, then repeat the full
adversarial review from a fresh clone and fresh browser contexts.
