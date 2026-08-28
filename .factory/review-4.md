# Adversarial first-read review 4 — forge-sync

**Verdict: FAIL**

Reviewed 28 August 2026 against live
<https://forge-sync.sociobot.in> and commit
`943e558a37d4d0db5bf316b33a3fd7d9f28df6a1`. This was a fresh mobile,
desktop, demo, claim, CLI sandbox, copy, history, structure, accessibility, and
link review. One blocking, one major, and one minor finding remain.

## Thirty-second cold read

Fresh Chromium contexts with empty storage opened `/` at 390 × 844 and
1440 × 900. No scrolling occurred before recording this result.

| Question | Phone | Desktop |
| --- | --- | --- |
| What does it do? | Mirrors a GitHub organization to another forge. | The same. |
| For whom? | Maintainers who need a Forgejo, Codeberg, or GitLab copy with issues and pull-request history. | The same. |
| What should I click first? | `Try it with sample data`. | The same. |

The exact passing text was “Mirror your GitHub organization to another
forge.”, “For maintainers who need a copy on Forgejo, Codeberg, or GitLab with
issues and pull-request history.”, and “Try it with sample data” / “See a
completed sample mirror; nothing is saved.” The primary action ended at
`y=543.05` on the phone and `y=723.75` on desktop. All three facts ended above
the fold (`y=749.94` phone; `y=844.03` desktop). Both pages stayed at
`scrollY=0`, had no horizontal overflow, console error, or cross-origin
request.

## Findings, ordered by severity

### BLOCKING F-4-1 / B2 — an ordinary demo exit leaves demo storage behind

**Quote/location:** the persistent banner promises “Demo — sample data,
nothing is saved.” The `demo-browser-isolation` claim says “leaving demo
discards it”. From `/?demo=1`, the shared header offers `Privacy`; selecting it
opens `/privacy/`, removes the demo banner, but leaves
`demo:forge-sync:session` in local storage.

**Exact evidence:** a fresh phone context started with
`real:sentinel=keep`. Demo entry added only `demo:forge-sync:session`. Reset
changed that demo value and preserved the sentinel. The explicit “Leave demo
and build configuration” action removed the demo key. After entering demo
again and choosing `Privacy`, `/privacy/` contained both the sentinel and the
demo key while showing no demo banner. Entering from `/` and using browser Back
also returned to `/` with the demo key still present and the banner hidden. The
same defect applies to other links that leave for a URL other than `/`. In
`site/main.js`, cleanup is attached only to
`document.querySelectorAll('a[href="/"]')`.

**Why this blocks acceptance:** the user has visibly left demo mode, yet demo
state persists. This contradicts the sandbox contract and a listed claim. The
passing claim test checks only the dedicated leave link, so it does not prove
the full claim. This is a regression/half-fix of review 1 finding B2.

**Concrete fix:** remove every `demo:forge-sync:` key before any navigation
from a demo page to a non-demo URL, including Privacy, Terms, the wordmark,
external links, and browser history exits where observable. Keep demo keys on
demo-to-demo navigation and reload. Extend `@claim:demo-browser-isolation` to
exercise each exit route, Back/Forward, and both `/?demo=1` and `/demo/`, while
asserting that a non-demo sentinel never changes.

### MAJOR F-4-2 — the documented container path is an unlisted, untested claim

**Quote/location:** README Install, “Or run the container:” followed by
`docker build -t forge-sync .` and a `docker run … forge-sync sync …` command.

**Evidence:** `.factory/claims.json` has no container/image claim. No test
mentions Docker, Podman, the Dockerfile, image startup, bind-mounted
configuration, or the documented `sync` invocation. Neither Docker nor Podman
is installed in this review environment, and the preceding handoff explicitly
records the wrapper as unexecuted. All 21 declared commands can therefore pass
without building the documented install path.

**Why this matters:** “Or run the container” is a production installation and
execution promise. A maintainer may choose it instead of the tested Rust build,
but the claims contract gives no evidence that the image builds or that the
shown mount/environment command reaches a working sync.

