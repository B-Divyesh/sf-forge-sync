# Perfection loop polish 4 — complete closure

Reviewed inputs: `.factory/review-1.md`, `.factory/review-2.md`,
`.factory/review-3.md`, `.factory/review-4.md`, and
`.factory/polish-1.md` through `.factory/polish-3.md`.

Repair commit: `d7460b7d9416c422eb336a381cb1f31847db00a2`.
Live build: <https://forge-sync.sociobot.in> (footer `d7460b7d`).

Evidence abbreviations below:

- **clean claims** — `/tmp/forge-sync-polish4-clean.A0kfRB/repo`:
  `npm ci && npm run test:claims`; all 21 claim commands passed.
- **clean suite** — the same clone: `npm test`, `npm run build`,
  `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo package --locked --allow-dirty` passed.
- **live cold** — `.factory/evidence/polish-4-live/cold-check.json`,
  `home-mobile.png`, `demo-mobile.png`, and `home-desktop.png`.
- **live a11y** — `npm run test:a11y -- https://forge-sync.sociobot.in` and
  `.factory/evidence/polish-4-live/verify/verify.json`: 0 axe violations,
  no console errors, title/lang/h1/main/alt/button checks passed.

## Review 4 findings

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| F-4-1 / B2 | Replaced the home-link-only cleanup with prefix-only cleanup for all same-tab non-demo links. Non-demo `pageshow` clears stale demo state after history traversal; demo initialization preserves a session on reload and demo-to-demo movement. | `@claim:demo-browser-isolation`; clean claims; live `/?demo=1` → Privacy and `/demo/` → Terms checks; `cold-check.json`; live demo screenshot. |
| F-4-2 | Removed the public Docker build/run path rather than leaving an untestable installation promise. A static regression test rejects Docker/Podman/container wording in the README. | `README documents only tested installation paths`; clean suite; pushed README and GitHub source URL. |
| F-4-3 | Rewrote the builder result to “Set the named token environment variables before you run the CLI.” | `@claim:configuration-has-no-token-field`; local and live cold checks; `home-mobile.png`. |

## Review 1 severity and site findings

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| B1 | Retained the direct seven-word job headline, maintainer audience, target names, one primary sample action, and mobile copy-before-art layout. | browser first-screen assertions; live `/`; `home-mobile.png`, `home-desktop.png`. |
| B2 | Retained the real CLI/browser sample, recording, sample banner, reset, and separate storage namespace; this round completes every exit path. | `@claim:demo-completed-mirror`, `@claim:demo-browser-isolation`; live `/?demo=1`, `/demo/`; `demo-mobile.png`. |
| B3 | Retained the non-empty 21-entry manifest, unique tagged-test enforcement, and clean claim runner. | `each declared claim has exactly one tagged test and a runnable command`; clean claims. |
| B4 | Retained real `/demo/`, product 404, Static Web Apps 404 override, and return route. | live `/round-four-missing` HTTP 404; `cold-check.json`; live a11y. |
| M1 | The unavailable paid checkout, price, and license promises remain absent. | static crawl test; live link crawl. |
| M2 | Route titles, descriptions, canonical/OG/Twitter metadata, icons, robots, sitemap, and social art remain on every route. | static metadata test; live `/`, `/demo/`, `/privacy/`, `/terms/`; `cold-check.json`. |
| M3 | Hash and document navigation retain focus, announcements, scroll restoration, and Back/Forward context. | browser route test; live history checks in `cold-check.json`. |
| M4 | Header/footer skeleton, legal links, Param Factory credit, version, and build ID remain on all routes. | static skeleton test; live route crawl and footer `d7460b7d`. |
| M5 | The terminology table, plain-word copy audit, and 22-word ceiling remain; the final dynamic instruction is now correct. | `.factory/copy-audit.md`; configuration claim; live screenshots. |

## Original unlisted-claim findings

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| U01, U04, U08, U17–U19, U22–U25, U27–U28, U33–U34, U36–U38, U40–U41, U44, U47 | Unsupported absolute, fabricated-count, paid, relay, release-binary, retry, partial-failure, token, and cached-license statements remain removed. | copy audit; static crawl/privacy tests; live `/` and pushed README. |
| U02, U05, U09–U10, U15, U26 | Remaining fidelity wording stays scoped to the shipped sample and is proved through real SQLite, rendered pull-request data, canary scans, and a real archive commit. | `@claim:demo-completed-mirror`; live `/?demo=1` and `/demo/`. |
| U03, U07, U14, U29, U32, U45–U46 | Retained MIT, no-analytics, token-free builder, website boundary, and offline statements each have an isolated claim. | `@claim:mit-license`, `website-no-tracking`, `configuration-has-no-token-field`, `offline-demo-after-first-visit`; live a11y/cold check. |
| U06, U11–U13, U30–U31, U35, U42 | Target contracts, refs, metadata, pull-request rendering, attribution, and archive outputs remain covered by local configured fixtures. | target, Git-ref, metadata, pull-request, attribution, archive claims; clean claims. |
| U16, U39, U43 | Doctor and dry-run claims remain real, bounded, and read-only. | `@claim:doctor-read-only`; `@claim:dry-run-read-only`; clean claims. |
| U20 | The narrowed status/sync JSON promise has its own parseable-output claim. | `@claim:status-sync-json-output`; clean claims; README. |
| U21 | Only the tested MIT statement remains from price/license copy. | `@claim:mit-license`; live `/terms/`. |

