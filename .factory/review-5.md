# Adversarial first-read review 5 — forge-sync

**Verdict: PASS**

Reviewed 28 August 2026 against the deployed build `44c03a40` at
<https://forge-sync.sociobot.in>. Fresh mobile, desktop, demo, claim, CLI,
offline, privacy, history, structure, link, accessibility, and copy checks
leave **zero findings**.

## Thirty-second cold read

Fresh empty contexts opened `/` at 390 × 844 and 1440 × 900 without scrolling.

| Question | Phone | Desktop |
| --- | --- | --- |
| What does it do? | Mirror a GitHub organization to another forge. | The same. |
| For whom? | Maintainers who need a copy on Forgejo, Codeberg, or GitLab, with issues and pull-request history. | The same. |
| What should I click first? | `Try it with sample data`. | The same. |

The exact passing copy is “Mirror your GitHub organization to another forge.”,
“For maintainers who need a copy on Forgejo, Codeberg, or GitLab with issues
and pull-request history.”, and “Try it with sample data” / “See a completed
sample mirror; nothing is saved.” The primary action ended at y=543 on phone
and y=724 on desktop; all three facts ended at y=750 and y=844 respectively.
No horizontal overflow or console error occurred.

## Copy audit

Words are whitespace-delimited; hyphenated words count once. Commands, TOML,
file paths, the wordmark, and captured terminal commands are code rather than
sentences and are excluded. The semicolon-separated units below list every
visible sentence, heading, label, fact, action, and dynamic message.

### Landing and demo

| Area | Exact copy unit(s) | Words | Result |
| --- | --- | ---: | --- |
| Navigation | “Skip to content”; “Demo”; “How it works”; “Privacy” | 3; 1; 3; 1 | Pass |
| Demo controls | “Demo — sample data, nothing is saved.”; “Reset demo”; “Leave demo and build configuration” | 7; 2; 5 | Listed isolation claim; actions name results |
| Demo panel | “Sample mirror ready”; “See a completed sample mirror.”; “Harbor Cooperative’s harbor-tools repository has two branches, one tag, and one pull-request record.”; “It includes links between GitHub and target records, dated run history, and a JSON archive saved in Git.” | 3; 5; 13; 18 | Listed sample/record-history claims |
| Recording | “Captured from the current CLI and bundled sample data.”; “Read the captured transcript”; “Open the full demo record” | 9; 4; 5 | Listed recording claim; action names result |
| Hero | “GitHub organization mirror”; “Mirror your GitHub organization to another forge.”; “For maintainers who need a copy on Forgejo, Codeberg, or GitLab with issues and pull-request history.” | 3; 7; 16 | Pass; capability claims listed |
| Hero controls | “Try it with sample data”; “See a completed sample mirror; nothing is saved.”; “Build your configuration” | 5; 8; 3 | Result-naming actions |
| Hero facts/art | “Free under the MIT License.”; “No website analytics.”; “Works offline after your first visit.”; “GitHub organization”; “JSON archive”; “Your forge” | 5; 3; 6; 2; 2; 2 | Claims listed; pass |
| Flow | “How the sample is arranged”; “Copy code, issues, and pull-request history.”; “01 / source”; “GitHub organization”; “The sample starts with Harbor Cooperative’s harbor-tools repository.”; “02 / record”; “Local archive”; “JSON records, links between GitHub and target records, and dated run history stay together.”; “03 / target”; “Another forge”; “The sample shows branches, tags, and a readable pull-request issue.” | 5; 6; 3; 2; 8; 3; 2; 14; 3; 2; 10 | Listed sample/record-history claims |
| Contents | “What the sample contains”; “Keep issues and pull-request history with the code.”; “The completed sample makes each copied record visible before you connect a real organization.” | 4; 9; 14 | Listed sample claim |
| Ledger | “Git data”; “Branches and tags”; “Two branches and one version tag.”; “Issue record”; “Pull-request discussion”; “Author, review, file name, and line number.”; “Local record”; “Links between GitHub and target records”; “One link and three dated run-history entries.”; “Archive”; “JSON archive saved in Git”; “A Git commit records the sample archive.” | 2; 3; 6; 2; 2; 7; 2; 6; 7; 1; 5; 7 | Listed claims |
| Builder | “Build locally”; “Create a configuration without token values.”; “This browser tool creates text only.”; “It does not ask for a token.”; “GitHub organization”; “Target forge”; “Forgejo”; “Codeberg”; “GitLab”; “Target URL”; “Target owner or namespace”; “Copy configuration”; “Download configuration”; “Then check access without making changes:” | 2; 6; 6; 7; 2; 2; 1; 1; 1; 2; 4; 2; 2; 6 | Listed configuration/doctor claims; actions name results |
| CLI sample | “Run the sample”; “Inspect a mirror before using your data.”; “Separate output.”; “The command creates a new temporary directory.”; “Disposable data.”; “Remove the printed directory when you finish.”; “Sample source files.”; “The source JSON ships in examples/sample-mirror.” | 3; 8; 2; 7; 2; 7; 3; 6 | Listed sample claim |
| Closing/footer | “Start with sample data”; “See the sample, then build your configuration.”; “Try it with sample data”; “Mirror GitHub organizations to another forge.”; “Privacy”; “Terms”; “GitHub source”; “Built by Param Factory · v0.1.0 · build 44c03a40” | 4; 8; 5; 6; 1; 1; 2; 9 | Pass |
| Dynamic feedback | “Demo reset.”; “The sample data is new.”; “Configuration ready.”; “Set the named token environment variables before you run the CLI.”; “Configuration copied.”; “Clipboard access was blocked.”; “Select the configuration and copy it manually.” | 2; 5; 2; 11; 2; 4; 7 | Pass |
| Validation/offline | “Complete the fields above to generate the configuration.”; “Use a valid GitHub organization name.”; “Use a valid target owner or namespace.”; “Enter a complete target URL, including https://.”; “Use HTTPS for a remote target.”; “You’re offline.”; “The sample and configuration builder remain available after a first visit.” | 8; 6; 7; 7; 6; 2; 11 | Listed offline claim; pass |
| Route announcements | “Moved to Mirror your GitHub organization to another forge.”; “Moved to See a completed sample mirror.”; “Moved to Inspect a completed sample mirror.”; “Moved to Privacy”; “Moved to Terms”; “Moved to That page does not exist.”; “Moved to Copy code, issues, and pull-request history.”; “Moved to Create a configuration without token values.” | 9; 7; 7; 3; 3; 7; 8; 8 | Pass |

