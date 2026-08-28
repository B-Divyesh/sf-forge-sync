# Perfection loop polish 2 — cumulative finding closure

Repair implementation: `1a38c93a2235db2ae698fca1107ca68de87b315f`.
Verification/deployment repair: `39c9899cd2ea43ff04e492bbf976ef80eb3e3dda`.
Reviewed inputs: `.factory/review-1.md`, `.factory/polish-1.md`, and
`.factory/review-2.md`.

Evidence used below:

- **clean claims** — `/tmp/forge-sync-polish2-clean.o8RWuj`: `npm ci` then
  `npm run test:claims`; all 16 declared claim commands passed.
- **clean suite** — the same clean clone: `npm test`, `npm run build`,
  `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo package --allow-dirty` passed. `npm test` reported 13 Node/browser
  tests and all Rust unit/integration tests, including six configured-run
  production tests.
- **a11y** — `npm run test:a11y -- https://forge-sync.sociobot.in`: 0 axe
  violations, 0 serious/critical. Local output: `.factory/evidence/axe.json`.
- **screens** — local:
  `.factory/evidence/polish-2-local/home-mobile.png`,
  `demo-mobile.png`, `home-desktop.png`; deployed cold browser:
  `.factory/evidence/polish-2-live/home-mobile.png` and `demo-mobile.png`.
