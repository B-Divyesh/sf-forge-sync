# Perfection loop polish 1 — finding closure

Candidate `5c5299059d7f1f6796c29bf75480066669a5dc00` was checked against
`.factory/review-1.md` from `70aadfd2482413bec11be55203e87d6683823138`.
No earlier `review-*.md` or `polish-*.md` exists in repository history. The
older verification findings embedded in that history were also rechecked.

Evidence keys used below:

- `browser`: `routes, metadata, focus, and phone first action work`.
- `demo-web`: `@claim:demo-browser-isolation`.
- `demo-cli`: `@claim:demo-completed-mirror`.
- `privacy`: `@claim:website-no-tracking` and
  `@claim:configuration-has-no-token-field`.
- `targets`: `@claim:supported-targets`.
- `dry-run`: `@claim:dry-run-read-only`.
- `doctor`: `@claim:doctor-read-only`.
- `static`: `all routes ship complete product metadata and the shared
  skeleton` plus `crawl surfaces list every route and no dead checkout
  remains`.
- Local screenshots: `.factory/evidence/polish-1-local/home-mobile.png`,
  `demo-mobile.png`, `home-desktop.png`, and `home-text-200.png`.
- Live evidence: `.factory/evidence/polish-1-live/` and the cold checks at
  `https://forge-sync.sociobot.in/`, `/?demo=1`, `/demo/`, `/privacy/`,
  `/terms/`, and `/definitely-missing-review-route`.

## Severity findings

| ID | Change made | Evidence |
| --- | --- | --- |
| B1 | Kept the requested seven-word job headline, named maintainers and all three targets, grouped the sample explanation with the primary action, and kept the action above the fold at 390 px. | `browser`; home mobile/desktop screenshots; live cold check. |
| B2 | Added direct `/?demo=1` and `/demo/` entry, fixed the leaked banner on real mode, isolated all browser state under `demo:forge-sync:`, made reset replace only demo state, made Start for real discard it, and made `forge-sync demo` build real SQLite/archive output in a new temporary directory from shipped fixtures. | `demo-web`; `demo-cli`; demo mobile screenshot; live cold check. |
| B3 | Added a non-empty ten-claim manifest, one unique tagged test per claim, and a runner that rejects missing or duplicate tags before executing every command. | `npm run test:claims`; `each declared claim has exactly one tagged test and a runnable command`. |
| B4 | Added the ceramic-styled 404, shared site skeleton, Static Web Apps 404 override with status 404, and an explicit `/404` sitemap entry. | `static`; live missing-route response and screenshot. |
| M1 | Removed the unavailable Migration Kit, checkout, license UI, price, and all billing claims instead of leaving a dead purchase action. | `crawl surfaces list every route and no dead checkout remains`; live link crawl. |
| M2 | Added route-specific descriptions, canonicals, Open Graph/Twitter fields, 1200×630 artwork metadata, touch icon, robots, and sitemap. Query demo mode now updates its title, description, canonical, and social fields. | `static`; `browser`; live metadata check. |
| M3 | Hash navigation now stores destination scroll positions, restores back and forward scroll, focuses the destination heading, and announces it. Direct hash loads focus the named heading. | `browser` covers direct URLs, click, back, forward, focus, and scroll. |
| M4 | Home, demo, privacy, terms, and 404 share the same wordmark/header/nav/footer, legal links, Param Factory credit, and build identifier. The identifier is injected from Git at build time. | `static`; live footer check. |
| M5 | Rewrote or removed every flagged term and published the exhaustive landing/dynamic copy audit with consistent organization, repository, pull request, configuration, sample, and JSON archive terms. | `.factory/copy-audit.md`; `browser`; live cold read. |

## Original claim findings

