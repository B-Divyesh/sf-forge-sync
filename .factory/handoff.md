# forge-sync v0.1.0 handoff

## What shipped

- A Rust single-binary CLI with `doctor`, `sync`, `daemon`, `status`, `config example`, `--json`, stable exit codes, dry-run behavior, graceful daemon shutdown, and retrying daemon passes.
- GitHub organization discovery with pagination and ETag caching; inclusion/exclusion filters; automatic creation of repositories on Forgejo, Codeberg, and GitLab.
- Authenticated, credential-safe Git mirror fetch/push for branches, tags, and other refs. Tokens are supplied through a generated `GIT_ASKPASS` helper and are not embedded in URLs or command arguments.
- One-way synchronization of labels, milestones, issues, comments, PR descriptions, reviews, and inline review comments. Authorship, timestamps, original links, and review file/line anchors are added to target bodies.
- Idempotent SQLite mappings, append-only audit records, partial-repository failure handling, change fingerprints, unchanged-item skipping, and a git-tracked forge-neutral JSON archive.
- A release Dockerfile, example config, MIT license, changelog, security policy, integration tests, and CI workflow. `cargo package` produces a publishable crate.
- A responsive static landing/docs site in the product-specific “glacial minimal ceramics” system, including an original generated WebP hero, live local-only config builder, offline shell, privacy/terms pages, and keyboard/mobile states.
- A $39 one-time Migration Kit flow through the Sociobot billing contract: hosted buy link, return-token capture, local storage, daily verification cache, optimistic offline unlock, restore field, revocation state, and downloadable runbook. The open-source mirroring/export core remains ungated.

## Run and build

```sh
cargo install --path .
forge-sync config example > forge-sync.toml
forge-sync doctor --config forge-sync.toml
forge-sync sync --config forge-sync.toml

npm ci
npm test
npm run build:site
```

The factory deploy target is exactly `dist/site` (with `index.html` at that root). The ready-to-publish package command is `cargo package --locked`; the verified artifact was `target/package/forge-sync-0.1.0.crate` (129 KB). The optimized release binary was 6.1 MB.

## Verification performed

- `cargo fmt --check`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm test`: passed 3 site tests, 4 Rust unit tests, 2 mock-API contract tests, 1 real local Git mirror integration test, and doc tests.
- `cargo build --release --locked`: passed.
- `cargo package --locked --allow-dirty`: packaged and verified from the crate contents.
- `npm run build:site`: passed with Vite 7.3.6; `npm audit --audit-level=high`: 0 vulnerabilities.
- Factory `verify-url.sh`: HTTP 200, title/lang/main present, exactly one h1, no missing alt text, no unlabeled buttons, and no browser console/page errors at desktop or 390×844 mobile.
- Playwright axe across `/`, `/privacy/`, and `/terms/`: 0 violations (including 0 serious/critical).
- Browser interaction smoke: license return token stored and stripped from the URL; invalid config error announced; valid GitLab config regenerated.
- Lighthouse mobile: performance 99, accessibility 100, best practices 100, SEO 100; FCP 1.4 s, LCP 2.0 s, TBT 0 ms, CLS 0.023, total transfer 177 KiB.
- Static budgets: JS 5.51 KB, CSS 11.75 KB, self-hosted fonts 82.33 KB total, hero WebP 70.11 KB. Hashed assets receive immutable-cache headers.

## Known gaps and honest boundaries

- Pull-request history is represented as a labeled target issue rather than a native target PR. This is intentional so merged, deleted-branch, and cross-fork discussions remain importable.
- Bidirectional comment/review relay remains non-sending behind the experimental config flag. CI translation, wikis/projects, reactions, and GitLab-as-source are outside v1.
- Source deletions are not propagated destructively; removed source comments may remain in the safety archive/target. Linked GitHub attachments are referenced, not downloaded.
- Live credentials for hosted GitHub/Forgejo/GitLab were not available in the worker. HTTP authentication/decoding was verified against local mock APIs and Git transport against real local repositories; a staging organization should be the next acceptance pass.
- The 50-repository/5,000-issue success target was not load-tested here. The implementation uses repository-wide comment pagination and unchanged-object skipping to stay within API limits, but real-world timing depends heavily on Git object volume and forge latency.

## Next steps

1. Run `doctor`, dry-run, then a full pass against a private staging organization on each target family.
2. Record 50-repository initial/incremental timings and tune request concurrency only if real measurements require it.
3. Add reconciliation for content deleted at the source, with an explicit non-destructive policy.
4. Validate a safe identity/anchor model before enabling any bidirectional relay.

## Asset provenance

The final hero is `site/ceramic-mirror.webp`; its full prompt and `factory-image` deployment metadata are in `site/public/ceramic-mirror.prompt.json`. It was generated through `/opt/fleet/lib/gen-image.sh`, inspected, and converted to a 69 KB WebP. No input asset, trademark, or third-party image was used.