**Concrete fix:** add a `container-configured-sync` claim. In CI, build the
image from a clean checkout, run its binary/version, then use a temporary bind
mount and local source/target contract servers to run the documented sync and
assert target, state, and archive output. If that test cannot run on every
claims build, remove the container instructions until it can.

### MINOR F-4-3 — the configuration success message gives the wrong next step

**Quote/location:** landing dynamic status in `site/main.js`: “Configuration
ready. Add token environment-variable names when you run the CLI.”

**Why this loses a first-time user:** the generated configuration already
contains `token_env = "GITHUB_TOKEN"` and `token_env = "FORGE_TOKEN"`. At run
time the user sets values for those variables; they do not add variable names.
The wording contradicts the generated result and README commands.

**Concrete rewrite:** “Configuration ready. Set the named token environment
variables before you run the CLI.”

## Copy audit

Words are whitespace-delimited. Hyphenated terms count once. Code blocks,
generated TOML, captured terminal output, URLs, and the wordmark are not
sentences and are excluded. Inline commands are counted. Headings, labels, actions,
facts, prose, and live messages are included because they are independently
presented copy. No unit exceeds 22 words. No supplied banned marketing word
appears. The only landing-copy flag is F-4-3; the README claim flag is F-4-2.

### Landing page and demo state

