# Perfection loop polish 3 — cumulative closure

Reviewed candidate `18ab53872546650039facbff88a6f04050a301ca` and every
earlier review/polish report. The repair implementation is
`9e8cfc3f22006568c508ee0dc75d10d502ee221a` and is deployed at
<https://forge-sync.sociobot.in>.

Evidence used below:

- Clean clone: `/tmp/forge-sync-polish3-clean.z2XH1c/repo` at `9e8cfc3`.
- Clean claims: `npm run test:claims` — all 21 declared claims passed.
- Clean suite: `npm test`, `npm run build`, `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, and
  `cargo package --locked --allow-dirty` — all passed.
- Local screenshots: `.factory/evidence/polish-3-local/home-mobile.png`,
  `demo-mobile.png`, `home-desktop.png`, and `demo-desktop.png`.
- Live screenshots: `.factory/evidence/polish-3-live/home-mobile.png`,
  `demo-mobile.png`, `home-desktop.png`, and `demo-desktop.png`.
- Live cold result: `.factory/evidence/polish-3-live/cold-check.json`.
- Live link, accessibility, offline, security, and Lighthouse results are in
  `.factory/evidence/polish-3-live/`.

## Review 3 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Raised the documented and package minimum to Rust 1.88. Added an exact-toolchain claim and a Rust 1.88 CI job that build and test with `--locked`. | `@claim:minimum-rust-build`; clean clone passed `cargo +1.88.0 build --locked` and `cargo +1.88.0 test --locked`; the pushed README at GitHub says Rust 1.88. CLI-only, so no screenshot applies. |
| F-3-2 / B2 | Added a self-hosted SVG terminal recording and text transcript generated from the real `forge-sync demo` command. The generator normalizes only the temporary path and deletes that output. Both browser demo surfaces use the generated asset. | `@claim:demo-recording-current-output`; `demo pages use the generated recording and captured transcript`; `.factory/evidence/polish-3-live/demo-mobile.png`; live `/?demo=1` and `/demo-recording.svg`, with byte equality recorded in `cold-check.json`. |
| F-3-3 / U20 / R06 | Added a declared claim that runs configured `status --json` and `sync --json`, parses each entire stdout document, and checks its fields. Status JSON now uses `record_links` and `run_history_entries`. | `@claim:status-sync-json-output`; clean 21-claim run; pushed README line 81; live footer identifies deployed build `9e8cfc3f`. |
| F-3-4 / R12 | Standardized visitor and human CLI terms to “links between GitHub and target records” and “dated run history”. Added a configured-run SQLite test for both records and renamed status JSON keys. | `@claim:configured-record-links-run-history`; `.factory/evidence/polish-3-live/demo-desktop.png`; live `/?demo=1` and `/demo/`. |
| F-3-5 | Added a deterministic release benchmark with 50 local Git repositories and 5,000 issues. It checks initial and unchanged passes with a 20% safety margin and reports hardware. | `@claim:organization-scale-performance`; clean clone measured 26.028 s initial and 2.431 s incremental on Linux x86_64 with one CPU; pushed README “Scale acceptance”. CLI-only, so no screenshot applies. |
| F-3-6 | Query demo mode promotes “See a completed sample mirror” to the sole h1, demotes the landing headline to h2, focuses the demo h1, and announces the route. | `routes, metadata, focus, and phone first action work`; `.factory/evidence/polish-3-live/demo-mobile.png`; live `/?demo=1` returned title `Demo — forge-sync`, one h1, and initial focus `demo-panel-title`. |
| F-3-7 | Reduced desktop hero type and vertical spacing while preserving the ceramic layout. Added bounds checks for all three facts at 1440×900. | `routes, metadata, focus, and phone first action work`; `.factory/evidence/polish-3-live/home-desktop.png`; live fact bottoms were 814.22, 814.22, and 844.03 px. |
| F-3-8 | Replaced “Start with a fixture” with “Start with sample data”. | `.factory/copy-audit.md`; `.factory/evidence/polish-3-live/home-mobile.png`; live `/`. |
| F-3-9 | Replaced “Copy code and its working record” with “Copy code, issues, and pull-request history”. | `.factory/copy-audit.md`; `.factory/evidence/polish-3-live/home-desktop.png`; live `/#how`. |