- **live** — cold checks at `https://forge-sync.sociobot.in/`, `/?demo=1`,
  `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and
  `/definitely-missing-polish-2`; deployed footer build was `39c9899c`.

## Review 2 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1a | Added the `configured-run-state-and-archive` claim and a two-repository configured local run which inspects selected SQLite and JSON archive paths. | `npm run test:claim:configured-state-archive`; clean claims. |
| F-2-1b | Added the `continuous-daemon-passes` claim. The real `daemon` command runs at a one-second configured interval; the test observes two discovery passes. Valid intervals now start at one second. | `npm run test:claim:continuous-passes`; clean claims. |
| F-2-1c | Added `configured-records-metadata`; it creates two local Git repositories and verifies discovery, branches, tags, labels, milestones, issues, target requests, and archive files. | `npm run test:claim:records-metadata`; clean claims. |
| F-2-1d | Added `configured-renders-pull-request-history`; its configured fixture checks description, review, inline comment, discussion comment, file/line, and pull-request label in target requests. | `npm run test:claim:pull-request-history`; clean claims. |
| F-2-1e | Added `configured-copied-body-attribution`; it checks author links, timestamps, and original GitHub URLs in rendered target bodies. | `npm run test:claim:copied-attribution`; clean claims. |
| F-2-1f | Added `configured-optional-git-archive`; it runs both `git_archive=true` and `false`, asserting JSON in both cases and a Git repository only when enabled. | `npm run test:claim:optional-archive`; clean claims. |
| F-2-2 | New-document initialization focuses and politely announces the sample heading for `?demo=1`, and the home h1 when leaving or returning. CTA, leave, Back, and Forward are covered. | `routes, metadata, focus, and phone first action work`; deployed cold browser check; live screenshots. |
| F-2-3 | Split the 26-word README sample sentence into two plain sentences. | `README.md`; `.factory/copy-audit.md`; clean suite. |
| F-2-4 | Replaced “usable copy”, mappings/audit, committed JSON, fixture, and README local-state wording with copy, record links/run history, JSON archive saved in Git, sample source files, and local record. | `.factory/copy-audit.md`; local/live screenshots; clean suite. |
| F-2-5 | Renamed every leave control to “Leave demo and build configuration” and made the destination explicit in surrounding copy. | browser route test; live `/?demo=1`; live demo screenshot. |

## Review 1 severity and structural findings

| Finding | Change retained and rechecked | Evidence |
| --- | --- | --- |
| B1 | Maintainer-focused job headline, audience, visible first-screen sample CTA, and phone-first copy remain in place. | browser first-screen test; local/live mobile screenshots. |
| B2 | `?demo=1`, `/demo/`, persistent sample banner, reset, leave action, separate `demo:forge-sync:` storage, shipped CLI fixture, and `forge-sync demo` remain isolated. | demo browser and CLI claims; live cold demo check. |
| B3 | Manifest now has 16 claims and runner enforcement for unique tags and runnable commands. | `npm run test:claims`; clean claims. |
| B4 | Product 404 and Static Web Apps status-404 override remain deployed. | live missing-route HTTP 404 and browser 404 check. |
| M1 | Dead paid checkout and all paid promises remain absent. | deployment static test; live link crawl. |
| M2 | Route metadata, canonicals, OG/Twitter image, apple icon, robots, and sitemap remain present. | clean static tests; live route metadata checks. |
| M3 | Hash focus/history restoration remains and document navigation focus is now also covered. | browser focus test; live cold CTA/leave/404 check. |
| M4 | Shared header/footer, legal links, Param Factory credit, and build ID remain on every page. | clean static tests; live route checks. |
| M5 | The revised copy audit retains consistent terminology and no banned words. | `.factory/copy-audit.md`; clean suite. |

## Every earlier claim, landing, README, and regression finding

The following rows name every prior ID explicitly. The original concrete
change is retained from `polish-1`; the evidence column records this round's
fresh verification rather than deferring any item.

| Finding IDs | Change made and still present | Evidence |
| --- | --- | --- |
| U01, U04, U07, U08, U11–U13, U17–U22, U27–U29 | Removed broad, quantitative, tracking, recovery, rate-limit, partial-failure, relay, paid, and privacy promises that did not have bounded visitor proof. | `.factory/copy-audit.md`; clean static/privacy claims; live source/cold check. |
| U02, U05, U09, U10, U15, U26 | Narrowed remaining fidelity language to the shipped sample and its real archive/state/rendering output. | `demo-completed-mirror`; clean claims; live demo screenshot. |
| U03, U21–U25 | Retained only tested MIT/no-analytics/offline facts and removed the unavailable paid tier, checkout, license, and refund claims. | `mit-license`, `website-no-tracking`, offline claims; static test; live crawl. |
| U06, U30–U31, U35 | Kept target support and record capabilities; now proves both target contracts and configured production runs. | `supported-targets`, configured metadata/history/attribution claims. |
| U14, U29, U45 | Kept the local builder/static-website privacy boundary with no token input and no off-origin website traffic. | configuration and privacy claims; live a11y/cold route checks. |
| U16, U39, U43 | Kept doctor and dry-run behavior with real read-only tests. | `doctor-read-only`, `dry-run-read-only`; clean claims. |
| U32–U34, U36–U44, U46–U47 | Removed unbounded README promises; retained only tested sample, dry-run, doctor, static privacy, and offline statements. | README source, clean claims, browser/offline tests. |
| L01–L10 | Plain job headline, organization/repository terminology, sample CTA, direct target names, and non-metaphorical wording remain. | copy audit; mobile screenshot; browser first-screen test. |
| L11–L22 | Process, archive, and pull-request record copy remains concrete and sample-scoped. | copy audit; demo CLI claim; live demo screenshot. |
| L23–L30 | Configuration and CLI controls use direct result labels; invented count, retry, failure, and universal-scriptability claims remain removed. | copy audit; configuration/doctor claims. |
| L31–L40 | Paid and unsupported roadmap/relay content remains removed. | static crawl test; live crawl. |
| L41–L47 | Privacy, closing, footer, builder feedback, and offline copy remain exact and tested. | copy audit; privacy/configuration/offline claims. |
| R01–R08 | README has short scope/audience/configuration/daemon/JSON wording. The daemon claim is now real rather than editorial. | README; `continuous-daemon-passes`; clean claims. |
| R09–R20 | README record/dry-run text is concrete; removed idempotence, retry, rate-limit, relay, and ambiguous wording. | README; git-ref, dry-run, configured production claims. |
| R21–R27 | README token, permission, archive, command, and deployment wording remains narrow and documented. | README; configuration/demo/static claims; clean build. |
| P0 | Dry run stays read-only and a following real run creates target objects. | `dry-run-read-only`; clean claims. |
| P1 | Reduced-motion focus rings and 390px keyboard traversal remain tested. | browser route test; live axe scan. |
| P2-exit | Configuration errors keep exit code 2. | `all_documented_configuration_failures_use_exit_code_two`; clean suite. |
| P2-host | Cache and security headers remain in host configuration and deployed responses. | deployment test; live header capture. |

No review finding remains unresolved.