| Area | Exact copy unit | Words | Result |
| --- | --- | ---: | --- |
| Access/navigation | “Skip to content”; “Demo”; “How it works”; “Privacy” | 3; 1; 3; 1 | Pass |
| Demo banner | “Demo — sample data, nothing is saved.” | 7 | Listed claim; F-4-1 contradicts it on an ordinary exit |
| Demo actions | “Reset demo”; “Leave demo and build configuration” | 2; 5 | Result-naming actions |
| Demo panel | “Sample mirror ready”; “See a completed sample mirror.” | 3; 5 | Pass |
| Demo panel | “Harbor Cooperative’s harbor-tools repository has two branches, one tag, and one pull-request record.” | 13 | `demo-completed-mirror` |
| Demo panel | “It includes links between GitHub and target records, dated run history, and a JSON archive saved in Git.” | 18 | `demo-completed-mirror`; `configured-record-links-run-history` |
| Demo recording | “Captured from the current CLI and bundled sample data.”; “Read the captured transcript” | 9; 4 | `demo-recording-current-output` |
| Demo link | “Open the full demo record” | 5 | Result-naming link |
| First screen | “GitHub organization mirror”; “Mirror your GitHub organization to another forge.” | 3; 7 | Pass |
| Audience | “For maintainers who need a copy on Forgejo, Codeberg, or GitLab with issues and pull-request history.” | 16 | Listed target and metadata capabilities |
| First actions | “Try it with sample data”; “See a completed sample mirror; nothing is saved.”; “Build your configuration” | 5; 8; 3 | Result-naming actions |
| First facts | “Free under the MIT License.”; “No website analytics.”; “Works offline after your first visit.” | 5; 3; 6 | Listed claims |
| Art labels | “GitHub organization”; “JSON archive”; “Your forge” | 2; 2; 2 | Pass |
| Sample flow | “How the sample is arranged”; “Copy code, issues, and pull-request history.” | 5; 6 | Pass |
| Sample flow labels | “01 / source”; “GitHub organization”; “02 / record”; “Local archive”; “03 / target”; “Another forge” | 3; 2; 3; 2; 3; 2 | Pass |
| Sample flow | “The sample starts with Harbor Cooperative’s harbor-tools repository.” | 8 | `demo-completed-mirror` |
| Sample flow | “JSON records, links between GitHub and target records, and dated run history stay together.” | 14 | `configured-record-links-run-history` |
| Sample flow | “The sample shows branches, tags, and a readable pull-request issue.” | 10 | `demo-completed-mirror` |
| Sample contents | “What the sample contains”; “Keep issues and pull-request history with the code.” | 4; 9 | Pass |
| Sample contents | “The completed sample makes each copied record visible before you connect a real organization.” | 14 | `demo-completed-mirror` |
| Git ledger | “Git data”; “Branches and tags”; “Two branches and one version tag.” | 2; 3; 6 | `demo-completed-mirror` |
| Issue ledger | “Issue record”; “Pull-request discussion”; “Author, review, file name, and line number.” | 2; 2; 7 | `demo-completed-mirror` |
| Local ledger | “Local record”; “Links between GitHub and target records”; “One link and three dated run-history entries.” | 2; 6; 7 | `configured-record-links-run-history` |
| Archive ledger | “Archive”; “JSON archive saved in Git”; “A Git commit records the sample archive.” | 1; 5; 7 | `demo-completed-mirror` |
| Builder | “Build locally”; “Create a configuration without token values.” | 2; 6 | `configuration-has-no-token-field` |
| Builder | “This browser tool creates text only.”; “It does not ask for a token.” | 6; 7 | `configuration-has-no-token-field` |
| Builder controls | “GitHub organization”; “Target forge”; “Forgejo”; “Codeberg”; “GitLab”; “Target URL”; “Target owner or namespace”; “Copy configuration”; “Download configuration” | 2; 2; 1; 1; 1; 2; 4; 2; 2 | Pass; actions name results |
| Builder next step | “Then check access without making changes:” | 6 | `doctor-read-only` |
| CLI sample | “Run the sample”; “Inspect a mirror before using your data.” | 3; 8 | Pass |
| CLI sample | “Separate output.”; “The command creates a new temporary directory.” | 2; 7 | `demo-completed-mirror` |
| CLI sample | “Disposable data.”; “Remove the printed directory when you finish.” | 2; 7 | Pass |
| CLI sample | “Sample source files.”; “The source JSON ships in examples/sample-mirror.” | 3; 6 | `demo-completed-mirror` |
| Closing | “Start with sample data”; “See the sample, then build your configuration.”; “Try it with sample data” | 4; 8; 5 | Pass |
| Footer | “Mirror GitHub organizations to another forge.”; “Privacy”; “Terms”; “GitHub source”; “Built by Param Factory · v0.1.0 · build 943e558a” | 6; 1; 1; 2; 9 | Pass |
| Demo success | “Demo reset.”; “The sample data is new.” | 2; 5 | Listed claim; F-4-1 covers incomplete exit behavior |
| Route announcements | “Moved to Mirror your GitHub organization to another forge.”; “Moved to See a completed sample mirror.”; “Moved to Inspect a completed sample mirror.”; “Moved to Privacy”; “Moved to Terms”; “Moved to That page does not exist.”; “Moved to Copy code, issues, and pull-request history.”; “Moved to Create a configuration without token values.” | 9; 7; 7; 3; 3; 7; 8; 8 | Pass |
| Configuration success | “Configuration ready.”; “Add token environment-variable names when you run the CLI.” | 2; 9 | **F-4-3** |
| Clipboard feedback | “Configuration copied.”; “Clipboard access was blocked.”; “Select the configuration and copy it manually.” | 2; 4; 7 | Pass |
| Dynamic error | “Complete the fields above to generate the configuration.” | 8 | Pass |
| Validation | “Use a valid GitHub organization name.”; “Use a valid target owner or namespace.”; “Enter a complete target URL, including https://.”; “Use HTTPS for a remote target.” | 6; 7; 7; 6 | Pass |
| Offline | “You’re offline.”; “The sample and configuration builder remain available after a first visit.” | 2; 11 | `offline-demo-after-first-visit` |

### README

