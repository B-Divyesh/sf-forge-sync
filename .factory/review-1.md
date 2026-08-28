# Adversarial first-read review 1 — forge-sync

**Verdict: FAIL**

Reviewed 28 August 2026 against `https://forge-sync.sociobot.in` and commit
`b3d507fde6d9e89a9002027087aff72b35bbb889`. There are four blocking findings,
five major findings, and copy/claim findings beyond the three-minor PASS limit.

## Thirty-second cold read

Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900 before any
scrolling.

| Question | Desktop, before scrolling | Phone, before scrolling |
| --- | --- | --- |
| What does it do? | I infer that it continually copies a GitHub organization's repositories and discussion history to another forge and a JSON archive. | The image, labels, and headline imply moving a GitHub organization elsewhere, but the explanatory sentence is cut off at the bottom of the viewport. |
| For whom? | **Cannot answer.** No person or situation is named. | **Cannot answer.** No person or situation is named. |
| What should I click first? | **Cannot answer.** “Build your config” and “Download v0.1.0” are equal adjacent choices; neither is a sample-data trial. | **Cannot answer.** No action is visible in the first viewport. |

Exact copy that failed the test: “Your GitHub org, somewhere else.
Continuously.” and “Every repository. Every new repo. Branches, tags, issues,
labels, milestones, PR conversations, and review context—kept usable on your
forge and in a portable archive.” The first is not the job in plain words; the
second lists objects but not the intended user. At 390 px the ceramic image is
shown before both lines, and no CTA is visible.

## Findings, ordered by severity

### BLOCKING B1 — the first screen does not identify the user or a first action

**Quote:** “Your GitHub org, somewhere else. Continuously.” / “Build your
config” / “Download v0.1.0”.

**Why this loses a first-time visitor:** “somewhere else” does not name the
actual result, “org” assumes GitHub shorthand, and neither viewport says this is
for maintainers planning forge independence or migration. Desktop offers two
competing actions. Phone offers none before scrolling.

**Concrete fix:** Use `Mirror your GitHub organization to another forge` as the
headline. Follow with `For maintainers who need a usable copy on Forgejo,
Codeberg, or GitLab, including issues and pull-request history.` Put `Try it with
sample data` first and add `See a completed sample mirror; nothing is saved.`
next to it. On mobile, place this copy and action before the art.

### BLOCKING B2 — there is no one-click, isolated demo

**Quote/evidence:** There is no “Try it with sample data” action. `/demo`
returns the generic Azure 404. `/?demo=1` returns the ordinary landing page with
no “Demo — sample data, nothing is saved” banner, no Reset, and no Start for
real. `forge-sync demo` in an empty `/tmp/forge-sync-demo-review.*` directory
returns `error: unrecognized subcommand 'demo'` and exit code 2; the directory
remains empty. No `examples/` or `.factory/demo.md` exists.

**Why this loses or misleads a visitor:** The product cannot be tried without
tokens, a source organization, and a target forge. There is no realistic output
that proves issues, pull-request discussion, mappings, and the archive survive.
Because demo mode does not exist, its reset, storage isolation, and protection
of real data cannot be verified.

**Concrete fix:** Ship an `examples/` fixture and `forge-sync demo`. It must use
only a newly created temporary directory, print that path, and show a completed
sample mirror with repositories, refs, issues, pull-request discussion, ID
mappings, audit events, and JSON archive output. Put a self-hosted recording of
that exact command behind the first-screen `Try it with sample data` action.
Document entry, reset, and isolation in `.factory/demo.md`.

### BLOCKING B3 — the claims contract is absent

**Quote/evidence:** `.factory/claims.json` does not exist. `rg '@claim:'`
finds no tagged tests. Therefore there were no listed commands to run from the
clean clone, and every claim in the claim inventory below is unlisted. This is
not a passing empty test set.

**Why this misleads a visitor:** The page makes broad scope, privacy,
idempotence, offline, price, and quantitative performance promises without a
reproducible sandbox test attached to any of them. Passing general unit tests
does not establish those promises.

**Concrete fix:** Add `.factory/claims.json`. Give every retained claim exactly
one `@claim:<id>` test, runnable only against the shipped demo fixture from a
clean state. Delete or narrow claims that cannot be observed. Include network
interception for privacy claims and exact count/time assertions for the sample
terminal output.

### BLOCKING B4 — required routes fall through to a generic host 404

**Quote/evidence:** `/definitely-missing-review-route` and `/demo` return HTTP
404 with title `Azure Static Web Apps - 404: Not found`, Microsoft Azure art,
no product header, and no way home. There is no authored 404 input in the Vite
build and no navigation fallback in `staticwebapp.config.json`.

**Why this loses a visitor:** A mistyped/deep URL exits the product's identity
and navigation. The missing `/demo` route also breaks the required try-out path.