| ID | Change made | Evidence |
| --- | --- | --- |
| U01 | Removed the absolute “every new repository” landing claim; the page now describes the shipped one-repository sample. | Copy audit; `demo-cli`. |
| U02 | Replaced the broad object list with sample-scoped branches, tags, pull-request discussion, mapping, audit, and archive records. | `demo-cli`. |
| U03 | Replaced the four-part badge with tested MIT, no-analytics, and offline facts; removed the untested one-binary/token-local badge. | license, privacy, and offline claim tests. |
| U04 | Removed “all new repositories discovered” from the page. | Copy audit and source search. |
| U05 | Replaced “1→1 stable ID mappings” with one observable sample mapping in real SQLite state. | `demo-cli`. |
| U06 | Retained the three supported target names and added local API-contract creation tests for each. | `targets`. |
| U07 | Replaced “0 tracking calls” with “No website analytics” and tests that exercise every route and builder flow without cross-origin traffic. | `privacy`. |
| U08 | Removed the pagination/rate-limit marketing sentence. | Copy audit and source search. |
| U09 | Narrowed the statement to visible sample mappings/audit and now creates them through real SQLite state code. | `demo-cli`. |
| U10 | Narrowed the archive statement to the sample and creates a real Git commit with the production Archive code. | `demo-cli`. |
| U11 | Removed the absolute production promise; sample branches, tags, author and GitHub links remain inspectable. | `demo-cli`. |
| U12 | Replaced the abstract fidelity claim with the sample’s rendered target issue. | `demo-cli`. |
| U13 | Removed the untested four-item ledger and replaced it with exact sample counts and records. | `demo-cli`; demo screenshot. |
| U14 | Kept the local builder claim and tests now edit every control, reject token fields, and reject off-origin requests. | configuration claim test. |
| U15 | Removed the universal CLI token-storage claim; the demo test uses canary tokens and scans every text output. | `demo-cli`. |
| U16 | Retained read-only doctor behavior and added a two-endpoint GET-only test with absent state/archive paths. | `doctor`. |
| U17 | Removed the fabricated 48/5,184/13,902/01:42 sample and replaced it with the shipped fixture’s asserted counts. | `demo-cli`. |
| U18 | Removed the interruption claim from landing copy. | Copy audit and source search. |
| U19 | Removed the partial-failure marketing claim from landing copy. | Copy audit and source search. |
| U20 | Removed “every operational command” and its unbounded JSON/exit-code claim. | Copy audit and README. |
| U21 | Removed “forever”, “unlimited”, and paid/free tier cards; retained only the tested MIT fact. | MIT claim test; static no-checkout test. |
| U22 | Removed “saves a weekend”. | Static no-checkout test. |
| U23 | Removed price, kit contents, and update entitlement because checkout was unavailable. | Static no-checkout test. |
| U24 | Removed license verification and daily-cache claims with the paid tier. | Static no-checkout test. |
| U25 | Removed the merchant/refund claim with the paid tier. | Static no-checkout test. |
| U26 | Narrowed pull-request fidelity to the shipped fixture and asserts description, review, discussion, file, line, author, and source URL in real rendered/archive output. | `demo-cli`. |
| U27 | Removed merge/deleted-branch/cross-fork absolutes. | Copy audit and source search. |
| U28 | Removed the experimental relay claim from the landing page. | Copy audit and source search. |
| U29 | Narrowed the privacy statement to website behavior and configured local paths; browser tests reject cross-origin requests. | `privacy`; configuration claim test. |
| U30 | Retained the plain job statement and split proof across target-contract, Git-ref, demo-metadata, and dry-run tests. | `targets`; Git-ref claim; `demo-cli`; `dry-run`. |
| U31 | Split the long README promise into short, inspectable record bullets backed by the same claim tests. | README; claim suite. |
| U32 | Replaced “local-first/no telemetry” with exact MIT and website no-analytics facts. | MIT and privacy claim tests. |
| U33 | Removed the unverified prebuilt-release statement. | README source search. |
| U34 | Removed the universal no-prompt statement from README and CLI marketing copy. | README and `--help` source search. |
| U35 | Rewrote synchronization bullets as concrete records; branches/tags and sample metadata are directly inspected. | Git-ref claim; `demo-cli`. |
| U36 | Removed the unlisted idempotence copy. | README and landing source search. |
| U37 | Removed the conditional-request/rate-limit copy. | README and landing source search. |
| U38 | Removed the partial-failure/exit-6 promise from visitor copy. | README and landing source search. |
| U39 | Retained and expanded the dry-run guarantee, including a following real run. | `dry-run`. |
| U40 | Removed the relay behavior claim from visitor copy. | README and landing source search. |
| U41 | Removed the universal token-write claim; configuration and demo canary isolation remain explicitly tested. | configuration claim; `demo-cli`. |
| U42 | Removed the prose default claim; target-contract tests assert private creation for every adapter. | `targets`. |
| U43 | Retained the doctor workflow and added a GET-only, no-filesystem-write claim test. | `doctor`. |
| U44 | Removed the unlisted “tests need no credentials” marketing statement. | README source search. |
| U45 | Retained the static-site privacy statement in narrower form and tests all website flows for outside traffic. | `privacy`; production build artifact inspection. |
| U46 | Retained offline sample/builder behavior and exercises the builder after an offline reload. | offline claim test. |
| U47 | Removed the paid license feature and offline-verdict claim. | Static no-checkout test. |