| Area | Exact copy unit | Words | Result |
| --- | --- | ---: | --- |
| Title/summary | “forge-sync”; “Mirror your GitHub organization to another forge.” | 1; 7 | Pass |
| Audience | “forge-sync is for maintainers who need an independent copy on Forgejo, Codeberg, or GitLab.” | 14 | Listed targets |
| Summary | “It writes a local record and a JSON archive alongside the target copy.” | 13 | `configured-run-state-and-archive` |
| Heading | “Try the completed sample” | 4 | Pass |
| Sample | “Run this before configuring a real organization:” | 7 | Pass |
| Sample | “Open the isolated browser sample in one click.” | 8 | `demo-browser-isolation` |
| Sample | “The command creates a new temporary directory and prints its path.” | 11 | `demo-completed-mirror` |
| Sample | “It does not read your configuration or token values.” | 9 | `demo-completed-mirror` |
| Sample | “The output contains the fictional Harbor Cooperative harbor-tools repository, two branches, and one tag.” | 14 | `demo-completed-mirror` |
| Sample | “It also contains a pull-request record and a committed JSON archive.” | 11 | `demo-completed-mirror` |
| Sample | “SQLite stores links between GitHub and target records plus dated run history.” | 12 | `configured-record-links-run-history` |
| Sample | “Delete the printed directory when you finish.” | 7 | Pass |
| Sample | “The source records are in examples/sample-mirror.” | 6 | `demo-completed-mirror` |
| Sample | “See .factory/demo.md for browser and CLI sandbox details.” | 8 | Pass |
| Install | “Install”; “Build from source with Rust 1.88 or newer:” | 1; 8 | `minimum-rust-build` |
| Install | “Or run the container:” | 4 | **F-4-2: unlisted claim** |
| Configuration | “Configure a real mirror”; “Create forge-sync.toml.” | 4; 2 | Pass |
| Configuration | “Name the environment variables that hold tokens.”; “Do not put token values in this file.” | 7; 8 | Configuration contract |
| Run | “Check access before making changes, then run a pass:” | 9 | `doctor-read-only` |
| Run | “Run forge-sync daemon --config forge-sync.toml for continuous passes.” | 8 | `continuous-daemon-passes` |
| Run | “Add --json to status or sync when a script needs JSON output.” | 12 | `status-sync-json-output` |
| Records heading | “What it records” | 4 | Pass |
| Record | “repository discovery, branches, tags, labels, milestones, and issues;” | 8 | `configured-records-metadata` |
| Record | “pull-request descriptions, reviews, inline comments, and discussion comments in a labeled target issue;” | 13 | `configured-renders-pull-request-history` |
| Record | “the author, time, and original GitHub link in copied bodies;” | 10 | `configured-copied-body-attribution` |
| Record | “links between GitHub and target records plus dated run history in SQLite; and” | 13 | `configured-record-links-run-history` |
| Record | “JSON snapshots, optionally committed to a local Git archive.” | 9 | `configured-optional-git-archive` |
| Dry run | “forge-sync sync --dry-run reports planned changes.” | 6 | `dry-run-read-only` |
| Dry run | “It does not change either forge, Git data, local state, dated run history, or the JSON archive.” | 17 | `dry-run-read-only` |
| Scale heading | “Scale acceptance” | 2 | Pass |
| Scale | “The local acceptance benchmark mirrors 50 repositories with 5,000 issues.” | 10 | `organization-scale-performance` |
| Scale | “It checks an initial pass against 30 minutes and a no-change pass against two minutes.” | 15 | `organization-scale-performance` |
| Scale | “The test reserves a 20% margin, so its limits are 24 minutes and 96 seconds.” | 15 | `organization-scale-performance` |
| Scale | “It uses local forge endpoints to remove internet delay and prints the runner’s operating system, architecture, CPU parallelism, and measured times.” | 21 | `organization-scale-performance` |
| Development | “Development”; “The static documentation build is written to dist/site.”; “Run it locally with npm run dev.” | 1; 8; 7 | Verified build instructions |
| Privacy | “Privacy and license”; “The browser sample stores one demo:forge-sync: marker.” | 3; 7 | `demo-browser-isolation`; F-4-1 |
| Privacy | “The configuration builder has no token field.”; “Read the full privacy policy and terms.” | 7; 7 | `configuration-has-no-token-field`; pass |
| License | “forge-sync is released under the MIT License.” | 7 | `mit-license` |