**Concrete fix:** Add a product-styled `/404` page and configure the host to use
it while retaining a real 404 status. Include the standard header, `That page
does not exist` as its single h1, and `Return to forge-sync`.

### MAJOR M1 — the purchase action is dead

**Quote/evidence:** `Buy Migration Kit` links to
`https://api.sociobot.in/api/v1/products/forge-sync/checkout`; a HEAD request,
following redirects, returns 404.

**Why this misleads a visitor:** The page offers a $39 product that cannot be
purchased through its only action.

**Concrete fix:** Point the action at a verified Sociobot checkout URL and add a
deployment test that follows the same GET navigation and confirms a checkout
page, not merely the presence of the link.

### MAJOR M2 — metadata and crawl surfaces are incomplete

**Quote/evidence:** Home, Privacy, and Terms have no canonical link, Open Graph
fields, Twitter card, 1200 × 630 product image, or apple-touch icon. Both
`/robots.txt` and `/sitemap.xml` return 404. Privacy and Terms descriptions are
only “forge-sync privacy policy” and “forge-sync terms”. The home and route
titles, `lang=en`, theme color, SVG favicon, one h1, and one main do pass.

**Why this matters:** Shared links have no product-specific preview, search
engines lack canonical/crawl guidance, and installed mobile bookmarks lack the
required icon.

**Concrete fix:** Add route-specific canonical/OG/Twitter metadata, a derived
1200 × 630 ceramic asset, a 180 px apple-touch icon, `robots.txt`, and a sitemap
containing `/`, `/demo`, `/privacy/`, `/terms/`, and the designed 404 route.

### MAJOR M3 — route changes do not manage focus or history reliably

**Quote/evidence:** Activating `Build your config` changes the URL to
`/#configure`, but focus falls to `body`, not a route heading or announced
target. Back navigation again leaves focus on `body`; the observed scroll
position was not restored to the starting top position. There is no route-change
live region or focus code.

**Why this loses a keyboard or screen-reader visitor:** Location changes are
silent and context is lost. The back button does not return the visitor to the
same reading position.

**Concrete fix:** On navigation, focus the destination heading with
`tabindex=-1`, announce it through a polite live region, and save/restore scroll
positions on popstate. Add keyboard tests for direct URLs, forward, and back.

### MAJOR M4 — headers and footers do not follow one site skeleton

**Quote/evidence:** The home header has How it works, Configure, Pricing, and
Source, but no Privacy link. Legal-page headers switch to Terms/Home or
Privacy/Home. Legal-page footers omit Privacy/Terms navigation. Every footer
omits “Built by Param Factory” and a version/build ID.

**Why this loses a visitor:** Navigation moves and disappears between routes,
and there is no visible build identifier for support or verification.

**Concrete fix:** Use one header and footer component on every route: wordmark,
Demo, the main section, Privacy; then the one-line description, Privacy, Terms,
Built by Param Factory, and the deployed build ID.

### MAJOR M5 — plain-word copy contains unresolved jargon, vague headings, and inconsistent terms

The exact flags and rewrites are embedded in the copy audit below. The main
terminology conflict is `org`/`organization`, `repo`/`repository`/`project`,
`PR`/`pull request`/`pull-request`, and `config`/`configuration`. Use
`organization`, `repository`, `pull request`, and `configuration` everywhere.
No banned words from the supplied list were found.

## Claims: every current claim is unlisted

Each row is an individual unlisted-claim finding because the manifest is
absent. “Test needed” is the concrete fix; if that outcome is not intended,
remove the quoted claim.

