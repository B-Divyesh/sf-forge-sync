# Adversarial first-read review 3 — forge-sync

**Verdict: FAIL**

Reviewed 28 August 2026 against live
<https://forge-sync.sociobot.in> and commit
`18ab53872546650039facbff88a6f04050a301ca`. This was a fresh mobile,
desktop, demo, copy, claim, sandbox, history, structure, and accessibility
review. Three blocking findings, two major findings, and four minor findings
remain.

## Thirty-second cold read

Fresh Chromium contexts opened `/` at 390 × 844 and 1440 × 900. Browser
storage was empty and no scrolling occurred before this result was recorded.

| Question | Phone | Desktop |
| --- | --- | --- |
| What does it do? | Mirrors a GitHub organization to another forge. | Mirrors a GitHub organization to another forge. |
| For whom? | Maintainers who need a Forgejo, Codeberg, or GitLab copy with issues and pull-request history. | The same. |
| What should I click first? | `Try it with sample data`. | The same. |

The exact passing text was “Mirror your GitHub organization to another
forge.”, “For maintainers who need a copy on Forgejo, Codeberg, or GitLab with
issues and pull-request history.”, and “Try it with sample data” / “See a
completed sample mirror; nothing is saved.” The primary action ended at y=543
on the phone and y=805 on desktop. Neither viewport had horizontal overflow.

## Findings, ordered by severity

### BLOCKING F-3-1 — the documented minimum Rust version cannot build the locked product

**Quote/location:** README, Install: “Build from source with Rust 1.85 or
newer:”. `Cargo.toml` also declares `rust-version = "1.85"`.

**Evidence:** In the clean review clone, Rust 1.85.0 was installed and the
documented locked build was exercised:

```text
$ cargo +1.85.0 build --locked
error: rustc 1.85.0 is not supported by the following packages:
  icu_collections@2.3.0 requires rustc 1.88
  icu_locale_core@2.3.0 requires rustc 1.88
  icu_normalizer@2.3.0 requires rustc 1.88
  icu_properties@2.3.0 requires rustc 1.88
  icu_provider@2.3.1 requires rustc 1.88
  idna_adapter@1.2.2 requires rustc 1.86
```

The command exited 101. The claim is absent from `.factory/claims.json`, so the
declared claim suite never tests the stated minimum toolchain.

**Why this misleads a first-time visitor:** the first documented installation
path fails when followed with the explicitly supported version.

**Concrete fix:** either pin the locked dependency graph to versions compatible
with Rust 1.85, or raise both the README and `package.rust-version` to the real
minimum. Add a `minimum-rust-build` claim whose CI job installs that exact
toolchain and runs `cargo build --locked` and `cargo test --locked`.

### BLOCKING F-3-2 / B2 — the CLI demo still has no recording of the real command

**Quote/location:** the one-click demo presents a hand-written `<pre>` headed
“Sample demo output”; the lower landing panel is labelled “Example forge-sync
demo terminal output”. No asciinema, cast, video, generated SVG transcript, or
other recording asset exists in `site/`.

The displayed transcript is not the real transcript. The live panel separates
`issue records: 1 · pull-request records: 1` and `JSON archive: committed`.
The real command prints:

```text
source-to-target mappings: 1 · audit events: 3 · JSON archive: committed
Demo output: /tmp/forge-sync-demo-…
Remove that directory when you are done; forge-sync did not read or write your configuration.
```

**Why this is blocking:** the attached CLI demo contract requires a
self-hosted recording of the real binary doing the main job. Review 1 B2 asked
for “a self-hosted recording of that exact command”; the repair supplied static
HTML summaries instead. A visitor cannot distinguish captured product behavior
from illustrative marketing output.

**Concrete fix:** generate and ship a self-hosted recording from the real
`forge-sync demo` binary and bundled sample. Show it after the first click,
caption it, preserve a reduced-motion still/transcript, and add a test that the
recording/transcript was produced from the current command output. Keep the
working isolated CLI command and browser controls.

### BLOCKING F-3-3 / U20 / R06 — the retained JSON-output claim is still unlisted

**Quote/location:** README, Configure a real mirror: “Add `--json` to `status`
or `sync` when a script needs JSON output.”