Terminology is otherwise consistent: `organization`, `repository`, `pull
request`/the adjectival `pull-request`, `configuration`, `sample`/`demo`,
`local record`/`JSON archive`, `links between GitHub and target records`, and
`dated run history`. The banned-word scan found no `leverage`, `seamless`,
`effortless`, `robust`, `powerful`, `intuitive`, `reimagine`, `supercharge`,
`unlock`, `delightful`, `journey`, `ecosystem`, or `AI-powered` usage.

## Demo and sandbox verification

- One click from the first screen opened `/?demo=1`. Before scrolling, the
  phone showed the persistent banner, both controls, “See a completed sample
  mirror”, realistic Harbor Cooperative records, and the captured real CLI
  output. The route had one h1 and focused it.
- Entry added only `demo:forge-sync:session`; Reset changed only that key; the
  explicit leave action removed it; the non-demo sentinel survived. F-4-1 is
  the separate ordinary-navigation failure.
- The full browser flow made no cross-origin request. After one online visit,
  an offline reload retained the demo, banner, and working configuration
  builder.
- The real CLI was run from a new
  `/tmp/forge-sync-review4-cli.H0PA5g` directory with canary source/target
  token values. It created only a new
  `/tmp/forge-sync-demo-21091-1787927926492411091` output tree, reported two
  branches, one tag, one issue, one pull-request record, one record link, three
  dated run-history entries, and a committed JSON archive. The output tree did
  not contain either canary; Git commit `cc4c441` recorded the archive.

## Declared claims

The clean clone was `/tmp/forge-sync-review4-clean.Ud98Fz/repo`. `npm ci`
reported zero vulnerabilities. `npm run test:claims` invoked every command in
`.factory/claims.json` and reported **21 passed**. The command result and the
independent observation are distinguished because F-4-1 exposes a missing
branch in one passing test.

| Claim ID | Declared command | Independent result |
| --- | --- | --- |
| `demo-completed-mirror` | PASS | PASS; CLI temp output and canary scan also passed |
| `demo-browser-isolation` | PASS | **FAIL — F-4-1: Privacy exit retains demo storage** |
| `configuration-has-no-token-field` | PASS | PASS |
| `website-no-tracking` | PASS | PASS; no cross-origin request observed |
| `supported-targets` | PASS | PASS |
| `git-refs-mirror` | PASS | PASS |
| `dry-run-read-only` | PASS | PASS |
| `doctor-read-only` | PASS | PASS |
| `mit-license` | PASS | PASS |
| `offline-demo-after-first-visit` | PASS | PASS live with network disabled |
| `configured-run-state-and-archive` | PASS | PASS |
| `continuous-daemon-passes` | PASS | PASS |
| `configured-records-metadata` | PASS | PASS |
| `configured-renders-pull-request-history` | PASS | PASS |
| `configured-copied-body-attribution` | PASS | PASS |
| `configured-optional-git-archive` | PASS | PASS |
| `minimum-rust-build` | PASS | PASS on exact Rust 1.88.0 |
| `demo-recording-current-output` | PASS | PASS; live recording asset returned 200 |
| `status-sync-json-output` | PASS | PASS |
| `configured-record-links-run-history` | PASS | PASS |
| `organization-scale-performance` | PASS | PASS; 50 repositories/5,000 issues took 24.347 s initial and 2.408 s incremental on Linux x86_64 with two reported CPUs |

F-4-2 is the only claim-like landing/README statement without a manifest
entry. Claim-like statements on Demo, Privacy, and Terms map to the demo,
configuration, privacy, configured-run, target, or MIT entries above.

## Earlier finding verification

Every earlier review, polish record, verification, and handoff was read. The
following checks use the current live site and current code, not closure labels.

