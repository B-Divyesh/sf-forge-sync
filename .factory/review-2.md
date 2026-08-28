# Adversarial first-read review 2 — forge-sync

**Verdict: FAIL**

Reviewed 28 August 2026 against live https://forge-sync.sociobot.in and commit
32107b0bda8c99a79cc459f8c0559b1db5cc23c2. This was a full, fresh review.
The product is clear and its demo works. It cannot pass because the README
retains production claims without matching entries and tests in claims.json.

## Thirty-second cold read

Fresh Chromium contexts with empty browser storage opened / at 390 × 844 and
1440 × 900. No scrolling occurred before recording this result.

| Question | Phone | Desktop |
| --- | --- | --- |
| What does it do? | Mirrors a GitHub organization to another forge. | Mirrors a GitHub organization to another forge. |
| For whom? | Maintainers needing a copy on Forgejo, Codeberg, or GitLab with issues and pull-request history. | The same. |
| What should I click first? | Try it with sample data. | The same. |

The passing copy was “Mirror your GitHub organization to another forge.”, “For
maintainers who need a usable copy on Forgejo, Codeberg, or GitLab, including
issues and pull-request history.”, and “Try it with sample data” / “See a
completed sample mirror; nothing is saved.” The primary action appeared in the
first 390px viewport with no horizontal overflow.

## Findings

### Major F-2-1 — README production promises are not declared or tested claims

The demo-completed-mirror claim proves a one-repository *sample*. It does not
cover these README statements about real use. No claim entry contains the
promise, and its named test does not observe it. Each table row is an
unlisted-claim finding.

| ID | Exact quote and location | Why a visitor can be misled | Concrete fix |
| --- | --- | --- | --- |
| F-2-1a | README introduction: “It writes local state and a JSON archive alongside the target copy.” | The sample test writes sample output; it does not run configured real sync and inspect selected state/archive paths. | Add real-run-state-and-archive using a configured local fixture, or make this sample-scoped. |
| F-2-1b | README configuration: “Run forge-sync daemon --config forge-sync.toml for continuous passes.” | No declared test starts the daemon, advances a controlled interval, and observes a later pass. | Add continuous-passes, or remove “for continuous passes.” |
| F-2-1c | README What it records: “repository discovery, branches, tags, labels, milestones, and issues;” | The refs claim checks branches/tags only. The sample claim does not assert discovery, labels, milestones, or a normal issue. | Add records-repository-metadata with a multi-repository mock and assertions for all six records. |
| F-2-1d | README What it records: “pull-request descriptions, reviews, inline comments, and discussion comments in a labeled target issue;” | The sample assertion checks selected text, not every named source field and target label in a real sync. | Add renders-pull-request-history asserting every field and label from source fixture to target record. |
| F-2-1e | README What it records: “the author, time, and original GitHub link in copied bodies;” | No declared test asserts author or time. A source URL alone does not prove this three-part promise. | Add copied-body-attribution asserting author, timestamp, and source URL, or reduce the sentence. |
| F-2-1f | README What it records: “JSON snapshots, optionally committed to a local Git archive.” | The sample test checks one commit, not enabled and disabled behavior in real configured use. | Add optional-git-archive for both settings, or reduce the sentence to the sample. |

A maintainer choosing a migration tool will read these as production
capabilities, not demo decoration. Adjacent unit tests or a sample test do not
satisfy the claim contract.

### Minor F-2-2 — document-route navigation leaves focus on body

**Evidence/location:** On the live phone site, activating “Try it with sample
data” navigated to /?demo=1 and left document.activeElement as BODY. The same
happened for “Start for real”. site/main.js calls focusDestination for hash
navigation and popstate, not ordinary document-route loads.

**Why this matters:** the required new-context focus and polite announcement do
not occur. A keyboard or screen-reader visitor reaches the demo visually but
does not land on its heading.

**Concrete fix:** on document-route initialization focus the h1 (or
demo-panel-title for ?demo=1) without scrolling and set route-status. Add a
browser test for the CTA, leave-demo, Back, and Forward.

### Minor F-2-3 — README has a 26-word sentence

**Quote/location:** README, Try the completed sample: “The output contains the
fictional Harbor Cooperative harbor-tools repository, branches, a tag, a
pull-request record, a SQLite mapping and audit log, and a committed JSON
archive.” — 26 words.