## Landing and README copy findings

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| L01–L10 | Direct job headline, organization/repository vocabulary, named targets, sample action, and literal wording remain. | copy audit; browser first-screen test; live home screenshots. |
| L11 | The process heading remains “Copy code, issues, and pull-request history.” | copy audit; live `/#how`. |
| L12–L22 | Rate-limit/internal jargon stays removed; sample archive and pull-request details remain concrete and tested. | demo/configured claims; live demo screenshots. |
| L23–L30 | Configuration and CLI controls retain result labels; invented counts and broad retry/failure/scriptability language remain absent. | configuration, doctor, JSON claims; live home. |
| L31–L40 | Paid and unsupported roadmap/relay material remains absent. | static crawl test; live link crawl. |
| L41–L47 | Privacy, closing, footer, validation, clipboard, and offline words remain plain and tested. | copy audit; privacy/configuration/offline claims; live `/privacy/` and `/`. |
| R01–R05, R07–R11, R13–R27 | README scope, audience, configuration, record, dry-run, privacy, and development copy remains short and concrete. | pushed README; clean suite; GitHub source URL. |
| R06 | The status/sync JSON sentence has its dedicated claim. | `@claim:status-sync-json-output`; clean claims. |
| R12 | Visitor and human CLI terms remain aligned around record links and dated run history. | `@claim:configured-record-links-run-history`; live demo. |

## Review 2, review 3, and regression findings

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| F-2-1a–F-2-1f | Configured production claims remain for state/archive, daemon passes, metadata, pull-request history, attribution, and optional Git archive behavior. | six named configured-run claim commands; clean claims. |
| F-2-2 | Document-route focus, announcements, CTA/leave behavior, Back, and Forward remain tested. | browser route test; live `cold-check.json`. |
| F-2-3 | The former 26-word sample sentence remains split. | README; copy audit; clean suite. |
| F-2-4 | “usable”, mapping/audit, committed JSON, fixture, and local-state visitor copy remains removed or rewritten. | copy audit; live demo screenshots. |
| F-2-5 | Every leave action still names its result. | demo browser claim; live demo. |
| F-3-1 | Rust 1.88 remains the documented/package minimum with exact-toolchain coverage. | `@claim:minimum-rust-build`; clean claims. |
| F-3-2 | The self-hosted SVG and transcript remain generated from the real CLI sample. | `@claim:demo-recording-current-output`; live recording URL. |
| F-3-3 | Status and sync JSON remain one parseable document with documented fields. | `@claim:status-sync-json-output`; clean claims. |
| F-3-4 | Record links/run history terminology and configured SQLite evidence remain aligned. | `@claim:configured-record-links-run-history`; live demo. |
| F-3-5 | The 50-repository / 5,000-issue deterministic benchmark remains within its safety-margin limits. | `@claim:organization-scale-performance`; 26.676 s / 2.842 s clean result. |
| F-3-6 | Query demo still promotes its sample heading to the sole h1 and focuses it. | browser route test; live `/?demo=1`. |
| F-3-7 | All three desktop facts remain within the 1440×900 first screen. | browser bounds test; live bottoms 814.22, 814.22, 844.03 in `cold-check.json`. |
| F-3-8 | “Sample data” remains the sole visitor term. | copy audit; live home. |
| F-3-9 | The descriptive sample-flow heading remains in place. | copy audit; live `/#how`. |
| P0 | Dry-run then real-run behavior remains non-mutating then complete. | `@claim:dry-run-read-only`; clean claims. |
| P1 | Reduced-motion 3 px focus and phone keyboard traversal remain covered. | browser route test; live a11y. |
| P2-exit | Documented configuration errors still exit 2. | Rust configuration-exit test; clean suite. |
| P2-host | CSP, policy headers, and immutable assets remain configured and served. | deployment static test; live header check. |

## Live release re-check

Cold fresh contexts verified the direct job/audience/action at 390×844 and
1440×900, query/path demo entry, sample banner/reset, Privacy/Terms exits,
Back/Forward, offline reload, route titles/metadata, shared skeleton, legal
links, and the product 404. Every HTTP link returned 200 except the intentional
404 route; mailto and in-page links were explicit. Lighthouse is 100/100 for
performance/accessibility. The glacial ceramic artwork, mineral palette,
serif/sans pairing, cobalt mapping line, and ember status mark remain intact.

No finding of any severity is deferred.
