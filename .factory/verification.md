# Independent verification — FAIL

**Candidate:** `0bb3e81a3c8f74b70e8191e3fb8fe6ba11044e5e`
**Live URL:** `https://forge-sync.sociobot.in/`
**Date:** 2026-08-27
**Scope:** clean-clone CLI/package, local mocked end-to-end flow, and the deployed documentation PWA. No product source code was changed by this verification.

## Verdict

**FAIL.** The normal mocked one-way mirror works, and the live site is byte-for-byte the candidate build for the checked assets. But running `sync --dry-run` poisons the durable ID mapping database: the following real synchronization reports success while silently omitting target labels and issues. This is a data-migration correctness failure in a documented/recommended workflow and blocks release.

## Release-blocking defect

### P0 — dry run causes a later real sync to omit all previously mapped metadata

Reproduction against local GitHub/Forgejo HTTP mocks and real local bare Git repositories:

1. Run `forge-sync sync --json --dry-run` for one repo with one GitHub issue.
   It returned `{"discovered":1,"synchronized":1,"failed":0,"issues":1,...,"dry_run":true}`.
2. Run the identical configuration without `--dry-run`.
   It returned `{"discovered":1,"synchronized":1,"failed":0,"issues":1,...,"dry_run":false}`.
3. The recorded target requests for the real run were only target `GET /user`, repo lookup, and `POST /orgs/team/repos`; there was **no** `POST /labels` and **no** `POST /issues`. The target was therefore an empty repository despite the reported success.

Cause verified in the candidate source: dry-mode adapter calls return source IDs, then `engine::sync_repository` persists them through `state.map`. On the next real pass, fingerprint comparisons treat those source IDs as genuine target mappings and skip creation. The shipped Migration Kit explicitly tells users to run a dry pass before cutover, so this is not an edge case.

## Other defects

### P1 — reduced-motion mobile keyboard navigation loses visible focus

At 390×844 with `prefers-reduced-motion: reduce`, keyboard Tab traversal over the skip link, navigation, buttons, selects, inputs, and code pre showed computed `outline: ... 0px`; only the first input retained a ring. The normal-motion desktop traversal had the designed 3px cobalt ring. The `@media (prefers-reduced-motion)` universal `!important` transition/animation rule wins over the global `:focus-visible` ring. This violates the required visible focus state for a keyboard-only user.

### P2 — documented configuration/usage errors do not consistently use exit code 2

`target/debug/forge-sync status --config bad.toml --json`, with an empty source org and invalid URL, emitted `source.org may not be empty` and exited **1**, though README promises exit **2** for configuration/usage errors. Missing token input did exit 2 as promised.

### P2 — live deployment does not apply its shipped immutable asset cache policy or Permissions-Policy

Candidate `site/_headers` declares immutable `/assets/*` caching and `Permissions-Policy`. The live matching build returns `Cache-Control: public, must-revalidate, max-age=30` for HTML, hashed JS, CSS, and `sw.js`; it omits `Permissions-Policy` and CSP. HSTS, `X-Content-Type-Options`, and `Referrer-Policy` are present. This does not alter the byte-identical build but misses the stated deployment/performance hardening.

## Evidence that passed

### Clean checkout and package

- Clean clone at the candidate SHA; `npm ci` completed with 0 audited vulnerabilities.
- `cargo fmt --check` passed.
- `cargo test` passed: 4 unit, 2 API-contract, 1 real local Git-mirror integration test, and doc tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo build --release` passed; release binary is 6,365,336 bytes.
- `cargo package --allow-dirty` passed and verified the crate (`target/package/forge-sync-0.1.0.crate`, 133,344 bytes).
- `npm test` passed (3 site tests plus the Rust suite); `npm run build` passed. The exact static output is `dist/site`.
- A clean unpacked consumer install of the packaged crate completed successfully; its installed `forge-sync 0.1.0` binary executed both `--version` and `config example`.
- The container image could not be built in this verifier environment because neither `docker` nor `podman` is installed. The exact release binary build used by the Dockerfile (`cargo build --release`) did pass on Rust 1.98.0.

### Normal, boundary, malformed, and recovery behavior

- Normal mocked Forgejo flow: first pass created 1 repo, 2 labels (including the PR marker), 1 issue, and 1 comment; mirrored `master` and `v1`; wrote `repositories/demo/items/1.json` and a Git archive.
- Boundary filter: an archived source repo was supplied with `include_archived=false`; only the normal repo was discovered/synchronized.
- Idempotent recovery: a second unchanged pass made no additional repo/label/issue/comment create calls; state had 4 mappings and 1 successful repository.
- Malformed config was rejected without contacting a remote; the exit-code inconsistency is recorded above.
- CLI help, `config example`, JSON `status`, and missing-token failure paths were exercised. Commands do not prompt.

### Live site, privacy, accessibility, PWA, and match

- The live `index.html`, `main-CNT8cG3r.js`, `style-DjAeVjhm.css`, and `sw.js` are byte-identical to the exact production build (SHA-256 comparison).
- Desktop and 390px mobile normal-motion smoke: one `h1`, `main`, `lang=en`, title, labelled config form, successful GitLab config generation, malformed-org feedback, no console errors/page errors, and no outbound requests on a normal page load.
- `node scripts/a11y.mjs https://forge-sync.sociobot.in` found 0 axe violations, including 0 serious/critical, on `/`, `/privacy/`, and `/terms/`.
- PWA: service worker `forge-sync-v1` installed/controlled the page; its shell cache contained `/`, `/privacy/`, and `/terms/`; an offline reload rendered the home h1. A local static-server update simulation served a revised worker, and the cache advanced from `forge-sync-v1` to `forge-sync-v2` after the old client closed.
- Plain-page outbound surface is local-only; source inspection shows the only runtime third-party request is the documented Sociobot license-verification request, made only after a license token is present. Privacy/terms accurately disclose local license storage and that request.
- Static budgets pass: JS 5.51 KB, CSS 11.75 KB, fonts 82.33 KB combined, hero 70.11 KB; all are within the specified limits.

## Required next steps

1. Make dry runs non-mutating for mappings, repository success state, and archive commits (or keep dry-run data in a separate disposable plan), then add a regression test for dry-run followed by real sync.
2. Restore a visible focus ring under reduced motion and add a keyboard/reduced-motion browser assertion.
3. Make every validation/configuration error return exit 2 and cover it in tests.
4. Configure the actual hosting layer to honor immutable hashed-asset caching and the shipped security headers; add a deployed-header check.
5. Re-run this verification, including the dry-run-to-real-sync regression, after fixes.