**Evidence:** none of the 16 claim entries names or tests JSON output for both
`status` and `sync`. Some unrelated tests happen to pass `--json`, but the
claims contract requires one listed, tagged test for the promised observable
result. Review 1 U20 rejected the broader JSON claim; polish R06 retained this
narrower claim without adding its claim entry.

**Why this misleads a first-time visitor:** scripts rely on valid, stable JSON,
not merely acceptance of a flag. The current claim suite can pass if either
command emits non-JSON or the wrong shape.

**Concrete fix:** add a `status-sync-json-output` claim and one tagged test that
runs both commands against a temporary configured sample, parses stdout as one
JSON document, and asserts their documented fields. Otherwise remove the
sentence.

### MAJOR F-3-4 — production record links and run history are an unlisted claim with inconsistent names

**Quote/location:** README, What it records: “source-to-target ID links and
audit events in SQLite”. The landing page instead uses “record links”, “run
history”, and “run events”; `forge-sync demo` prints “source-to-target
mappings” and “audit events”.

**Why this loses or misleads a first-time visitor:** four names describe two
concepts, and “ID links” / “audit events” expose implementation language. The
sample claim proves one sample mapping and three sample events. The configured
state claim asserts mappings incidentally but does not assert production audit
events or list this README location.

**Concrete fix:** use “links between GitHub and target records” and “dated run
history” everywhere, including CLI output. Add a listed configured-run claim
that inspects both persisted links and run-history rows, or remove the
production bullet.

### MAJOR F-3-5 — the brief’s organization-scale success measure is unverified

**Quote/location:** `.factory/brief.json`: “Mirror a 50-repository organization
with 5,000 issues in under 30 minutes initially and under two minutes
incrementally.”

**Evidence:** the largest declared configured-run fixture has two repositories
and two issues. There is no benchmark or acceptance test for 50 repositories,
5,000 issues, the initial threshold, or the incremental threshold.

**Why this matters:** “organization mirror” is the core job, and the source of
truth supplies a concrete scale target. Small fixtures establish correctness,
not that the product performs the researched job at the intended scale.

**Concrete fix:** add a deterministic local benchmark fixture with 50
repositories and 5,000 issues, test both initial and no-change incremental
passes against the two thresholds, and document representative hardware and
margin. Do not put the numbers on the landing page until that claim is listed
and reproducible.

### MINOR F-3-6 — the one-click demo route has the wrong heading hierarchy

**Quote/location:** `/?demo=1` has title “Demo — forge-sync”, but its first
visible heading is the h2 “See a completed sample mirror.” The page’s only h1
is the later landing headline “Mirror your GitHub organization to another
forge.”

**Why this matters:** the route identifies itself as Demo in metadata and moves
focus to the h2, while the document outline says the page is the landing page.
This fails the requirement that the one h1 be the route headline.

**Concrete fix:** make the visible sample heading the h1 in `?demo=1` mode and
demote or hide the later landing headline from that route’s outline. Add a
browser assertion for heading order, not only h1 count and focus.

### MINOR F-3-7 — the third required first-screen fact is below the desktop fold

**Quote/location:** at 1440 × 900, “Works offline after your first visit.”
starts at y=903.7 and ends at y=925.5. It is not visible before scrolling. The
other two facts end at y=895.7.

**Why this matters:** the standard first-screen structure requires three plain
facts. The phone shows all three, but the requested desktop cold view does not.

**Concrete fix:** reduce the desktop hero’s vertical offset or headline/image
height so all three facts end above 900px. Add a 1440 × 900 bounding-box test
for every first-screen fact.

### MINOR F-3-8 — “fixture” breaks the landing page’s sample terminology

**Quote/location:** closing eyebrow: “Start with a fixture”. Everywhere else
the visitor-facing landing page says “sample” or “sample data”.

**Why this matters:** “fixture” is test jargon and creates a second name for
the same concept.

**Concrete rewrite:** “Start with sample data”.

### MINOR F-3-9 — a landing heading is vague out of context

**Quote/location:** How it works h2: “Copy code and its working record.”

**Why this matters:** “working record” is not a recognized Git hosting object
and does not tell a screen-reader user whether it means issues, reviews, or an
activity log.

**Concrete rewrite:** “Copy code, issues, and pull-request history.”

## Copy audit

Counting uses whitespace-delimited words. Hyphenated terms count once. Code
blocks and generated configuration are excluded because they are not
sentences; every heading, label, action, fact, prose sentence, and live message
is included. No item exceeds 22 words, and no banned marketing word appears.
All buttons name a result. Flags point to findings above.