## Earlier severity and review 2 findings

| Finding | Change retained or strengthened | Evidence |
| --- | --- | --- |
| B1 | The first screen names the job, maintainers, targets, sample action, result, and three facts. Copy precedes art at 390 px. | browser route test; live home screenshots; live `/`. |
| B2 | Query and path demos, prefixed storage, reset/leave behavior, shipped CLI sample, and generated real-command recording are all present. | demo browser/CLI/recording claims; live demo screenshots; live `/?demo=1` and `/demo/`. |
| B3 | `.factory/claims.json` now contains 21 claims with exactly one tagged test each; CI runs the claim suite. | `each declared claim has exactly one tagged test and a runnable command`; clean `npm run test:claims`. |
| B4 | The product 404 and host override remain, with a real 404 status and return action. | `crawl surfaces list every route and no dead checkout remains`; `cold-check.json`; live `/polish-3-missing-route`. |
| M1 | The unavailable paid tier, checkout, price, and license UI remain removed. | `crawl surfaces list every route and no dead checkout remains`; `link-crawl.json`; live route crawl. |
| M2 | Route-specific titles, descriptions, canonicals, OG/Twitter data, social art, icons, robots, and sitemap remain complete. | `all routes ship complete product metadata and the shared skeleton`; `cold-check.json`; live `/robots.txt` and `/sitemap.xml`. |
| M3 | Hash/document navigation and Back/Forward restore context, scroll, heading focus, and announcements. | browser route test; live query demo initial focus in `cold-check.json`; live `/?demo=1`. |
| M4 | Every route retains the shared header/footer, legal links, factory credit, version, and build ID. | static metadata/skeleton test; `cold-check.json`; live `/`, `/demo/`, `/privacy/`, `/terms/`, and 404. |
| M5 | Copy stays within 22 words, avoids banned terms, and uses the terminology table. | `.factory/copy-audit.md`; live home/demo screenshots; live `/`. |
| F-2-1a, F-2-1b, F-2-1c, F-2-1d, F-2-1e, F-2-1f | Retained six configured-run claims for state/archive, continuous passes, metadata, pull-request history, attribution, and optional Git commits. | `@claim:configured-run-state-and-archive`, `continuous-daemon-passes`, `configured-records-metadata`, `configured-renders-pull-request-history`, `configured-copied-body-attribution`, and `configured-optional-git-archive`; clean claim run; live README link. |
| F-2-2 | Document-route focus, announcements, CTA, leave, Back, and Forward remain tested. | browser route test; `cold-check.json`; live `/?demo=1`. |
| F-2-3 | The long README sample sentence remains split into short concrete sentences. | README; clean static suite; pushed README. |
| F-2-4 | Public terminology is now stricter: no “usable”, mapping/audit, committed-JSON, or fixture wording remains on product surfaces. | `.factory/copy-audit.md`; live demo screenshots; live `/` and `/demo/`. |
| F-2-5 | Every leave action names its result: “Leave demo and build configuration”. | `@claim:demo-browser-isolation`; live demo screenshots; live `/?demo=1`. |

## Review 1 claim findings