## Landing copy findings

| ID | Change made | Evidence |
| --- | --- | --- |
| L01 | Replaced the vague headline with “Mirror your GitHub organization to another forge.” | Copy audit; screenshots. |
| L02 | Uses “repository” consistently; removed “repo” from prose. | Copy audit. |
| L03 | Uses “pull-request” and sample-scoped record copy. | Copy audit; `demo-cli`. |
| L04 | Renamed the action “Build your configuration”. | Copy audit; browser test. |
| L05 | Removed the unavailable download action. | Static link test. |
| L06 | Art label now says “GitHub organization”. | Copy audit; screenshots. |
| L07 | Removed the absolute discovery proof. | Copy audit. |
| L08 | Replaced mapping jargon with one visible sample mapping. | `demo-cli`. |
| L09 | Names Forgejo, Codeberg, and GitLab directly. | Copy audit; `targets`. |
| L10 | Removed the cold-backup metaphor. | Copy audit. |
| L11 | Replaced the vague layer heading with “Copy code and its working record.” | Copy audit. |
| L12 | Removed the rate-limit paragraph from the landing page. | Copy audit. |
| L13 | Uses “Local archive” and plain record copy. | Copy audit. |
| L14 | Replaced “idempotent” with an observable SQLite sample mapping. | `demo-cli`. |
| L15 | Removed “forge-neutral” and describes committed JSON. | Copy audit. |
| L16 | Uses repository, branches, tags, discussion author, and GitHub link. | `demo-cli`. |
| L17 | Replaced crossing metaphor with “What the sample contains”. | Copy audit. |
| L18 | Heading now says “Keep issues and pull-request history with the code.” | Copy audit. |
| L19 | Uses the concrete rendered sample record instead of abstract social-record prose. | `demo-cli`. |
| L20 | Replaced “true mirror push” with “Branches and tags”. | Copy audit; Git-ref claim. |
| L21 | Replaced “PR/anchors” with pull-request discussion, file name, and line number. | Copy audit; `demo-cli`. |
| L22 | Replaced canonical/attribution jargon with author, review, file, line, and source URL evidence. | `demo-cli`. |
| L23 | Builder heading uses “configuration”. | Copy audit. |
| L24 | Actions read “Copy configuration” and “Download configuration”. | Browser test. |
| L25 | Next step reads “Then check access without making changes”. | Copy audit; `doctor`. |
| L26 | Replaced personified health copy with “Inspect a mirror before using your data.” | Copy audit. |
| L27 | Removed invented large counts and shows exact sample counts. | `demo-cli`; demo screenshot. |
| L28 | Removed “Retry-safe” copy. | Copy audit. |
| L29 | Removed “Partial-failure aware” copy. | Copy audit. |
| L30 | Removed the universal scriptability claim. | Copy audit. |
| L31 | Removed the entire pricing section after checkout failure. | Static no-checkout test. |
| L32 | Removed both free/kit comparative claims; first screen states MIT only. | MIT claim test. |
| L33 | Removed “forever”, “unlimited”, and adapter jargon. | Copy audit. |
| L34 | Removed undefined paid entitlements. | Static no-checkout test. |
| L35 | Removed license verification UI. | Static no-checkout test. |
| L36 | Removed the marketing FAQ heading. | Copy audit. |
| L37 | Moved detailed fidelity into the inspectable sample record. | `demo-cli`. |
| L38 | Removed the unclear post-merge absolute. | Copy audit. |
| L39 | Removed relay copy from the landing page. | Copy audit. |
| L40 | Removed the two-way copying roadmap claim. | Copy audit. |
| L41 | Privacy page now names website, local paths, repository data, and access tokens plainly. | Privacy claim test. |
| L42 | Closing now says “See the sample, then build your configuration.” | Copy audit. |
| L43 | Footer now says “Mirror GitHub organizations to another forge.” | Static skeleton test. |
| L44 | Builder fallback says “generate the configuration”; no vague “safe”. | Browser test. |
| L45 | Clipboard success says “Configuration copied.” | Browser source and copy audit. |
| L46 | Removed all paid-license state copy. | Static no-checkout test. |
| L47 | Offline copy spells out sample and configuration builder. | Offline claim test. |