**Why this matters:** it combines sample, repository, Git data, rendered
record, state, audit, and archive in one scan-resistant sentence.

**Concrete fix:** “The output contains the fictional Harbor Cooperative
harbor-tools repository, two branches, and one tag. It also contains a
pull-request record, SQLite mapping, audit log, and committed JSON archive.”

### Minor F-2-4 — copy retains unexplained internal terms and a vague adjective

| Exact copy/location | Finding | Concrete rewrite |
| --- | --- | --- |
| Landing audience: “a usable copy” | “Usable” is a marketing adjective, not an observable result. | “a copy on Forgejo, Codeberg, or GitLab with issues and pull-request history” |
| Landing ledger: “Mappings and audit” / “One source-to-target ID and three events.” | These are internal implementation terms without an explanation. | “Record links and run history” / “One link between source and target records, plus three run events.” |
| Landing ledger: “Committed JSON” | This is a terse implementation phrase, not a plain result. | “JSON archive saved in Git” |
| Landing run note: “Real fixture.” | “Fixture” is test jargon; this heading has no useful meaning out of context. | “Sample source files.” |
| README introduction: “local state” | The landing uses “local record”; the README switches terms. | “a local record” |

### Minor F-2-5 — Start for real does not name its result

**Quote/location:** persistent demo banner and demo page: “Start for real”.

**Why this matters:** it does not start a real sync. It removes demo storage and
returns to the landing page/configuration builder.

**Concrete fix:** “Leave demo and build configuration”, with the storage-removal
explanation retained beside it.

## Copy audit

Counting rule: whitespace-delimited words. Commands, TOML, terminal output,
file paths, and the wordmark are code or identifiers and are excluded. Labels,
headings, actions, live messages, and visitor-facing sentences are included.
No banned plain-words term appeared. F-2-3 and F-2-4 are the only copy flags.

### Landing page

| Area | Copy | Words | Result |
| --- | --- | ---: | --- |
| Access | Skip to content | 3 | OK |
| Header | Demo; How it works; Privacy | 1; 3; 1 | OK |
| Demo banner | Demo — sample data, nothing is saved. | 7 | OK |
| Demo actions | Reset demo; Start for real | 2; 3 | F-2-5 on second action |
| Demo panel | Sample mirror ready; See a completed sample mirror. | 3; 5 | OK |
| Demo panel | Harbor Cooperative has one repository, branches, a tag, a pull-request record, mappings, audit events, and a committed JSON archive. | 18 | Sample claim |
| Demo action | Open the full demo record | 6 | OK |
| Hero | GitHub organization mirror; Mirror your GitHub organization to another forge. | 3; 7 | OK |
| Hero audience | For maintainers who need a usable copy on Forgejo, Codeberg, or GitLab, including issues and pull-request history. | 17 | F-2-4 |
| Hero actions | Try it with sample data; See a completed sample mirror; nothing is saved.; Build your configuration | 5; 8; 3 | OK |
| Hero facts | Free under the MIT License.; No website analytics.; Works offline after your first visit. | 5; 3; 6 | Declared claims |
| Art | GitHub organization; JSON archive; Your forge | 2; 2; 2 | OK |
| Process | How the sample is arranged; Copy code and its working record. | 5; 6 | OK |
| Source step | Source; GitHub organization; The sample starts with Harbor Cooperative’s harbor-tools repository. | 1; 2; 8 | Sample claim |
| Record step | Record; Local archive; JSON records, source-to-target IDs, and audit events remain together. | 1; 2; 9 | F-2-4 |
| Target step | Target; Another forge; The sample shows branches, tags, and a readable pull-request issue. | 1; 2; 10 | Sample claim |
| Contents | What the sample contains; Keep issues and pull-request history with the code. | 4; 9 | OK |
| Contents | The completed sample makes each copied record visible before you connect a real organization. | 15 | Sample claim |
| Git ledger | Git data; Branches and tags; Two branches and one version tag. | 2; 3; 6 | Sample claim |
| Issue ledger | Issue record; Pull-request discussion; Author, review, file name, and line number. | 2; 2; 7 | Sample claim |
| Local ledger | Local record; Mappings and audit; One source-to-target ID and three events. | 2; 3; 7 | F-2-4 |
| Archive ledger | Archive; Committed JSON; A Git commit records the sample archive. | 1; 2; 6 | F-2-4 on heading |
| Builder | Build locally; Create a configuration without token values. | 2; 6 | Configuration claim |
| Builder | This browser tool creates text only.; It does not ask for a token. | 6; 7 | Configuration claim |
| Builder labels | GitHub organization; Target forge; Target URL; Target owner or namespace | 2; 2; 2; 4 | OK |
| Builder actions | Copy configuration; Download configuration | 2; 2 | OK |
| Builder next | Then check access without making changes: | 6 | Doctor claim |
| Run | Run the sample; Inspect a mirror before using your data. | 3; 8 | OK |
| Run note | Separate output.; The command creates a new temporary directory. | 2; 7 | Sample claim |
| Run note | Disposable data.; Remove the printed directory when you finish. | 2; 7 | OK |
| Run note | Real fixture.; The source JSON ships in examples/sample-mirror. | 2; 6 | F-2-4 on heading |
| Closing | Start with a fixture; See the sample, then build your configuration.; Try it with sample data | 4; 8; 5 | OK |
| Footer | Mirror GitHub organizations to another forge.; Privacy; Terms; GitHub source | 6; 1; 1; 2 | OK |
| Dynamic | Demo reset.; The sample data is new.; Configuration ready.; Add token environment-variable names when you run the CLI.; Configuration copied. | 2; 5; 2; 9; 2 | OK |
| Errors | Complete the fields above to generate the configuration.; Clipboard access was blocked.; Select the configuration and copy it manually. | 8; 4; 7 | OK |
| Errors | Use a valid GitHub organization name.; Use a valid target owner or namespace.; Enter a complete target URL, including https://.; Use HTTPS for a remote target. | 6; 7; 7; 6 | OK |
| Offline | You’re offline.; The sample and configuration builder remain available after a first visit. | 2; 10 | Offline claim |