### Landing page

| Area | Exact copy and word count | Result |
| --- | --- | --- |
| Access/header | “Skip to content” (3); “Demo” (1); “How it works” (3); “Privacy” (1) | Pass |
| Demo banner/actions | “Demo — sample data, nothing is saved.” (7); “Reset demo” (2); “Leave demo and build configuration” (5) | Pass |
| Demo panel | “Sample mirror ready” (3); “See a completed sample mirror.” (5) | Pass |
| Demo panel | “Harbor Cooperative has one repository, branches, a tag, a pull-request record, record links, run events, and a JSON archive saved in Git.” (22) | Listed sample claim |
| Demo panel | “Open the full demo record” (5); “Completed sample mirror — no tokens used” (7) | Pass / listed sample claim |
| Hero | “GitHub organization mirror” (3); “Mirror your GitHub organization to another forge.” (7) | Pass |
| Hero | “For maintainers who need a copy on Forgejo, Codeberg, or GitLab with issues and pull-request history.” (16) | Listed capability claims |
| Hero actions | “Try it with sample data” (5); “See a completed sample mirror; nothing is saved.” (8); “Build your configuration” (3) | Pass |
| Hero facts | “Free under the MIT License.” (5); “No website analytics.” (3); “Works offline after your first visit.” (6) | Listed claims; F-3-7 placement |
| Art labels | “GitHub organization” (2); “JSON archive” (2); “Your forge” (2) | Pass |
| Process heading | “How the sample is arranged” (5); “Copy code and its working record.” (6) | F-3-9 |
| Process labels | “01 / source” (3); “GitHub organization” (2); “02 / record” (3); “Local archive” (2); “03 / target” (3); “Another forge” (2) | Pass |
| Process prose | “The sample starts with Harbor Cooperative’s harbor-tools repository.” (8) | Listed sample claim |
| Process prose | “JSON records, record links, and run history remain together.” (9) | Listed sample claim |
| Process prose | “The sample shows branches, tags, and a readable pull-request issue.” (10) | Listed sample claim |
| Contents heading | “What the sample contains” (4); “Keep issues and pull-request history with the code.” (9) | Pass |
| Contents prose | “The completed sample makes each copied record visible before you connect a real organization.” (14) | Listed sample claim |
| Ledger | “Git data” (2); “Branches and tags” (3); “Two branches and one version tag.” (6) | Listed sample claim |
| Ledger | “Issue record” (2); “Pull-request discussion” (2); “Author, review, file name, and line number.” (7) | Listed sample claim |
| Ledger | “Local record” (2); “Record links and run history” (5); “One link between source and target records, plus three run events.” (11) | Listed sample claim |
| Ledger | “Archive” (1); “JSON archive saved in Git” (5); “A Git commit records the sample archive.” (7) | Listed sample claim |
| Builder | “Build locally” (2); “Create a configuration without token values.” (6) | Pass / listed configuration claim |
| Builder | “This browser tool creates text only.” (6); “It does not ask for a token.” (7) | Listed configuration claim |
| Builder controls | “GitHub organization” (2); “Target forge” (2); “Target URL” (2); “Target owner or namespace” (4) | Pass |
| Builder actions | “Copy configuration” (2); “Download configuration” (2) | Pass; both actions worked live |
| Builder next step | “Then check access without making changes:” (6) | Listed doctor claim |
| CLI sample | “Run the sample” (3); “Inspect a mirror before using your data.” (8) | Pass |
| CLI notes | “Separate output.” (2); “The command creates a new temporary directory.” (7) | Listed sample claim |
| CLI notes | “Disposable data.” (2); “Remove the printed directory when you finish.” (7) | Pass |
| CLI notes | “Sample source files.” (3); “The source JSON ships in examples/sample-mirror.” (6) | Listed sample claim |
| Closing | “Start with a fixture” (4); “See the sample, then build your configuration.” (8); “Try it with sample data” (5) | F-3-8 |
| Footer | “Mirror GitHub organizations to another forge.” (6); “Privacy” (1); “Terms” (1); “GitHub source” (2) | Pass |
| Dynamic success | “Demo reset.” (2); “The sample data is new.” (5); “Configuration ready.” (2); “Add token environment-variable names when you run the CLI.” (9) | Pass |
| Dynamic feedback | “Configuration copied.” (2); “Clipboard access was blocked.” (4); “Select the configuration and copy it manually.” (7) | Pass |
| Dynamic error | “Complete the fields above to generate the configuration.” (8) | Pass |
| Validation | “Use a valid GitHub organization name.” (6); “Use a valid target owner or namespace.” (7) | Pass |
| Validation | “Enter a complete target URL, including https://.” (7); “Use HTTPS for a remote target.” (6) | Pass |
| Offline | “You’re offline.” (2); “The sample and configuration builder remain available after a first visit.” (11) | Listed offline claim |

