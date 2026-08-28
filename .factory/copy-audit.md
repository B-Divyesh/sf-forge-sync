# Landing copy audit — polish round 1

Words are whitespace-delimited. Code, file paths, command output, the wordmark,
and forge names inside form options are excluded. Every visitor-facing copy
unit in `site/index.html` and every dynamic status in `site/main.js` is listed.

| Area | Copy unit | Words | Result |
| --- | --- | ---: | --- |
| Access | Skip to content | 3 | OK |
| Navigation | Demo / How it works / Privacy | 5 | OK |
| Demo banner | Demo — sample data, nothing is saved. | 7 | OK |
| Demo banner | Reset demo / Start for real | 5 | OK |
| Demo panel | Sample mirror ready | 3 | OK |
| Demo panel | See a completed sample mirror. | 5 | OK |
| Demo panel | Harbor Cooperative has one repository, branches, a tag, a pull-request record, mappings, audit events, and a committed JSON archive. | 18 | OK |
| Demo panel | Open the full demo record | 6 | OK |
| First screen | GitHub organization mirror | 3 | OK |
| First screen | Mirror your GitHub organization to another forge. | 7 | OK |
| First screen | For maintainers who need a usable copy on Forgejo, Codeberg, or GitLab, including issues and pull-request history. | 17 | OK |
| First screen | Try it with sample data | 5 | OK |
| First screen | Build your configuration | 3 | OK |
| First screen | See a completed sample mirror; nothing is saved. | 8 | OK |
| First screen | Free under the MIT License. | 5 | OK |
| First screen | No website analytics. | 3 | OK |
| First screen | Works offline after your first visit. | 6 | OK |
| Art labels | GitHub organization / JSON archive / Your forge | 7 | OK |
| Process | How the sample is arranged | 5 | OK |
| Process | Copy code and its working record. | 6 | OK |
| Process | Source / GitHub organization | 3 | OK |
| Process | The sample starts with Harbor Cooperative’s harbor-tools repository. | 8 | OK |
| Process | Record / Local archive | 3 | OK |
| Process | JSON records, source-to-target IDs, and audit events remain together. | 9 | OK |
| Process | Target / Another forge | 3 | OK |
| Process | The sample shows branches, tags, and a readable pull-request issue. | 10 | OK |
| Contents | What the sample contains | 4 | OK |
| Contents | Keep issues and pull-request history with the code. | 9 | OK |
| Contents | The completed sample makes each copied record visible before you connect a real organization. | 15 | OK |
| Contents | Git data / Branches and tags / Two branches and one version tag. | 12 | OK |
| Contents | Issue record / Pull-request discussion / Author, review, file name, and line number. | 12 | OK |
| Contents | Local record / Mappings and audit / One source-to-target ID and three events. | 13 | OK |
| Contents | Archive / Committed JSON / A Git commit records the sample archive. | 11 | OK |
| Builder | Build locally | 2 | OK |
| Builder | Create a configuration without token values. | 6 | OK |
| Builder | This browser tool creates text only. | 6 | OK |
| Builder | It does not ask for a token. | 7 | OK |
| Builder labels | GitHub organization / Target forge / Target URL / Target owner or namespace | 11 | OK |
| Builder actions | Copy configuration / Download configuration | 4 | OK |
| Builder | Then check access without making changes: | 7 | OK |
| Sample | Run the sample | 3 | OK |
| Sample | Inspect a mirror before using your data. | 8 | OK |
| Sample | Separate output. | 2 | OK |
| Sample | The command creates a new temporary directory. | 7 | OK |
| Sample | Disposable data. | 2 | OK |
| Sample | Remove the printed directory when you finish. | 7 | OK |
| Sample | Real fixture. | 2 | OK |
| Sample | The source JSON ships in examples/sample-mirror. | 6 | OK |
| Closing | Start with a fixture | 4 | OK |
| Closing | See the sample, then build your configuration. | 8 | OK |
| Footer | Mirror GitHub organizations to another forge. | 6 | OK |
| Footer | Privacy / Terms / GitHub source | 5 | OK |
| Dynamic | Demo reset. | 2 | OK |
| Dynamic | The sample data is new. | 5 | OK |
| Dynamic | Configuration ready. | 2 | OK |
| Dynamic | Add token environment-variable names when you run the CLI. | 9 | OK |
| Dynamic | Complete the fields above to generate the configuration. | 8 | OK |
| Dynamic | Configuration copied. | 2 | OK |
| Dynamic | Clipboard access was blocked. | 4 | OK |
| Dynamic | Select the configuration and copy it manually. | 7 | OK |
| Dynamic | You’re offline. | 2 | OK |
| Dynamic | The sample and configuration builder remain available after a first visit. | 10 | OK |
| Errors | Use a valid GitHub organization name. | 6 | OK |
| Errors | Use a valid target owner or namespace. | 7 | OK |
| Errors | Enter a complete target URL, including https://. | 7 | OK |
| Errors | Use HTTPS for a remote target. | 6 | OK |

No unit exceeds 22 words. The banned-word scan found no matches for leverage,
seamless, effortless, robust, powerful, intuitive, reimagine, supercharge,
delightful, journey, ecosystem, or AI-powered.

## Terminology

| Concept | One term used |
| --- | --- |
| GitHub account group | organization |
| Code container | repository |
| Review record | pull request / pull-request record |
| Setup file | configuration |
| Example mode | sample / demo |
| Stored data | JSON archive |