| ID | Location and exact claim | Test needed |
| --- | --- | --- |
| U01 | Landing: “Every repository.” / “Every new repo.” | Demo adds a repository after start and asserts discovery and target creation for all fixtures. |
| U02 | Landing: “Branches, tags, issues, labels, milestones, PR conversations, and review context—kept usable on your forge and in a portable archive.” | End-to-end demo asserts each named object in the target and archive. |
| U03 | Landing: “Open source · One binary · No telemetry · Tokens stay local” | Assert license, one packaged executable, zero telemetry requests, and no token in files/logs/state/archive. |
| U04 | Landing: “all new repositories discovered” | Add-repository daemon fixture and assert discovery. |
| U05 | Landing: “1→1 stable ID mappings” | Repeat sync and assert one unchanged mapping per source ID. |
| U06 | Landing: “3 target forge families” | Run the same fixture against Forgejo, Codeberg, and GitLab contract mocks. |
| U07 | Landing: “0 tracking calls” | Intercept the complete landing/demo flow and assert only documented same-origin requests. |
| U08 | Landing: “Pagination and conditional requests catch private, archived, and newly created repositories without burning the rate limit.” | Assert pagination, private/archived/new discovery, conditional headers, and measured request count. |
| U09 | Landing: “SQLite keeps idempotent mappings and an audit trail.” | Inspect demo SQLite after two runs for stable mappings and expected append-only events. |
| U10 | Landing: “Forge-neutral snapshots are committed to your own Git archive.” | Inspect JSON schema and archive Git commit after demo sync. |
| U11 | Landing: “Missing projects are created, every ref is mirror-pushed, and discussions arrive with author and source links intact.” | Assert target creation, every fixture ref, discussion author, and canonical link. |
| U12 | Landing: “forge-sync keeps the social record readable even when the target cannot model a GitHub object exactly.” | Assert documented pull-request-to-issue rendering against a text snapshot. |
| U13 | Landing ledger: “True mirror push”; “State and labels preserved”; “Inline anchors retained as context”; “Canonical source link in body”. | Assert each of the four observable target outcomes. |
| U14 | Landing: “This local builder never asks for tokens and sends nothing anywhere.” | Fill every builder control while intercepting requests; assert no token control and no request. |
| U15 | Landing: “Tokens stay in environment variables.” | Run every CLI command and scan config, output, SQLite, archive, and process arguments for the fixture token. |
| U16 | Landing: “Then validate without writing” | Snapshot filesystem and mock request methods around `doctor`; assert no writes. |
| U17 | Landing sample: “48 repositories”, “48 ref sets”, “5,184 issues · 13,902 comments”, “Completed in 01:42 · 0 failed”. | Either label these numbers illustrative or run a fixture with those exact counts and assert completion within 102 seconds. |
| U18 | Landing: “Stable source IDs prevent duplicates after interruption.” | Interrupt mid-run, restart twice, and assert no duplicate target object. |
| U19 | Landing: “One broken repo is audited while the rest continue.” | Fail one fixture repository and assert remaining repositories complete plus one audit failure. |
| U20 | Landing: “Every operational command can emit JSON and meaningful exit codes.” | Invoke every operational command with `--json`; parse output and assert documented failure codes. |
| U21 | Landing: “The mirror is free.” / “$0 forever” / “Unlimited repositories and metadata” / “All target adapters”. | Add package/license and no-enforced-limit tests; otherwise narrow the pricing copy. |
| U22 | Landing: “The cutover kit saves a weekend.” | Remove this unmeasurable promise; no honest deterministic test is apparent. |
| U23 | Landing: “$39 one-time”; kit feature list; “Founding-buyer updates”. | Verify checkout amount/recurrence and licensed download contents; define and test update entitlement. |
| U24 | Landing: “Your license is checked at most once per day.” | Control time and assert at most one verification request per 86,400,000 ms. |
| U25 | Landing: “Refunds are handled there and revoke the license automatically.” | Billing webhook contract test must assert refund revocation. |
| U26 | Landing: “It preserves pull-request descriptions, reviews, discussion comments, and inline file/line anchors in a labeled target issue.” | Fixture asserts every named field in the target issue. |
| U27 | Landing: “This remains valid after merge, branch deletion, or cross-fork work.” | Run merged, deleted-branch, and cross-fork fixtures and assert retained readable output. |
| U28 | Landing: “The experimental relay flag records intent but sends nothing.” | Enable the flag, assert warning/audit output and zero target-to-GitHub writes. |
| U29 | Landing: “Metadata goes only to your chosen target, local SQLite state, and your local JSON archive. The documentation site never receives it.” | Intercept all network traffic and inspect all filesystem writes during demo sync. |
| U30 | README: “`forge-sync` continuously mirrors every repository in a GitHub organization to Forgejo, Codeberg, or GitLab.” | Continuous discovery fixture against all three adapters. |
| U31 | README: “It discovers new repositories, mirrors every Git ref, carries issues and pull-request discussions into the target with source attribution, and writes the same metadata to a portable JSON archive.” | One end-to-end demo assertion for every named outcome. |
| U32 | README: “The core is open source, local-first, and has no telemetry.” | Assert license, filesystem ownership of state, and no telemetry requests. |
| U33 | README: “Prebuilt binaries are attached to GitHub releases.” | Release smoke test downloads each promised platform binary. |
| U34 | README: “Commands never prompt.” | Run every command with closed stdin and a timeout. |
| U35 | README synchronization bullets, from repository discovery through Git snapshots. | Map each bullet to observable adapter/archive assertions. |
| U36 | README: “Updates are idempotent.” / retrying “does not duplicate objects.” | Run the same fixture twice and compare target counts and mappings. |
| U37 | README: “GitHub conditional requests and pagination reduce rate-limit use.” | Assert conditional request headers, pagination, and a stated request ceiling. |
| U38 | README: “A failed repository does not prevent the remainder…; the run exits `6` and records the failure.” | Multi-repository partial-failure test asserts continuation, audit, and exit 6. |
| U39 | README dry-run paragraph: no forge, Git, SQLite, cache/audit, or archive writes; next real run creates every missing object. | Existing regression is relevant, but tag it and enumerate every promised no-write surface. |
| U40 | README: relay “records a warning but never sends a target comment to GitHub.” | Flag-on network test asserts warning and zero reverse write. |
| U41 | README: tokens are read only from environment variables and never written to any named surface. | Canary-token scan of config, output, logs, SQLite, and archive. |
| U42 | README: “Private target repositories are the default.” | Omit the setting and assert a private create request for each adapter. |
| U43 | README: “`forge-sync doctor` checks identities and permissions before a first run.” | Mock insufficient identity/permissions and assert specific failures without writes. |
| U44 | README: “The repository contains integration tests…; tests do not require network credentials.” | Run tests with proxy-denied network and all token variables unset. |
| U45 | README: “The documentation site is static…; it does not receive tokens or mirror data.” | Static artifact inspection plus intercepted builder/demo flow and canary-data check. |
| U46 | Dynamic landing status: “Docs and the config builder still work” offline. | Prime the service worker, block all network, reload, and exercise every builder control. |
| U47 | Dynamic landing status: “Offline — using the last valid license check.” | Cache a valid verdict, block the API, advance time within the cache window, and assert licensed access without a request. |