| Earlier ID | Current result and evidence |
| --- | --- |
| B1 | Fixed. Both cold viewports name the job, maintainer audience, targets, sample action, and result above the fold. |
| B2 | **Regressed/half-fixed as F-4-1.** The browser/CLI sample, recording, banner, Reset, explicit leave, prefix, and canary isolation work; ordinary navigation can leave demo storage behind. |
| B3 | The original missing-manifest defect is fixed: 21 unique tagged commands ran. F-4-2 is a new unlisted README claim. |
| B4 | Fixed. `/demo/` is real; an unknown route returned the designed page with HTTP 404 and a return action. |
| M1 | Fixed. Paid checkout and pricing copy remain absent; every destination discovered on the 200 routes returned 200. |
| M2 | Fixed. Every route has title, description, canonical, OG/Twitter fields, SVG favicon, apple icon, and social image; robots and sitemap returned 200. |
| M3 | Fixed. Live direct-route focus lands on the h1; the browser suite passed CTA, leave, hash, Back, Forward, focus, announcement, and scroll checks. |
| M4 | Fixed. Home, demo, legal, and 404 routes share the same header/footer, Privacy/Terms links, factory credit, version, and build `943e558a`. |
| M5 | Fixed for its original jargon and terminology. F-4-3 is a different incorrect dynamic instruction. |
| U01, U04, U08, U17–U19, U22–U25, U27–U28, U33–U34, U36–U38, U40–U41, U44, U47 | Fixed. The exact unsupported absolute, fabricated-count, paid, relay, release-binary, retry, partial-failure, broad token, test-network, and cached-license promises remain absent. |
| U02, U05, U09–U10, U15, U26 | Fixed. Remaining sample fidelity text is narrow and covered by the real CLI demo/archive/token-scan test. |
| U03, U07, U14, U29, U32, U45–U46 | Fixed. Retained MIT, analytics, token-free builder, browser boundary, and offline statements have isolated tests and live network checks. |
| U06, U11–U13, U30–U31, U35, U42 | Fixed. Target creation, refs, metadata, pull-request rendering, attribution, and archive behavior have local contract tests. |
| U16, U39, U43 | Fixed. Doctor and dry-run read-only commands passed their isolated tests. |
| U20 | Fixed. The narrowed `status`/`sync` JSON claim is listed and parses both complete documents. |
| U21 | Fixed. Paid/unlimited copy remains absent; the retained MIT statement passed its claim. |
| L01–L10 | Fixed. The live first screen uses the direct headline, full words, named targets, and result-naming actions; old proof slogans are absent. |
| L11–L22 | Fixed. Process, metadata, archive, and pull-request text is concrete and sample-scoped; the old crossing/layer/ref/anchor metaphors are absent. |
| L23–L30 | Fixed. Configuration and CLI controls use full result names; invented terminal counts, recovery, partial-failure, and universal scripting language are absent. |
| L31–L40 | Fixed. Paid-tier, license, roadmap, relay, and “sharp edges” marketing copy remains absent. |
| L41–L47 | Fixed. Privacy, closing, footer, validation, clipboard, and offline copy uses the prior concrete rewrites. |
| R01–R05, R07–R11, R13–R27 | Fixed. The current README uses short scope, setup, record, dry-run, privacy, and development sentences; the removed jargon and broad promises remain absent. |
| R06 | Fixed. The narrowed JSON sentence has `status-sync-json-output`. |
| R12 | Fixed. README, landing, demo, and human CLI consistently say links between GitHub and target records and dated run history. |
| F-2-1a–F-2-1f | Fixed. All six production promises now have configured-run claims; every command passed. |
| F-2-2 | Fixed. New-document and history navigation focus/announcement passed locally and live. |
| F-2-3 | Fixed. The former 26-word sample sentence is split; current maximum README unit is 21 words. |
| F-2-4 | Fixed. “usable”, “mapping/audit”, “committed JSON”, “fixture”, and README “local state” visitor copy is absent or replaced. |
| F-2-5 | Fixed. The dedicated action says “Leave demo and build configuration”; its incomplete coverage is separately F-4-1. |
| F-3-1 | Fixed. README/Cargo say Rust 1.88; the exact-toolchain build and tests passed. |
| F-3-2 | Fixed. The generated SVG recording/transcript matches current real demo output and is shown on both demo surfaces. |
| F-3-3 / U20 / R06 | Fixed. The JSON claim and configured parsing test passed. |
| F-3-4 / R12 | Fixed. Terminology and configured SQLite record/history assertions are aligned. |
| F-3-5 | Fixed. The deterministic 50-repository/5,000-issue benchmark passed both limits with margin. |
| F-3-6 | Fixed. Query demo promotes its visible sample heading to the sole h1 and focuses it. |
| F-3-7 | Fixed. All three desktop facts ended above `y=900`. |
| F-3-8 | Fixed. “Start with a fixture” is now “Start with sample data”. |
| F-3-9 | Fixed. The heading now says “Copy code, issues, and pull-request history.” |
| P0 | Fixed. The dry-run-then-real regression test passed. |
| P1 | Fixed. Reduced-motion keyboard tests and live axe scan passed with the 3 px focus ring. |
| P2-exit | Fixed. Documented configuration failures still return exit 2 in the full suite. |
| P2-host | Fixed. Live HTML and hashed assets carry the configured security and cache headers. |