| Finding IDs | Change retained or strengthened | Evidence |
| --- | --- | --- |
| U01, U04, U08, U17, U18, U19, U22, U23, U24, U25, U27, U28, U33, U34, U36, U37, U38, U40, U41, U44, U47 | Unsupported absolute, fabricated quantitative, paid, relay, release-binary, retry, partial-failure, token, and license-cache promises remain removed. | Copy/static tests; `.factory/copy-audit.md`; `link-crawl.json`; live `/` and pushed README. |
| U02, U05, U09, U10, U15, U26 | Fidelity language remains sample-scoped and is proved through real SQLite state, rendered pull-request content, canary-token scans, and a real Git archive commit. | `@claim:demo-completed-mirror`; live demo recording/screenshots; live `/?demo=1`. |
| U03, U07, U14, U29, U32, U45, U46 | Retained MIT, no-analytics, token-free builder, browser/CLI boundary, and offline statements have isolated tests. | `@claim:mit-license`, `website-no-tracking`, `configuration-has-no-token-field`, and `offline-demo-after-first-visit`; `offline-summary.json`; live `/privacy/`. |
| U06, U11, U12, U13, U30, U31, U35, U42 | Target creation, Git refs, configured metadata, pull-request rendering, attribution, and archives are covered by real local contract fixtures. | target, Git-ref, configured metadata/history/attribution, and archive claim tests; live README and `/demo/`. |
| U16, U39, U43 | Doctor and dry-run promises remain bounded and read-only. | `@claim:doctor-read-only` and `@claim:dry-run-read-only`; clean claim run; live README. |
| U20 | The retained status/sync JSON promise now has its own manifest entry and parsed-output test. | `@claim:status-sync-json-output`; clean claim run; pushed README. |
| U21 | The only retained price/license claim is the MIT license. | `@claim:mit-license`; live home screenshot and `/terms/`. |

## Review 1 copy findings

| Finding IDs | Change retained or strengthened | Evidence |
| --- | --- | --- |
| L01, L02, L03, L04, L05, L06, L07, L08, L09, L10 | Direct job headline, organization/repository terms, named targets, sample action, and non-metaphorical copy remain. | `.factory/copy-audit.md`; browser route test; live home screenshots and `/`. |
| L11 | The final heading is “Copy code, issues, and pull-request history.” | `.factory/copy-audit.md`; live `/#how`. |
| L12, L13, L14, L15, L16, L17, L18, L19, L20, L21, L22 | Rate-limit and internal jargon remain removed; sample archive and pull-request results are concrete and tested. | demo/recording/configured claims; live demo screenshots and `/demo/`. |
| L23, L24, L25, L26, L27, L28, L29, L30 | Configuration and CLI actions use result labels; invented counts and broad retry/failure/scriptability claims remain absent. | configuration/doctor/JSON claims; live `/`. |
| L31, L32, L33, L34, L35, L36, L37, L38, L39, L40 | Paid and unsupported roadmap/relay content remains absent. | static crawl test; `link-crawl.json`; live `/`. |
| L41, L42, L43, L44, L45, L46, L47 | Privacy, closing, footer, validation, clipboard, and offline wording remains plain and tested; “fixture” is now “sample data”. | `.factory/copy-audit.md`; privacy/offline/browser tests; live `/privacy/` and `/`. |
| R01, R02, R03, R04, R05, R07, R08, R09, R10, R11, R13, R14, R15, R16, R17, R18, R19, R20, R21, R22, R23, R24, R25, R26, R27 | README scope, audience, setup, record, dry-run, security, and development copy remains short and concrete. | pushed README; clean suite; live GitHub README URL. |
| R06 | The narrowed JSON sentence is now backed by its own claim. | `@claim:status-sync-json-output`; pushed README line 81. |
| R12 | README, browser, human CLI, and status JSON now use record-link/run-history terms consistently. | `@claim:configured-record-links-run-history`; live demo; pushed README. |

## Earlier verification regressions

| Finding | Change retained | Evidence |
| --- | --- | --- |
| P0 | Dry run remains non-mutating, and the next real run creates every reported record. | `@claim:dry-run-read-only`; clean claim run. |
| P1 | Reduced-motion keyboard focus keeps the designed 3 px ring. | browser route test and live a11y run; `a11y-summary.json`; live all-route scan. |
| P2-exit | Documented configuration failures continue to exit with code 2. | `all_documented_configuration_failures_use_exit_code_two`; clean `npm test`. |
| P2-host | Production serves CSP, permissions, referrer, frame, nosniff, and immutable hashed-asset headers. | deployment static test; `security-summary.json`; live `/` and hashed JS response. |

Every finding is closed. No severity is deferred.