The observed ordinary builder used only the site origin, and an offline reload
worked after one online visit. Those spot checks are not substitutes for U07,
U14, or a demo-backed claim test.

## Copy audit — landing page

Counting rule: whitespace-delimited words; hyphenated terms, slash terms,
versions, and formatted numbers each count as one. Sentences separated by a
period are listed separately. Headings, labels, facts, and controls are included
because they are independently presented copy. Generated TOML is code, not a
sentence, and is not counted.

`OK` means no length, jargon, adjective, terminology, context, or action-label
flag. Each `Lxx` entry is a finding and includes its proposed rewrite.

| Section | Exact copy (word count) | Result / proposed rewrite |
| --- | --- | --- |
| Header | “Skip to content” (3); “forge-sync” (1); “How it works” (3); “Configure” (1); “Pricing” (1); “Source ↗” (1) | OK |
| Hero | “Continuous organization mirroring” (3) | OK |
| Hero h1 | “Your GitHub org, somewhere else. Continuously.” (6) | **L01 vague headline + `org`:** `Mirror your GitHub organization to another forge.` |
| Hero | “Every repository.” (2); “Every new repo.” (3) | **L02 inconsistent `repository`/`repo`:** `Every existing and new repository.` |
| Hero | “Branches, tags, issues, labels, milestones, PR conversations, and review context—kept usable on your forge and in a portable archive.” (20) | **L03 jargon:** `Keep branches, tags, issues, labels, milestones, and pull-request discussions on your chosen forge and in a JSON archive.` |
| Hero actions | “Build your config” (3) | **L04 `config`:** `Build your configuration.` |
| Hero actions | “Download v0.1.0” (2) | **L05 result unclear:** `Download forge-sync v0.1.0.` |
| Hero facts | “Open source” (2); “One binary” (2); “No telemetry” (2); “Tokens stay local” (3) | OK for wording; all are unlisted claims. |
| Art labels | “GitHub org” (2); “JSON archive” (2); “Your forge” (2) | **L06 `org`:** `GitHub organization`; keep the other two. |
| Proof | “all new repositories discovered” (4) | **L07 unsupported absolute:** `Discovers new repositories during later runs.` |
| Proof | “1→1 stable ID mappings” (4) | **L08 jargon:** `Keeps one target record for each source record.` |
| Proof | “3 target forge families” (4) | **L09 jargon:** `Works with Forgejo, Codeberg, and GitLab.` |
| Proof | “0 tracking calls” (3) | `No tracking requests.` |
| How eyebrow | “A living copy, not a cold backup” (7) | **L10 metaphor:** `Keep a usable copy on another forge.` |
| How h2 | “One path through three durable layers.” (6) | **L11 out-of-context heading/adjective:** `How forge-sync copies and records your work.` |
| How | “01 / discover” (2); “GitHub organization” (2) | OK |
| How | “Pagination and conditional requests catch private, archived, and newly created repositories without burning the rate limit.” (16) | **L12 jargon/metaphor:** `forge-sync checks every results page and reuses unchanged responses to reduce GitHub API requests.` |
| How | “02 / preserve” (2); “Local state + JSON” (3) | **L13 jargon:** `Save mappings and JSON locally.` |
| How | “SQLite keeps idempotent mappings and an audit trail.” (8) | **L14 jargon:** `SQLite links each source record to one target record and records every run.` |
| How | “Forge-neutral snapshots are committed to your own Git archive.” (9) | **L15 jargon:** `JSON snapshots are committed to a local Git archive that is not tied to one hosting service.` |
| How | “03 / mirror” (2); “Forgejo · Codeberg · GitLab” (3) | OK |
| How | “Missing projects are created, every ref is mirror-pushed, and discussions arrive with author and source links intact.” (17) | **L16 jargon + `project` inconsistency:** `Missing repositories are created, all branches and tags are copied, and each discussion keeps its author and GitHub link.` |
| Fidelity eyebrow | “What survives the crossing” (4) | **L17 context-dependent metaphor:** `Metadata forge-sync preserves.` |
| Fidelity h2 | “Code is only half the project.” (6) | **L18 vague heading:** `Keep issues and pull-request history with the code.` |
| Fidelity | “forge-sync keeps the social record readable even when the target cannot model a GitHub object exactly.” (16) | **L19 abstract jargon:** `When a target lacks a GitHub feature, forge-sync writes its details into a readable target issue.` |
| Ledger | “Git refs” (2); “Branches + tags” (2); “True mirror push” (3) | **L20 jargon/adjective:** `Branches and tags — copied with Git mirror mode.` |
| Ledger | “Planning” (1); “Issues + milestones” (2); “State and labels preserved” (4) | `Issues and milestones — keeps state and labels.` |
| Ledger | “Review” (1); “PR conversation” (2); “Inline anchors retained as context” (5) | **L21 jargon + `PR`:** `Pull-request discussion — keeps file names and line numbers in the copied text.` |
| Ledger | “Identity” (1); “Author + timestamp” (2); “Canonical source link in body” (5) | **L22 jargon:** `Attribution — keeps the author, time, and original GitHub link.` |
| Builder | “Start safely” (2); “Make a config.” (3); “Keep secrets out of it.” (5) | **L23 `config`:** `Create a configuration. Keep secrets out of it.` |
| Builder | “This local builder never asks for tokens and sends nothing anywhere.” (11) | OK for wording; unlisted claim. |
| Builder labels | “GitHub organization” (2); “Target forge” (2); “Target URL” (2); “Target owner / namespace” (3); “Forgejo” (1); “Codeberg” (1); “GitLab” (1) | OK |
| Builder controls | “Copy config” (2); “Download” (1) | **L24 inconsistent/generic action:** `Copy configuration`; `Download configuration.` |
| Builder status | “Configuration ready.” (2); “Tokens stay in environment variables.” (5) | OK |
| Builder next | “Then validate without writing:” (4); “forge-sync doctor --config forge-sync.toml” (4) | **L25 `config`:** `Then check access without making changes:`; keep the command. |
| Operation | “Designed for unattended runs” (4) | `Run it unattended.` |
| Operation h2 | “Quiet when healthy.” (3); “Specific when not.” (3) | **L26 out-of-context/personification:** `See structured output and specific errors.` |
| Terminal | “mirror-host” (1); “forge-sync sync --config forge-sync.toml” (4); “discovered 48 repositories” (3); “mirrored 48 ref sets” (4); “synchronized 5,184 issues · 13,902 comments” (5); “archived commit 8e10f4a” (3); “Completed in 01:42 · 0 failed” (5) | **L27 jargon + unexplained sample numbers:** label this `Example output` and replace `ref sets` with `repository refs`; list/verify all numbers in claims. |
| States | “Retry-safe.” (1); “Stable source IDs prevent duplicates after interruption.” (7) | **L28 jargon:** `Restart after an interruption. Source IDs prevent duplicate target records.` |
| States | “Partial-failure aware.” (2); “One broken repo is audited while the rest continue.” (9) | **L29 jargon + `repo`:** `If one repository fails, forge-sync records the error and continues with the rest.` |
| States | “Scriptable.” (1); “Every operational command can emit JSON and meaningful exit codes.” (10) | **L30 vague adjective:** `Use JSON output and documented exit codes in scripts.` |
| Pricing | “Fair, legible pricing” (3) | **L31 marketing adjectives:** `Pricing.` |
| Pricing h2 | “The mirror is free.” (4); “The cutover kit saves a weekend.” (6) | **L32 unproved marketing promise:** `The CLI is free. The optional Migration Kit costs $39 once.` |
| Community | “Community” (1); “$0 forever” (2); “Unlimited repositories and metadata” (4); “All target adapters” (3); “Portable archive and audit log” (5); “Docker and single binary” (4); “Get the source” (3) | **L33 ambiguous `forever`/`unlimited`/`adapters`:** `Free CLI`; `No repository limit in the CLI`; `Forgejo, Codeberg, and GitLab`; keep the concrete remainder. |
| Kit | “Migration Kit” (2); “$39 one-time” (2); “Production cutover runbook” (3); “Verification and rollback checklists” (4); “Founding-buyer updates” (2); “Supports ongoing maintenance” (3); “Buy Migration Kit” (3) | **L34 vague entitlements:** replace the last two benefits with a defined update period and what maintenance receives. |
| License | “Download your runbook” (3); “Have a license? Restore purchase” (5); “License token” (2); “Verify” (1); “Your license is checked at most once per day.” (9) | **L35 generic action:** `Verify license.` |
| Merchant | “Sociobot/Dodo is the merchant of record.” (6); “Refunds are handled there and revoke the license automatically.” (9); “Terms” (1); “Privacy” (1) | OK |
| FAQ heading | “The sharp edges, stated plainly” (5); “Before you point it at production.” (6) | **L36 context-dependent headings:** `Limits to check before a production sync.` |
| FAQ | “Does it recreate pull requests?” (5) | OK |
| FAQ | “It preserves pull-request descriptions, reviews, discussion comments, and inline file/line anchors in a labeled target issue.” (16) | **L37 jargon:** `It copies each pull request into a labeled target issue with its description, reviews, comments, file names, and line numbers.` |
| FAQ | “This remains valid after merge, branch deletion, or cross-fork work.” (10) | **L38 unclear `This`:** `The copied issue remains readable after a merge, branch deletion, or contribution from a fork.` |
| FAQ | “Does it sync changes back to GitHub?” (7); “Not in v1.” (3) | OK |
| FAQ | “The experimental relay flag records intent but sends nothing.” (9) | **L39 jargon:** `The experimental setting records a warning but does not copy target comments back to GitHub.` |
| FAQ | “Reliable bidirectional identity and review-anchor mapping needs more real-world validation.” (9) | **L40 jargon/grammar:** `Two-way comment copying needs more testing to preserve authors and line references correctly.` |
| FAQ | “Where do tokens and data go?” (6); “Tokens stay in environment variables.” (5) | OK |
| FAQ | “Metadata goes only to your chosen target, local SQLite state, and your local JSON archive.” (15); “The documentation site never receives it.” (6) | **L41 jargon:** `Issue and pull-request data goes only to your target forge, local database, and local JSON archive. This website does not receive it.` |
| Closing | “Keep the exit open” (4); “Your forge should be a choice, not a single point of memory.” (12); “Build your config” (3) | **L42 metaphor + `config`:** `Keep an independent copy`; `Build your configuration.` |
| Footer | “Open infrastructure for a more portable commons.” (7); “Privacy” (1); “Terms” (1); “GitHub” (1) | **L43 abstract marketing:** `Mirror GitHub organizations to Forgejo, Codeberg, or GitLab.` |
| Builder errors | “Complete the fields above to generate a safe configuration.” (9); “Use a valid GitHub organization name.” (6); “Use a valid target owner or namespace.” (7); “Enter a complete target URL, including https://.” (7); “Use HTTPS for a remote target.” (6) | **L44 vague `safe`:** `Complete the fields above to generate the configuration.` The other errors state what to correct and are OK. |
| Clipboard state | “Copied” (1); “Clipboard access was blocked.” (4); “Select the configuration and copy it manually.” (7) | **L45 result label:** `Configuration copied.` Keep the two-part error. |
| License states | “Migration Kit unlocked on this device.” (6); “License no longer active.” (4); “You can purchase a new license below.” (7); “Offline — using the last valid license check.” (7); “License check is unavailable.” (4); “The free CLI remains fully available.” (7); “Checking license…” (2) | **L46 vague `fully`:** `The free CLI is still available.` The remaining states are concrete. |
| Offline banner | “You’re offline.” (2); “Docs and the config builder still work; license checks will resume later.” (11) | **L47 `Docs`/`config`:** `Documentation and the configuration builder still work. License checks will resume when you reconnect.` |
| Accessible control names | “Download your migration runbook” (4); “Verify license” (2) | OK; expose `Verify license` as the visible label too (L35). |

