# forge-sync

`forge-sync` continuously mirrors every repository in a GitHub organization to Forgejo, Codeberg, or GitLab. It discovers new repositories, mirrors every Git ref, carries issues and pull-request discussions into the target with source attribution, and writes the same metadata to a portable JSON archive.

It is for maintainers and small organizations that want an independently usable copy of their work before, during, or after a forge migration. The core is open source, local-first, and has no telemetry.

## Install

Prebuilt binaries are attached to GitHub releases. From source (Rust 1.85+):

```sh
cargo install --path .
forge-sync --help
```

Or build and run the container:

```sh
docker build -t forge-sync .
docker run --rm -v "$PWD:/data" --env-file .env forge-sync sync --config /data/forge-sync.toml
```

## Quick start

Create `forge-sync.toml` (tokens may use environment-variable names; never put token values in the file):

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

Validate access without changing either forge, then perform one complete pass:

```sh
export GITHUB_TOKEN=github_pat_…
export FORGE_TOKEN=…
forge-sync doctor --config forge-sync.toml
forge-sync sync --config forge-sync.toml
```

Run continuously (SIGINT/SIGTERM exits cleanly):

```sh
forge-sync daemon --config forge-sync.toml
```

Every command supports scripting output:

```sh
forge-sync status --config forge-sync.toml --json
forge-sync sync --config forge-sync.toml --json
```

Exit codes are `0` success, `2` configuration/usage (including unreadable or invalid TOML and missing configured token variables), `3` authentication, `4` API/rate-limit, `5` git transport, and `6` partial synchronization. Commands never prompt.

## What gets synchronized

- organization repository discovery, including repositories created after the daemon starts;
- branches, tags, and other Git refs via a true mirror push;
- labels and milestones;
- issues, state, body, labels, milestone, and comments;
- pull request description, review summaries, inline review comments, and conversation comments, represented as a clearly labeled target issue so closed and cross-fork PR history remains portable;
- author, timestamps, and canonical GitHub links in every mirrored body;
- stable source→target ID mappings and append-only audit events in SQLite;
- forge-neutral JSON snapshots, committed to a local Git archive when enabled.

Updates are idempotent. forge-sync writes a hidden source marker into target content and retains its mapping database, so restarting or retrying does not duplicate objects. GitHub conditional requests and pagination reduce rate-limit use. A failed repository does not prevent the remainder of the organization from being processed; the run exits `6` and records the failure.

## Configuration

Run `forge-sync config example` for the complete annotated file. Important optional controls:

```toml
[sync]
repos = ["api", "docs"]       # empty means all repositories
exclude = ["scratch-*"]       # simple * and ? glob patterns
dry_run = false
experimental_comment_relay = false
```

`forge-sync sync --dry-run` reads the source and target to produce the same
report as a pass, but never changes either forge, Git refs, SQLite state,
discovery cache/audit data, or the JSON archive. It is safe to run before the
first real migration; the next real run creates every missing object it found.

Bidirectional comment relay is intentionally disabled in v1; setting the experimental flag records a warning but never sends a target comment to GitHub. This avoids silently misattributing authors or drifting inline anchors.

## Data and security

Tokens are read only from environment variables. They are never written to config, logs, SQLite, or the JSON archive. Use a GitHub fine-grained token with organization repository read access and a target token allowed to create repositories/issues and push Git. Private target repositories are the default. `forge-sync doctor` checks identities and permissions before a first run.

The archive contains the organization content you elect to mirror. Protect it like the source organization. See [SECURITY.md](SECURITY.md) for safe reporting and token guidance.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
cargo package --allow-dirty

npm ci
npm test
npm run build:site       # static site -> dist/site
```

The repository contains integration tests backed by a local mock HTTP forge and temporary Git repositories; tests do not require network credentials.

## Deployment

The CLI is a single release binary or container. Run exactly one daemon per state directory. The documentation site is static and the factory deploys `dist/site`; it does not receive tokens or mirror data.

## License

MIT. See [LICENSE](LICENSE).
