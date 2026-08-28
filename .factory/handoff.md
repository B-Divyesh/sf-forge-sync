# forge-sync repair handoff — perfection loop 2

## Delivered

Repair implementation commit: `1a38c93a2235db2ae698fca1107ca68de87b315f`.
CI browser-install repair: `39c9899cd2ea43ff04e492bbf976ef80eb3e3dda`.

- Added six real configured-run claims/tests for every README production
  promise found in review 2: state/archive paths, daemon second pass,
  discovery/refs/metadata, complete pull-request history, attribution, and
  optional Git archive behavior.
- Repaired document-route focus and route announcements for the one-click demo
  and leave-demo paths; tested CTA, leave, Back, and Forward.
- Rewrote the remaining vague/internal labels, split the README long sentence,
  and named the leave-demo result. Updated the full copy audit and catalog line.
- Kept the glacial ceramic visual system and original product assets intact.
- Fixed CI to install Playwright Chromium before browser tests.
- Deployed the built static site with `/opt/fleet/lib/deploy-static.sh
  forge-sync dist/site`. The live footer reported build `39c9899c` after deploy.

## Exact verification evidence

Fresh clone: `/tmp/forge-sync-polish2-clean.o8RWuj`, created with
`git clone --no-local /work/repo` at repair commit `129944d` (the later
`1a38c93` change was Rust formatting only, and `cargo fmt --check` passed on
the final tree).

```sh
npm ci
npm run test:claims       # 16/16 declared claims passed
npm test                  # 13 Node/browser tests plus all Rust tests passed
npm run build             # dist/site produced
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Final-tree local checks also passed:

```sh
npm run test:a11y -- http://127.0.0.1:4174
```

This reported 0 axe violations and 0 serious/critical findings. Build output
is 5.92 KB raw JavaScript (2.57 KB gzip), 14.00 KB CSS (3.97 KB gzip), and a
70.11 KB hero image. The initial JavaScript budget is therefore comfortably
below 200 KB.

Deployed checks passed:

- `npm run test:a11y -- https://forge-sync.sociobot.in` — 0 axe violations,
  0 serious/critical.
- Fresh 390px browser: first CTA was in the first viewport; CTA entered demo;
  demo heading received focus and an announcement; leaving focused the home
  h1; the product 404 had the expected title and h1.
- `https://forge-sync.sociobot.in/`, `/?demo=1`, `/demo/`, `/privacy/`,
  `/terms/`, and `/404.html` returned product metadata, one main, and one h1.
  `/definitely-missing-polish-2` returned HTTP 404 with product headers.
- Screenshots: `.factory/evidence/polish-2-local/` and
  `.factory/evidence/polish-2-live/` (ignored build evidence).

## Run, test, deploy

```sh
npm ci
npm test
npm run test:claims
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
/opt/fleet/lib/deploy-static.sh forge-sync dist/site
```

`dist/site` is the static deployment artifact. The committed
`site/public/staticwebapp.config.json` supplies the CSP, cache policy, and
product-styled 404 status override.

## Known gaps

None. The unsupported paid checkout remains intentionally absent rather than
advertised. No tracking, payment, or third-party runtime service was added.