### README

| Area | Exact copy unit(s) | Words | Result |
| --- | --- | ---: | --- |
| Title/summary | “forge-sync”; “Mirror your GitHub organization to another forge.”; “forge-sync is for maintainers who need an independent copy on Forgejo, Codeberg, or GitLab.”; “It writes a local record and a JSON archive alongside the target copy.” | 1; 7; 14; 13 | Listed targets/state claim |
| Sample | “Try the completed sample”; “Run this before configuring a real organization:”; “Open the isolated browser sample in one click.”; “The command creates a new temporary directory and prints its path.”; “It does not read your configuration or token values.”; “The output contains the fictional Harbor Cooperative harbor-tools repository, two branches, and one tag.”; “It also contains a pull-request record and a committed JSON archive.”; “SQLite stores links between GitHub and target records plus dated run history.”; “Delete the printed directory when you finish.”; “The source records are in examples/sample-mirror.”; “See .factory/demo.md for browser and CLI sandbox details.” | 4; 7; 8; 11; 9; 14; 11; 12; 7; 6; 8 | Listed sample/isolation/record claims |
| Install/configuration | “Install”; “Build from source with Rust 1.88 or newer:”; “Configure a real mirror”; “Create forge-sync.toml.”; “Name the environment variables that hold tokens.”; “Do not put token values in this file.” | 1; 8; 4; 2; 7; 8 | Listed Rust/configuration claims |
| Run/records | “Check access before making changes, then run a pass:”; “Run forge-sync daemon --config forge-sync.toml for continuous passes.”; “Add --json to status or sync when a script needs JSON output.”; “What it records”; “repository discovery, branches, tags, labels, milestones, and issues;”; “pull-request descriptions, reviews, inline comments, and discussion comments in a labeled target issue;”; “the author, time, and original GitHub link in copied bodies;”; “links between GitHub and target records plus dated run history in SQLite; and”; “JSON snapshots, optionally committed to a local Git archive.” | 9; 8; 12; 4; 8; 13; 10; 13; 9 | Listed production claims |
| Dry run/scale | “forge-sync sync --dry-run reports planned changes.”; “It does not change either forge, Git data, local state, dated run history, or the JSON archive.”; “Scale acceptance”; “The local acceptance benchmark mirrors 50 repositories with 5,000 issues.”; “It checks an initial pass against 30 minutes and a no-change pass against two minutes.”; “The test reserves a 20% margin, so its limits are 24 minutes and 96 seconds.”; “It uses local forge endpoints to remove internet delay and prints the runner’s operating system, architecture, CPU parallelism, and measured times.” | 6; 17; 2; 10; 15; 15; 21 | Listed dry-run/scale claims |
| Development/privacy | “Development”; “The static documentation build is written to dist/site.”; “Run it locally with npm run dev.”; “Privacy and license”; “The browser sample stores one demo:forge-sync: marker.”; “The configuration builder has no token field.”; “Read the full privacy policy and terms.”; “forge-sync is released under the MIT License.” | 1; 8; 7; 3; 7; 7; 7; 7 | Listed claims; pass |