## Copy audit — README

Code blocks are excluded because they are commands/configuration, not
sentences. Headings and list items are included. `Rxx` is an individual finding
with its concrete rewrite.

| # | Exact sentence or copy unit | Words | Result / proposed rewrite |
| --- | --- | ---: | --- |
| 1 | “forge-sync” | 1 | OK |
| 2 | “`forge-sync` continuously mirrors every repository in a GitHub organization to Forgejo, Codeberg, or GitLab.” | 14 | OK |
| 3 | “It discovers new repositories, mirrors every Git ref, carries issues and pull-request discussions into the target with source attribution, and writes the same metadata to a portable JSON archive.” | 29 | **R01 >22 + jargon:** `It finds new repositories and copies all branches and tags. It also copies attributed issues and pull-request discussions to the target and a JSON archive.` |
| 4 | “It is for maintainers and small organizations that want an independently usable copy of their work before, during, or after a forge migration.” | 23 | **R02 >22:** `It is for maintainers who need an independent copy before, during, or after moving to another forge.` |
| 5 | “The core is open source, local-first, and has no telemetry.” | 10 | **R03 jargon `local-first`:** `The CLI is open source, stores its state locally, and sends no telemetry.` |
| 6 | “Install” | 1 | OK |
| 7 | “Prebuilt binaries are attached to GitHub releases.” | 7 | OK |
| 8 | “From source (Rust 1.85+):” | 4 | OK |
| 9 | “Or build and run the container:” | 6 | OK |
| 10 | “Quick start” | 2 | OK |
| 11 | “Create `forge-sync.toml` (tokens may use environment-variable names; never put token values in the file):” | 14 | **R04 dense parenthesis:** `Create forge-sync.toml. Name the environment variables that hold your tokens; do not put token values in this file.` |
| 12 | “Validate access without changing either forge, then perform one complete pass:” | 11 | OK |
| 13 | “Run continuously (SIGINT/SIGTERM exits cleanly):” | 6 | **R05 jargon:** `Run continuously. Ctrl+C and termination signals stop it cleanly:` |
| 14 | “Every command supports scripting output:” | 5 | **R06 vague:** `Operational commands can return JSON:` |
| 15 | “Exit codes are `0` success, `2` configuration/usage (including unreadable or invalid TOML and missing configured token variables), `3` authentication, `4` API/rate-limit, `5` git transport, and `6` partial synchronization.” | 30 | **R07 >22:** Split into a table: `0 success; 2 configuration or usage; 3 authentication; 4 API or rate limit; 5 Git transport; 6 partial sync.` Put the code-2 cases beneath it. |
| 16 | “Commands never prompt.” | 3 | OK |
| 17 | “What gets synchronized” | 3 | OK |
| 18 | “organization repository discovery, including repositories created after the daemon starts;” | 10 | **R08 jargon `daemon`:** `repositories created after continuous sync starts;` |
| 19 | “branches, tags, and other Git refs via a true mirror push;” | 11 | **R09 jargon/adjective:** `all Git branches, tags, and other references through Git mirror mode;` |
| 20 | “labels and milestones;” | 3 | OK |
| 21 | “issues, state, body, labels, milestone, and comments;” | 7 | OK |
| 22 | “pull request description, review summaries, inline review comments, and conversation comments, represented as a clearly labeled target issue so closed and cross-fork PR history remains portable;” | 26 | **R10 >22 + `PR`:** `pull-request descriptions, review summaries, inline comments, and discussion comments. forge-sync stores them in a labeled target issue so they remain readable after closure or work from a fork;` |
| 23 | “author, timestamps, and canonical GitHub links in every mirrored body;” | 10 | **R11 jargon:** `the author, time, and original GitHub link in every copied body;` |
| 24 | “stable source→target ID mappings and append-only audit events in SQLite;” | 10 | **R12 jargon:** `one source-to-target ID link and a permanent SQLite event record for each action;` |
| 25 | “forge-neutral JSON snapshots, committed to a local Git archive when enabled.” | 11 | **R13 jargon:** `JSON snapshots that work across supported forges, optionally committed to a local Git archive.` |
| 26 | “Updates are idempotent.” | 3 | **R14 jargon:** `Repeating a sync does not create duplicates.` |
| 27 | “forge-sync writes a hidden source marker into target content and retains its mapping database, so restarting or retrying does not duplicate objects.” | 22 | **R15 jargon:** `forge-sync adds a hidden source ID to target content and keeps a mapping database. Restarting or retrying does not create duplicates.` |
| 28 | “GitHub conditional requests and pagination reduce rate-limit use.” | 8 | **R16 jargon:** `forge-sync checks every results page and asks GitHub to resend only changed data.` |
| 29 | “A failed repository does not prevent the remainder of the organization from being processed; the run exits `6` and records the failure.” | 22 | `If one repository fails, forge-sync records it and continues with the rest. The command exits with code 6.` |
| 30 | “Configuration” | 1 | OK |
| 31 | “Run `forge-sync config example` for the complete annotated file.” | 9 | **R17 `config`:** `Run forge-sync config example to print the complete configuration with notes.` |
| 32 | “Important optional controls:” | 3 | OK |
| 33 | “`forge-sync sync --dry-run` reads the source and target to produce the same report as a pass, but never changes either forge, Git refs, SQLite state, discovery cache/audit data, or the JSON archive.” | 33 | **R18 >22 + jargon:** `forge-sync sync --dry-run reads both forges and reports planned changes. It does not change either forge, Git data, local state, the audit log, or the JSON archive.` |
| 34 | “It is safe to run before the first real migration; the next real run creates every missing object it found.” | 20 | **R19 vague `safe`/`object`:** `Run it before a migration to review planned changes. A later real run creates the repositories and records it found.` |
| 35 | “Bidirectional comment relay is intentionally disabled in v1; setting the experimental flag records a warning but never sends a target comment to GitHub.” | 23 | **R20 >22 + jargon:** `Version 1 does not copy target comments back to GitHub. The experimental setting records a warning but sends no comment.` |
| 36 | “This avoids silently misattributing authors or drifting inline anchors.” | 9 | **R21 unclear `This` + jargon:** `One-way copying avoids assigning comments to the wrong author or line.` |
| 37 | “Data and security” | 3 | OK |
| 38 | “Tokens are read only from environment variables.” | 7 | OK |
| 39 | “They are never written to config, logs, SQLite, or the JSON archive.” | 12 | **R22 `config`:** `Tokens are never written to configuration, logs, the local database, or the JSON archive.` |
| 40 | “Use a GitHub fine-grained token with organization repository read access and a target token allowed to create repositories/issues and push Git.” | 22 | **R23 jargon/slash:** `Create a GitHub fine-grained token with read access to the organization's repositories. Give the target token permission to create repositories and issues and push Git data.` |
| 41 | “Private target repositories are the default.” | 6 | OK |
| 42 | “`forge-sync doctor` checks identities and permissions before a first run.” | 10 | OK |
| 43 | “The archive contains the organization content you elect to mirror.” | 10 | **R24 formal `elect`:** `The archive contains the organization content you choose to copy.` |
| 44 | “Protect it like the source organization.” | 6 | OK |
| 45 | “See `SECURITY.md` for safe reporting and token guidance.” | 9 | OK |
| 46 | “Development” | 1 | OK |
| 47 | “The repository contains integration tests backed by a local mock HTTP forge and temporary Git repositories; tests do not require network credentials.” | 22 | **R25 jargon:** `Integration tests use temporary local Git repositories and simulated forge APIs. They need no network credentials.` |
| 48 | “Deployment” | 1 | OK |
| 49 | “The CLI is a single release binary or container.” | 9 | OK |
| 50 | “Run exactly one daemon per state directory.” | 7 | **R26 jargon:** `Run only one continuous forge-sync process for each state directory.` |
| 51 | “The documentation site is static and the factory deploys `dist/site`; it does not receive tokens or mirror data.” | 19 | **R27 internal jargon:** `The documentation site is static. It receives neither tokens nor repository data.` |
| 52 | “License” | 1 | OK |
| 53 | “MIT.” | 1 | OK |
| 54 | “See `LICENSE`.” | 3 | OK |