### README

| Area | Exact copy and word count | Result |
| --- | --- | --- |
| Title/summary | “forge-sync” (1); “Mirror your GitHub organization to another forge.” (7) | Pass |
| Audience | “forge-sync is for maintainers who need an independent copy on Forgejo, Codeberg, or GitLab.” (14) | Listed target claims |
| Summary | “It writes a local record and a JSON archive alongside the target copy.” (13) | Listed configured-run claim |
| Heading/introduction | “Try the completed sample” (4); “Run this before configuring a real organization:” (7) | Pass |
| Sample | “The command creates a new temporary directory and prints its path.” (11); “It does not read your configuration or token values.” (9) | Listed demo claim |
| Sample | “The output contains the fictional Harbor Cooperative harbor-tools repository, two branches, and one tag.” (14) | Listed demo claim |
| Sample | “It also contains a pull-request record, SQLite mapping, audit log, and committed JSON archive.” (14) | Listed demo claim; terminology differs from landing |
| Sample | “Delete the printed directory when you finish.” (7); “The source records are in examples/sample-mirror.” (6) | Pass / listed demo claim |
| Sample | “See .factory/demo.md for browser and CLI sandbox details.” (8) | Pass |
| Install | “Install” (1); “Build from source with Rust 1.85 or newer:” (8); “Or run the container:” (4) | F-3-1 |
| Configuration | “Configure a real mirror” (4); “Create forge-sync.toml.” (2) | Pass |
| Configuration | “Name the environment variables that hold tokens.” (7); “Do not put token values in this file.” (8) | Pass / listed configuration claim |
| Run | “Check access before making changes, then run a pass:” (9) | Listed doctor claim |
| Run | “Run forge-sync daemon --config forge-sync.toml for continuous passes.” (8) | Listed daemon claim |
| Run | “Add --json to status or sync when a script needs JSON output.” (12) | F-3-3 |
| Records heading | “What it records” (4) | Pass |
| Record | “repository discovery, branches, tags, labels, milestones, and issues;” (8) | Listed configured-run claim |
| Record | “pull-request descriptions, reviews, inline comments, and discussion comments in a labeled target issue;” (11) | Listed configured-run claim |
| Record | “the author, time, and original GitHub link in copied bodies;” (10) | Listed configured-run claim |
| Record | “source-to-target ID links and audit events in SQLite;” (8) | F-3-4 |
| Record | “JSON snapshots, optionally committed to a local Git archive.” (9) | Listed configured-run claim |
| Dry run | “forge-sync sync --dry-run reports planned changes.” (5) | Listed dry-run claim |
| Dry run | “It does not change either forge, Git data, local state, the audit log, or the JSON archive.” (16) | Listed dry-run claim |
| Development | “Development” (1); “The static documentation build is written to dist/site.” (8); “Run it locally with npm run dev.” (7) | Verified by build/dev setup; not a product behavior claim |
| Privacy | “Privacy and license” (3); “The browser sample stores one demo:forge-sync: marker.” (7) | Listed browser-isolation claim |
| Privacy | “The configuration builder has no token field.” (7); “Read the full privacy policy and terms.” (7) | Listed configuration claim / pass |
| License | “forge-sync is released under the MIT License.” (7) | Listed license claim |

Terminology conflicts are recorded in F-3-4 and F-3-8. No action label uses
`Submit`, `Go`, or `Continue`; every button states the result.

## Demo and sandbox behavior

- One click from the first screen opened `/?demo=1` and immediately showed the
  Harbor Cooperative repository, two branches, one tag, an issue record, a
  pull-request record, and an archive result.
- The persistent banner read “Demo — sample data, nothing is saved.” and showed
  Reset plus the explicit leave action.