No unit exceeds 22 words. No supplied banned word appears. Terminology remains
consistent: **organization**, **repository**, **pull request** /
**pull-request record**, **configuration**, **sample/demo**, **link between
GitHub and target records**, and **dated run history**. Every claim-like
sentence maps to a declared claim; no unlisted claim is found.

## Demo, sandbox, and claims

- The first-screen action reaches `/?demo=1` in one click. It immediately shows
  a completed Harbor Cooperative mirror, terminal recording, banner, Reset,
  and named exit action.
- With `real:sentinel=keep`, demo entry added only `demo:forge-sync:session`.
  Reset replaced only that value. Banner exit, Privacy, Back, and Forward
  removed demo data on non-demo pages; the sentinel remained unchanged.
- Intercepted landing, demo, reset, builder, and legal-route traffic was
  same-origin only. After service-worker activation, offline reload retained
  the demo and the configuration builder worked with the offline notice shown.
- From a new temporary working directory with invalid `forge-sync.toml` and
  canary tokens, `forge-sync demo` ignored both, created a new OS-temp output,
  and produced branch/tag files, SQLite state, a rendered pull-request record,
  and committed JSON archive. No canary appeared in its output tree.
- In clean clone `/tmp/forge-sync-review5-clean.R3xeGp/repo` at
  `44c03a40e319909773644e9b65a79dbd0f624ab3`, `npm ci` then
  `npm run test:claims` passed all 21 manifest commands: `demo-completed-mirror`,
  `demo-browser-isolation`, `configuration-has-no-token-field`,
  `website-no-tracking`, `supported-targets`, `git-refs-mirror`,
  `dry-run-read-only`, `doctor-read-only`, `mit-license`,
  `offline-demo-after-first-visit`, `configured-run-state-and-archive`,
  `continuous-daemon-passes`, `configured-records-metadata`,
  `configured-renders-pull-request-history`, `configured-copied-body-attribution`,
  `configured-optional-git-archive`, `minimum-rust-build`,
  `demo-recording-current-output`, `status-sync-json-output`,
  `configured-record-links-run-history`, and `organization-scale-performance`.

## Earlier-history regression check

Every earlier review, polish record, and handoff was read and rechecked in
current source and live behavior.

| Earlier IDs | Confirmation |
| --- | --- |
| B1–B4; M1–M5 | Clear first screen, isolated demo, manifest, designed HTTP 404, complete metadata, focus/history, no dead paid checkout, and shared skeleton are present. |
| U01–U47 | Unsupported broad/paid/recovery claims remain removed; retained promises are in the 21 passing claims. |
| L01–L47; R01–R27 | The audit above confirms plain wording, consistent terms, result-naming actions, and the 22-word ceiling. |
| F-2-1a–F-2-1f; F-2-2–F-2-5 | Configured-run claims, route focus, copy, terms, and named exit are present and pass. |
| F-3-1–F-3-9 | Rust 1.88, generated recording, JSON output, record-history naming, scale, demo h1, first-screen facts, sample terminology, and context heading pass. |
| F-4-1–F-4-3 | Every ordinary demo exit clears prefixed storage; untested container instructions are absent; corrected configuration feedback is live. |
| P0; P1; P2-exit; P2-host | Read-only dry run, reduced-motion/keyboard focus, configuration exits, and host security/cache headers remain covered. |

## Structure, accessibility, and identity

`/`, `/demo/`, `/privacy/`, `/terms/`, and a missing route have appropriate
route titles, one h1, descriptions, canonicals, OG/Twitter metadata, favicon,
and apple-touch icon. The missing route is a styled HTTP 404 with a return
path. The crawl found 200 for every discovered navigable link. Shared header
and footer include Privacy, Terms, Param Factory, version, and build ID.

`npm run test:a11y -- https://forge-sync.sociobot.in` reported zero axe
violations. `verify-url.sh` reported title, `lang=en`, one h1, main, complete
alt text, labelled buttons, and zero console errors. The glacial ceramic art,
mineral palette, cobalt transfer line, and serif/sans pairing follow
`.factory/design.md` and are visibly product-specific, not a generic SaaS
template.

## Missed leverage

No AI feature is expected: deterministic source-to-target transfer is the core
job, and AI would not make it safer or more complete. The implied configuration
builder, sample run, continuous sync, archive, and JSON export are present. No
provider key or decorative AI feature is found.

## What would make this perfect

Keep the current discipline: add an isolated observable claim before publishing
any new visitor-facing capability, privacy promise, or quantitative statement.
No current product change is required.

No product code was modified during this review.