## README copy findings

| ID | Change made | Evidence |
| --- | --- | --- |
| R01 | Split the long scope sentence into the headline, audience paragraph, and record list. | README; copy audit conventions. |
| R02 | Audience sentence is now 12 words. | README. |
| R03 | Removed “local-first” and the broad telemetry claim. | README; privacy claim. |
| R04 | Token instructions are two short sentences without parentheses. | README. |
| R05 | Replaced signal jargon with one direct daemon command sentence. | README. |
| R06 | Narrowed JSON wording to `status` and `sync`. | README. |
| R07 | Removed the long exit-code marketing paragraph. | README. |
| R08 | Uses “continuous passes”, not daemon jargon, outside the command name. | README. |
| R09 | Uses branches and tags rather than “true mirror push”. | README; Git-ref claim. |
| R10 | Split pull-request content into short record bullets. | README; `demo-cli`. |
| R11 | Uses author, time, and original GitHub link. | README; `demo-cli`. |
| R12 | Uses source-to-target ID links and audit events. | README; `demo-cli`. |
| R13 | Uses JSON snapshots and local Git archive. | README; `demo-cli`. |
| R14 | Removed “idempotent”. | README. |
| R15 | Removed the hidden-marker retry paragraph. | README. |
| R16 | Removed conditional-request jargon. | README. |
| R17 | Removed the annotated-config marketing sentence. | README. |
| R18 | Split dry-run behavior into two short sentences. | README; `dry-run`. |
| R19 | Uses “reports planned changes”; no vague “safe/object”. | README; `dry-run`. |
| R20 | Removed relay marketing text. | README. |
| R21 | Removed the unclear misattribution sentence. | README. |
| R22 | Token setup says not to put token values in configuration. | README; configuration and demo claim tests. |
| R23 | Removed the dense permission sentence from the quick path. | README. |
| R24 | Removed the formal archive warning sentence. | README. |
| R25 | Removed the internal implementation claim from visitor documentation. | README. |
| R26 | Uses `forge-sync daemon` only as the actual command name. | README. |
| R27 | Deployment text is limited to the build output and local run command. | README; `npm run build`. |

## Earlier verification regressions rechecked

| ID | Status | Evidence |
| --- | --- | --- |
| P0 | Dry run remains fully read-only and a following real run creates target records. | `@claim:dry-run-read-only`. |
| P1 | Reduced-motion focus rings remain 3 px; 390 px keyboard traversal is tested. | `browser`; `npm run test:a11y`. |
| P2-exit | Configuration failures still return exit code 2. | `all_documented_configuration_failures_use_exit_code_two`. |
| P2-host | Immutable asset cache and security headers remain in deployment config and are checked live. | deployment test; live header capture. |