### README

| Area | Copy | Words | Result |
| --- | --- | ---: | --- |
| Title | forge-sync | 1 | OK |
| Summary | Mirror your GitHub organization to another forge. | 7 | OK |
| Audience | forge-sync is for maintainers who need an independent copy on Forgejo, Codeberg, or GitLab. | 14 | Needs F-2-1 coverage |
| Summary | It writes local state and a JSON archive alongside the target copy. | 12 | F-2-1a; F-2-4 |
| Heading | Try the completed sample | 4 | OK |
| Sample | Run this before configuring a real organization: | 7 | OK |
| Sample | The command creates a new temporary directory and prints its path. | 11 | Sample claim |
| Sample | It does not read your configuration or token values. | 9 | Sample claim |
| Sample | The output contains the fictional Harbor Cooperative harbor-tools repository, branches, a tag, a pull-request record, a SQLite mapping and audit log, and a committed JSON archive. | 26 | F-2-3 |
| Sample | Delete the printed directory when you finish. | 7 | OK |
| Sample | The source records are in examples/sample-mirror. | 6 | Sample claim |
| Sample | See .factory/demo.md for browser and CLI sandbox details. | 8 | OK |
| Heading | Install | 1 | OK |
| Install | Build from source with Rust 1.85 or newer: | 8 | OK |
| Install | Or run the container: | 4 | OK |
| Heading | Configure a real mirror | 4 | OK |
| Configuration | Create forge-sync.toml. | 2 | OK |
| Configuration | Name the environment variables that hold tokens. | 6 | OK |
| Configuration | Do not put token values in this file. | 8 | Configuration claim |
| Run | Check access before making changes, then run a pass: | 9 | Doctor claim |
| Run | Run forge-sync daemon --config forge-sync.toml for continuous passes. | 6 | F-2-1b |
| Run | Add --json to status or sync when a script needs JSON output. | 12 | OK |
| Heading | What it records | 4 | OK |
| Record | repository discovery, branches, tags, labels, milestones, and issues; | 8 | F-2-1c |
| Record | pull-request descriptions, reviews, inline comments, and discussion comments in a labeled target issue; | 11 | F-2-1d |
| Record | the author, time, and original GitHub link in copied bodies; | 10 | F-2-1e |
| Record | source-to-target ID links and audit events in SQLite; | 8 | Sample-only coverage |
| Record | JSON snapshots, optionally committed to a local Git archive. | 9 | F-2-1f |
| Dry run | forge-sync sync --dry-run reports planned changes. | 5 | Dry-run claim |
| Dry run | It does not change either forge, Git data, local state, the audit log, or the JSON archive. | 16 | Dry-run claim |
| Heading | Development | 1 | OK |
| Development | The static documentation build is written to dist/site. | 8 | OK |
| Development | Run it locally with npm run dev. | 5 | OK |
| Heading | Privacy and license | 3 | OK |
| Privacy | The browser sample stores one demo:forge-sync: marker. | 7 | Browser-isolation claim |
| Privacy | The configuration builder has no token field. | 6 | Configuration claim |
| Privacy | Read the full privacy policy and terms. | 7 | OK |
| License | forge-sync is released under the MIT License. | 7 | MIT claim |