## Structure, accessibility, links, and visual identity

- `/`, `/?demo=1`, `/demo/`, `/privacy/`, and `/terms/` returned 200. The
  missing route returned 404 with product navigation. Every route had a plain
  route-specific title, one h1, one main, `lang=en`, description, canonical,
  OG image, favicon, apple icon, and focused route heading.
- The complete link crawl covered all links on home, query demo, demo, privacy,
  terms, and 404. Every HTTP destination other than the intentional current
  404 returned 200; `mailto:` links were exempt. External GitHub links are
  visibly marked.
- Live HTML is byte-identical to the current clean build
  (`e48009296dd975e289bdbc84d5ebb80aeaeb73b5d7f0754326231c78371d7764`).
  The live footer reports build `943e558a`. CSP, HSTS, referrer, nosniff,
  frame, and permissions headers are present. Hashed assets use one-year
  immutable caching.
- `npm run test:a11y -- https://forge-sync.sociobot.in` reported zero axe
  violations. The URL verifier reported one h1, `lang=en`, a main landmark,
  zero missing alt attributes, zero unlabeled buttons, and zero console errors.
  Phone reduced-motion keyboard traversal also passed in the repository suite.
- The production build contains 6.18 kB raw JavaScript (2.66 kB gzip) and
  14.93 kB CSS. It produced `dist/site` successfully.
- The glacial ceramic transfer image, mineral palette, serif/sans pairing,
  cobalt mapping line, and ember status mark remain recognizably specific to
  repository mirroring. This is not a generic centered-gradient or three-card
  SaaS template. Original asset provenance remains recorded in
  `.factory/design.md`.

## Missed leverage

No missing AI feature was identified. The core job is deterministic transfer,
record mapping, and audit/archive output; an AI step would make this operation
less predictable rather than complete an implied task. The product already
includes the expected sync, local archive, configuration export, sample data,
and status JSON. No decorative AI control, provider key, Azure endpoint, or
runtime model call exists.

## Additional verification

- `npm test`: passed 16 Node/browser/static tests plus all Rust unit,
  integration, and documentation tests. The scale test is intentionally run by
  its claim command.
- `npm run build`: passed and produced `dist/site`.
- `/opt/fleet/lib/verify-url.sh`: passed the live home route in 873 ms with no
  console errors.
- Live `/robots.txt`, `/sitemap.xml`, favicon, apple icon, OG image, recording,
  and transcript all returned 200.

No product code was modified during this review.

## What would make this perfect

Make demo cleanup cover every way a visitor leaves demo mode and strengthen
the isolation claim test around those routes. Add a clean, reproducible
container claim for the documented Docker workflow or remove that workflow
from README. Correct the configuration success sentence, then rerun this full
review. Zero remaining findings—not merely a green existing suite—is the PASS
condition.
