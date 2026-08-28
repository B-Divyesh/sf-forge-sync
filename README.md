# forge-sync

Mirror your GitHub organization to another forge.

forge-sync is for maintainers who need an independent copy on Forgejo,
Codeberg, or GitLab. It writes a local record and a JSON archive alongside the
target copy.

## Try the completed sample

Run this before configuring a real organization:

```sh
cargo run -- demo
```

Open the isolated [browser sample](https://forge-sync.sociobot.in/?demo=1) in
one click.

The command creates a new temporary directory and prints its path. It does not
read your configuration or token values. The output contains the fictional
Harbor Cooperative `harbor-tools` repository, two branches, and one tag. It
also contains a pull-request record and a committed JSON archive. SQLite stores
links between GitHub and target records plus dated run history. Delete the
printed directory when you finish.

The source records are in [`examples/sample-mirror`](examples/sample-mirror/).
See [`.factory/demo.md`](.factory/demo.md) for browser and CLI sandbox details.

## Install

Build from source with Rust 1.88 or newer:

```sh
cargo install --path .
forge-sync --help
```

## Configure a real mirror

Create `forge-sync.toml`. Name the environment variables that hold tokens.
Do not put token values in this file.

```toml
[source]
org = "acme"
token_env = "GITHUB_TOKEN"

[target]
kind = "forgejo" # forgejo, codeberg, or gitlab
base_url = "https://codeberg.org"
owner = "acme-mirror"
token_env = "FORGE_TOKEN"

[sync]
interval_seconds = 300
include_archived = true
private = true
state_dir = ".forge-sync"
archive_dir = "forge-archive"
git_archive = true
```

Check access before making changes, then run a pass:

```sh
export GITHUB_TOKEN=github_pat_…
export FORGE_TOKEN=…
forge-sync doctor --config forge-sync.toml
forge-sync sync --config forge-sync.toml
```

Run `forge-sync daemon --config forge-sync.toml` for continuous passes. Add
`--json` to `status` or `sync` when a script needs JSON output.

## What it records

- repository discovery, branches, tags, labels, milestones, and issues;
- pull-request descriptions, reviews, inline comments, and discussion comments
  in a labeled target issue;
- the author, time, and original GitHub link in copied bodies;
- links between GitHub and target records plus dated run history in SQLite; and
- JSON snapshots, optionally committed to a local Git archive.

`forge-sync sync --dry-run` reports planned changes. It does not change either
forge, Git data, local state, dated run history, or the JSON archive.

## Scale acceptance

The local acceptance benchmark mirrors 50 repositories with 5,000 issues. It
checks an initial pass against 30 minutes and a no-change pass against two
minutes. The test reserves a 20% margin, so its limits are 24 minutes and 96
seconds. It uses local forge endpoints to remove internet delay and prints the
runner’s operating system, architecture, CPU parallelism, and measured times.

## Development

```sh
npm ci
npm test
npm run test:claims
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

The static documentation build is written to `dist/site`. Run it locally with
`npm run dev`.

## Privacy and license

The browser sample stores one `demo:forge-sync:` marker. The configuration
builder has no token field. Read the full [privacy policy](site/privacy/index.html)
and [terms](site/terms/index.html).

forge-sync is released under the [MIT License](LICENSE).