## Demo and sandbox verification

- The first-screen CTA reached /?demo=1 in one click. At 390 px its first screen
  showed the persistent banner, Reset demo, Start for real, a Harbor Cooperative
  sample, and terminal output before the ordinary hero.
- With a pre-existing real:sentinel=keep, demo added only
  demo:forge-sync:session. Reset replaced that value and preserved the sentinel.
  Start for real removed the demo key and preserved the sentinel. No
  cross-origin request occurred.
- From a newly created /tmp/forge-sync-review2-cli.* directory with canary
  token variables, cargo run -- demo --json returned one repository, one issue,
  one pull-request record, a committed archive, and a newly created
  /tmp/forge-sync-demo-* output directory.
- The offline claim test entered demo, waited for the service worker, reloaded
  offline, and changed configuration. The privacy claim test intercepted every
  route, demo, and builder interaction and allowed only same-origin requests.

## Claim test results

Fresh clone: /tmp/forge-sync-review2-clean.BBroWE created with git clone
--no-local at the reviewed commit. After npm ci, each claims.json command
passed:

| Claim ID | Result |
| --- | --- |
| demo-completed-mirror | PASS |
| demo-browser-isolation | PASS |
| configuration-has-no-token-field | PASS |
| website-no-tracking | PASS |
| supported-targets | PASS |
| git-refs-mirror | PASS |
| dry-run-read-only | PASS |
| doctor-read-only | PASS |
| mit-license | PASS |
| offline-demo-after-first-visit | PASS |

npm test, npm run build, and npm run test:a11y -- https://forge-sync.sociobot.in
also passed. Axe reported zero violations, including zero serious/critical.

## Earlier-review regression check

Every earlier review-1, polish-1, handoff, and verification finding was read and
rechecked in live site and code. B1 through B4 and M1 through M5 are fixed:
the phone-first hero is clear; the isolated browser/CLI demo exists; claim
tests exist; the product-styled 404 returns HTTP 404; unavailable checkout is
absent; metadata/crawl files are present; hash navigation restores heading
focus; and all routes share the required header/footer. Earlier dry-run,
reduced-motion focus, configuration-exit, and host-header failures are covered
by passing tests and observed live headers.

F-2-1 is new: it concerns broad README production promises that remain outside
the repaired sample claim inventory.

## Structure, links, and visual identity

- /, /?demo=1, /demo/, /privacy/, /terms/, and /404.html returned the expected
  title, exactly one h1, one main, language, description, canonical, Open
  Graph/Twitter data, favicon, and apple touch icon.
- A nonexistent live route returned HTTP 404 with a designed product page and
  return link. robots.txt, sitemap.xml, hero, icon, and social image returned
  200. Discovered internal links returned 200; the deliberately probed missing
  route returned 404; both GitHub links returned 200.
- Live responses carried CSP, referrer policy, nosniff, frame policy, and
  permissions policy. Hashed JavaScript was immutable cached. No console errors
  occurred in the exercised flows.
- The ceramic hero, cobalt path, serif/sans pairing, clay surfaces, and terminal
  create a distinctive visual system aligned with design.md, not a generic SaaS
  template.

## Missed leverage

No AI feature is expected: the brief is a deterministic organization-mirroring
CLI, and an AI step would not improve the core transfer or make it safer. The
product already provides the implied sync, archive, configuration generation,
and sample path. No provider key or decorative AI feature was found.

## What would make this perfect

Add isolated claims/tests for every retained README production promise, then
use their measured results or reduce the copy to the sample facts already
proven. Move focus and announce document-route changes, and replace the few
remaining internal labels and generic leave-demo button. Re-run this complete
review from a clean clone and fresh contexts.