- A preloaded `real:sentinel=keep` survived entry, Reset, exit, Back, and
  Forward. Demo entry added only `demo:forge-sync:session`; Reset replaced only
  that value; exit removed it.
- The complete browser flow made no cross-origin request. The offline claim test
  primed the service worker, blocked the network, reloaded, and edited the
  configuration successfully.
- From empty `/tmp/forge-sync-review3-demo.25aUih`, the real CLI created only a
  new `/tmp/forge-sync-demo-*` directory. Canary source and target token values
  were absent from its output tree. The result contained SQLite state, target
  refs and issue markdown, JSON archive files, and Git commit `a58fe2c`.
- These checks pass isolation. F-3-2 remains because the required browser
  recording is not implemented.

## Declared claim results

Clean clone: `/tmp/forge-sync-review3-clean.rdSq7R/repo`. `npm ci` completed
with zero reported vulnerabilities. `npm run test:claims` ran every command in
`.factory/claims.json` and reported **16 passed**.

| Claim ID | Result |
| --- | --- |
| `demo-completed-mirror` | PASS |
| `demo-browser-isolation` | PASS |
| `configuration-has-no-token-field` | PASS |
| `website-no-tracking` | PASS |
| `supported-targets` | PASS |
| `git-refs-mirror` | PASS |
| `dry-run-read-only` | PASS |
| `doctor-read-only` | PASS |
| `mit-license` | PASS |
| `offline-demo-after-first-visit` | PASS |
| `configured-run-state-and-archive` | PASS |
| `continuous-daemon-passes` | PASS |
| `configured-records-metadata` | PASS |
| `configured-renders-pull-request-history` | PASS |
| `configured-copied-body-attribution` | PASS |
| `configured-optional-git-archive` | PASS |

No declared claim failed. F-3-1, F-3-3, and F-3-4 are unlisted claims, so the
passing 16-item set is incomplete.

## Earlier-finding verification

Every earlier review, polish report, verification report, and handoff was read.
Ranges below are inclusive; every earlier identifier is accounted for.

| Earlier ID(s) | Current result and fresh evidence |
| --- | --- |
| B1 | Fixed. Both cold viewports answer what, who, and first action. |
| B2 | **Reopened; BLOCKING.** Browser/CLI isolation works, but the exact self-hosted recording required by B2 is absent. See F-3-2. |
| B3 | The manifest and unique-tag runner now exist. All 16 listed tests pass, but new omissions are F-3-1, F-3-3, and F-3-4. |
| B4 | Fixed. The unknown route returns the designed product page with HTTP 404 and a way home. |
| M1 | Fixed. Paid checkout and paid promises remain absent; the live crawl found no checkout link. |
| M2 | Fixed. All tested routes have descriptions, canonicals, OG/Twitter data, favicon, apple icon, robots, and sitemap. |
| M3 | Fixed. Hash, document, Back, and Forward navigation restore the tested heading focus and announce route changes. |
| M4 | Fixed. Header/footer, Privacy, Terms, factory credit, and build `18ab5387` are consistent. |
| M5 | Prior flagged terms are removed. New copy issues are listed separately as F-3-4, F-3-8, and F-3-9. |
| U01, U04, U07, U08, U11, U12, U13, U17, U18, U19, U21, U22, U23, U24, U25, U27, U28, U29 | Fixed. The prior broad, quantitative, tracking, recovery, rate-limit, partial-failure, relay, paid, and privacy promises remain absent or narrowed. |
| U02, U05, U09, U10, U15, U26 | Fixed for the shipped sample by the passing CLI demo claim and direct output inspection. |
| U03 | Fixed. Only the tested MIT, no-analytics, and offline facts remain. |
| U06, U30, U31, U35 | Fixed by target-contract, Git-ref, configured metadata/history, attribution, and archive claims. |
| U14, U45 | Fixed by form inspection and the no-cross-origin browser flow. |
| U16, U39, U43 | Fixed by the passing doctor and dry-run claims. |
| U20 | **Reopened; BLOCKING.** The narrower status/sync JSON statement remains unlisted. See F-3-3. |
| U32, U33, U34, U36, U37, U38, U40, U41, U42, U44, U46, U47 | Fixed. Those universal claims remain absent; retained offline behavior passes. |
| L01–L10 | Fixed. The job, organization/repository terms, target names, first action, and direct labels remain. |
| L11–L22 | Prior exact text is removed; current heading/copy issues are separately recorded in F-3-4 and F-3-9. |
| L23–L30 | Fixed. Configuration actions are result-naming; sample counts are bounded; unsupported universal claims remain absent. |
| L31–L40 | Fixed. Paid, relay, and unsupported roadmap material remains absent. |
| L41–L47 | Fixed for the earlier exact text. Legal wording, footer, form feedback, and offline message remain present and tested. |
| R01–R05, R07–R11, R13–R27 | Fixed for the earlier exact findings. Sentences remain under 22 words and the named behaviors are removed or covered. |
| R06 | **Reopened with U20; BLOCKING.** Narrowing the JSON sentence did not add a claim entry. See F-3-3. |
| R12 | The earlier wording remains, but it conflicts with the landing terms and lacks explicit production run-history coverage. See F-3-4. |
| P0 | Fixed. The dry-run-to-real regression passed in the clean clone. |
| P1 | Fixed. Reduced-motion keyboard tests pass; live axe found zero violations. |
| P2-exit | Fixed. Configuration error exit-code tests pass. |
| P2-host | Fixed. Live CSP, HSTS, nosniff, referrer, frame, and permissions headers are present. |
| F-2-1a–F-2-1f | Fixed. All six configured production claims are listed and pass. |
| F-2-2 | Fixed. CTA, leave, Back, and Forward focus tests pass live and locally. |
| F-2-3 | Fixed. The 26-word sample sentence is now two 14-word sentences. |
| F-2-4 | Prior exact landing/internal terms were replaced. F-3-4 records the remaining cross-surface inconsistency. |
| F-2-5 | Fixed. The action now says “Leave demo and build configuration”. |

