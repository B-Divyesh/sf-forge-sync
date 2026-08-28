# Landing copy audit — polish round 3

Words are whitespace-delimited. Commands, file paths, generated terminal
output, the wordmark, and forge names inside form options are excluded. This
lists every visitor-facing landing and demo-mode unit in `site/index.html` and
`site/main.js`; all are at most 22 words.

| Area | Copy unit | Words | Result |
| --- | --- | ---: | --- |
| Access/navigation | Skip to content; Demo; How it works; Privacy | 3; 1; 3; 1 | OK |
| Demo banner | Demo — sample data, nothing is saved. | 7 | Claim: demo-browser-isolation |
| Demo actions | Reset demo; Leave demo and build configuration | 2; 5 | OK — result is named |
| Demo panel | Sample mirror ready; See a completed sample mirror. | 3; 5 | OK |
| Demo panel | Harbor Cooperative’s harbor-tools repository has two branches, one tag, and one pull-request record. | 13 | Claim: demo-completed-mirror |
| Demo panel | It includes links between GitHub and target records, dated run history, and a JSON archive saved in Git. | 18 | Claim: demo-completed-mirror; configured-record-links-run-history |
| Demo recording | Captured from the current CLI and bundled sample data.; Read the captured transcript | 10; 4 | Claim: demo-recording-current-output |
| First screen | GitHub organization mirror; Mirror your GitHub organization to another forge. | 3; 7 | OK |
| First-screen audience | For maintainers who need a copy on Forgejo, Codeberg, or GitLab with issues and pull-request history. | 16 | Declared capability claims |
| First-screen actions | Try it with sample data; See a completed sample mirror; nothing is saved.; Build your configuration | 5; 8; 3 | OK |
| First-screen facts | Free under the MIT License.; No website analytics.; Works offline after your first visit. | 5; 3; 6 | Declared claims |
| Art labels | GitHub organization; JSON archive; Your forge | 2; 2; 2 | OK |
| Sample flow | How the sample is arranged; Copy code, issues, and pull-request history. | 5; 6 | OK |
| Sample flow | The sample starts with Harbor Cooperative’s harbor-tools repository. | 8 | Claim: demo-completed-mirror |
| Sample flow | JSON records, links between GitHub and target records, and dated run history stay together. | 14 | Claim: configured-record-links-run-history |
| Sample flow | The sample shows branches, tags, and a readable pull-request issue. | 10 | Claim: demo-completed-mirror |
| Sample contents | What the sample contains; Keep issues and pull-request history with the code. | 4; 9 | OK |
| Sample contents | The completed sample makes each copied record visible before you connect a real organization. | 15 | Claim: demo-completed-mirror |
| Ledger | Git data; Branches and tags; Two branches and one version tag. | 2; 3; 6 | Claim: demo-completed-mirror |
| Ledger | Issue record; Pull-request discussion; Author, review, file name, and line number. | 2; 2; 7 | Claim: demo-completed-mirror |
| Ledger | Local record; Links between GitHub and target records; One link and three dated run-history entries. | 2; 6; 7 | Claim: configured-record-links-run-history |
| Ledger | Archive; JSON archive saved in Git; A Git commit records the sample archive. | 1; 5; 7 | Claim: demo-completed-mirror |
| Builder | Build locally; Create a configuration without token values. | 2; 6 | Claim: configuration-has-no-token-field |
| Builder | This browser tool creates text only. It does not ask for a token. | 6; 7 | Claim: configuration-has-no-token-field |
| Builder controls | GitHub organization; Target forge; Target URL; Target owner or namespace; Copy configuration; Download configuration | 2; 2; 2; 4; 2; 2 | OK |
| Builder next step | Then check access without making changes: | 6 | Claim: doctor-read-only |
| CLI sample | Run the sample; Inspect a mirror before using your data. | 3; 8 | OK |
| CLI sample | Separate output. The command creates a new temporary directory. | 2; 7 | Claim: demo-completed-mirror |
| CLI sample | Disposable data. Remove the printed directory when you finish. | 2; 7 | OK |
| CLI sample | Sample source files. The source JSON ships in examples/sample-mirror. | 3; 6 | Claim: demo-completed-mirror |
| Closing/footer | Start with sample data; See the sample, then build your configuration. | 4; 8 | OK |
| Footer | Mirror GitHub organizations to another forge.; Privacy; Terms; GitHub source | 6; 1; 1; 2 | OK |
| Dynamic success | Demo reset. The sample data is new. | 2; 5 | Claim: demo-browser-isolation |
| Dynamic success | Configuration ready. Add token environment-variable names when you run the CLI. | 2; 9 | OK |
| Dynamic feedback | Configuration copied. Clipboard access was blocked. Select the configuration and copy it manually. | 2; 4; 7 | OK |
| Dynamic error | Complete the fields above to generate the configuration. | 8 | OK |
| Dynamic validation | Use a valid GitHub organization name.; Use a valid target owner or namespace.; Enter a complete target URL, including https://.; Use HTTPS for a remote target. | 6; 7; 7; 6 | OK |
| Offline | You’re offline. The sample and configuration builder remain available after a first visit. | 2; 10 | Claim: offline-demo-after-first-visit |

The banned-word scan found no matches for `leverage`, `seamless`, `effortless`,
`robust`, `powerful`, `intuitive`, `reimagine`, `supercharge`, `delightful`,
`journey`, `ecosystem`, or `AI-powered`.

## Terminology

| Concept | One term used |
| --- | --- |
| GitHub account group | organization |
| Code container | repository |
| Review record | pull request / pull-request record |
| Setup file | configuration |
| Example mode | sample / demo |
| Persistent local data | local record / JSON archive |
| Source-to-target connection | link between GitHub and target records |
| Execution log | dated run history |