README average sentence/copy-unit length is 10.4 words (559 words / 54
units), but six individual units exceed the 22-word hard cap: rows 3, 4, 15,
22, 33, and 35. The landing page has no sentence over 22 words once sentences
are separated correctly; its failures are clarity, jargon, consistency, and
action naming.

## Structure, behavior, and verification record

| Check | Result |
| --- | --- |
| Fresh phone/desktop contexts | FAIL as B1; no horizontal overflow and no console errors. |
| Demo URL and CLI temp-dir command | FAIL as B2. `/demo` 404; query ignored; CLI exits 2 and writes nothing. |
| Demo banner, Reset, Start for real, storage separation | FAIL as B2; none exists, so real-data isolation is untestable. |
| Claims manifest/tests | FAIL as B3; file and tags absent. |
| Offline/privacy spot check | Ordinary landing/config flow contacted only `forge-sync.sociobot.in`; offline reload after first visit passed. No demo sandbox exists. |
| Title pattern per existing route | PASS: home, `Privacy — forge-sync`, and `Terms — forge-sync` are route-specific and under 60 characters. |
| Semantic basics | PASS: `lang=en`, one h1, main landmark, image alt, and no console errors on existing routes. |
| Metadata/crawl | FAIL as M2. |
| 404/deep routes | FAIL as B4. |
| Back/focus | FAIL as M3. |
| Dead-link crawl | FAIL as M1. Home, Privacy, Terms, GitHub source, and releases returned 200; checkout returned 404; mailto links were exempt. |
| Header/footer consistency | FAIL as M4. |
| Visual identity | PASS: asymmetric editorial layout, ceramic transfer art, mineral palette, and serif/sans pairing are recognizably product-specific rather than a centered gradient/three-card template. Asset provenance is recorded in `.factory/design.md`. |
| Accessibility automation | PASS: `npm run test:a11y -- https://forge-sync.sociobot.in` reported 0 axe violations; the supplied URL verifier reported one h1, `lang`, main, alt coverage, and zero console errors. |
| Clean-clone tests | PASS: `npm ci`; `npm test` (4 Node tests and 9 Rust tests); `npm run build`. Build produced `dist/site`; JS was 5.51 kB raw / 2.47 kB gzip. |

No product code was modified during this review.