## Structure, links, accessibility, and identity

- `/`, `/?demo=1`, `/demo/`, `/privacy/`, `/terms/`, and the 404 response have
  route-specific titles, English language, one `main`, one h1, descriptions,
  canonicals, OG/Twitter metadata, SVG favicon, and apple-touch icon. F-3-6 is
  the semantic mismatch on the query demo route.
- `/review-3-missing-route` returned HTTP 404 with the designed ceramic-style
  page and “Return to forge-sync”. `robots.txt` and `sitemap.xml` returned 200.
- Every discovered internal HTTP link returned 200. Both GitHub source URLs
  returned 200. The two `mailto:` links were treated as explicit non-HTTP
  actions.
- `/opt/fleet/lib/verify-url.sh` passed the live home page with no console
  errors, one h1, one main, `lang=en`, and no missing alt text.
- `npm run test:a11y -- https://forge-sync.sociobot.in` reported zero axe
  violations, including zero serious or critical findings.
- The glacial ceramic art, mineral palette, Fraunces/IBM Plex pairing, cobalt
  path motif, and asymmetric layout match `.factory/design.md` and are
  recognizably product-specific rather than a generic SaaS template.
- The live asset names match the reviewed build, and the footer reports build
  `18ab5387`. The built initial JavaScript is 5.92 kB raw / 2.57 kB gzip.

## Other verification

From the clean clone:

- `npm test`: PASS — 13 Node/browser tests and all Rust tests passed.
- `npm run build`: PASS — `dist/site` produced.
- Live accessibility scan: PASS — zero axe violations.
- Live first-screen, demo reset/exit, Back/Forward, copy, and download checks:
  PASS. Download produced `forge-sync.toml`; clipboard output matched the
  generated configuration.
- Rust 1.85 locked build: **FAIL**, as documented in F-3-1.

## Missed leverage

No AI feature is warranted. This is a deterministic transfer and archive tool;
model output would make fidelity harder to verify. Sync, a JSON archive,
configuration generation, and sample data already cover the obvious product
loop. The missed leverage is measurement at the brief’s intended organization
scale, recorded as F-3-5, not an AI add-on.

## What would make this perfect

Make the documented minimum Rust build pass and claim-test it. Replace the
illustrative terminal blocks with a self-hosted recording generated from the
real demo command. List and test the retained JSON, record-link, and run-history
claims. Add the 50-repository/5,000-issue benchmark. Then correct the demo h1,
fit all three facts in the desktop first screen, and replace “fixture” and
“working record” with the concrete sample language proposed above. Re-run the
entire review from fresh contexts and a clean clone; perfection requires zero
remaining findings and no untested claim.
