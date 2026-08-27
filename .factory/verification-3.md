# Independent verification 3 — PASS

**Candidate:** `fe31571a023330b9cfe53da10c5c9ad7f1a6af10`
**Live URL:** `https://forge-sync.sociobot.in/`
**Date:** 2026-08-27
**Scope:** clean-checkout CLI/package verification, mocked GitHub/Forgejo end-to-end flows, and deployed static PWA verification. Product source was not changed.

## Verdict

**PASS.** The candidate fixes the preceding release blocker: a dry run is now fully read-only, and an immediately following real synchronization creates the labels and issue that the plan reported. The CLI satisfies the researched v1 flow: organization discovery, archive/state handling, Git ref mirroring, one-way issue/PR metadata transfer, and a portable archive. The deployed documentation PWA is byte-identical to the candidate production output and serves the intended security and cache headers.

## Clean checkout, build, package

- Created a fresh `git clone --no-local` at exactly the candidate SHA; its worktree was clean before verification.
- `npm ci` passed with **0 vulnerabilities** reported.
- `cargo fmt --check`, `cargo test --locked`, `cargo clippy --all-targets -- -D warnings`, `cargo build --release --locked`, `cargo package --locked --allow-dirty`, `npm test`, and `npm run build` all passed.
- Rust tests: 4 unit tests, 2 API/auth contract tests, CLI exit-code regression, dry-run-to-real synchronization regression, and a real local bare-Git branch/tag mirror test all passed. Site tests: 4/4 passed.
- Production release binary: **6,368,376 bytes**. Packaged crate: **138,878 bytes** (`target/package/forge-sync-0.1.0.crate`); package verification build passed.
- Extracted that `.crate` into an isolated consumer directory and ran `cargo install --path ... --locked`. The installed binary reported `forge-sync 0.1.0`, printed `config example`, returned empty JSON status on first run, and returned documented exit **2** for a missing configured token.
- No separate JavaScript typecheck/lint script is declared by this repository; the available Rust lint gate passed.

## Product behavior exercised

- **Normal/recovery:** the checked-in regression uses local GitHub/Forgejo HTTP mocks and real local Git repositories. A dry plan reports one repository/issue without target writes, state, or archive creation; the following real pass creates the repository metadata, label, and issue and records mappings. This is the previously failing workflow.
- **Git refs:** `tests/git_mirror.rs` created local bare source/target repositories, then verified the target has both the mirrored branch and tag.
- **Boundary:** against an independent local HTTP mock returning one normal and one archived repository, `sync --dry-run --json` with `include_archived=false` returned `{"discovered":1,"synchronized":1,"failed":0,"issues":0,"comments":0,"dry_run":true,...}`. The mock would fail if the archived repository were touched; neither state nor archive directory existed afterwards.
- **Malformed/input recovery:** invalid TOML/configuration returned exit **2** and `configuration error`; a clean valid `status --json` returned zero repositories/mappings/audit events; missing configured environment variables returned exit **2** without a prompt. `--help`, `--version`, and `config example` were exercised.
- Source review and the tests confirm source tokens are read from environment variables, dry-run mappings/audit/cache writes are suppressed, and errors redact source/target token values before durable repository-error/audit storage.

## Live deployment, privacy, security, and usability

- Rebuilt `dist/site` and compared it with the live deployment. `index.html`, `main-CNT8cG3r.js`, `style-Cp36xAMf.css`, `ceramic-mirror-D653xRQn.webp`, `sw.js`, `/privacy/`, and `/terms/` were byte-for-byte equal. Example SHA-256 values: index `68db99c5ccfd5e9d0db09162c159c08b9d65b8272afe22f2ac3f278fd909b16f`; main JS `d2e84cfe5a3173f61f416b67c1eb0c0c6b46c1f6aa56feec6cc269cbabdfca14`.
- Desktop and 390×844 mobile smoke tests generated a GitLab target configuration from valid inputs, gave clear malformed-org feedback, had one `h1` and one `main`, no horizontal overflow at 390px, and produced no console or page errors. Normal page load made no third-party request.
- Keyboard-only checks at desktop and 390px under reduced motion found the designed visible `3px` cobalt focus ring. `node scripts/a11y.mjs https://forge-sync.sociobot.in` found **0 axe violations**, including **0 serious/critical**, across home, privacy, and terms.
- The only runtime cross-origin request in source is the disclosed Sociobot license verification request after a license token is supplied. A browser test confirmed it stores the token locally, strips it from the URL, and requests only `https://api.sociobot.in/api/v1/products/forge-sync/verify?...`. Privacy and terms accurately disclose this; no analytics/telemetry or CDN font/script request was observed.
- Live responses have HSTS, CSP, `X-Content-Type-Options: nosniff`, `Referrer-Policy`, `X-Frame-Options: DENY`, and `Permissions-Policy: camera=(), microphone=(), geolocation=()`. HTML and `sw.js` use revalidation; hashed JS, CSS, font, and WebP assets use `public, max-age=31536000, immutable`.
- Built budgets: JS **5.51 kB**, CSS **12.03 kB**, self-hosted fonts **82.33 kB** combined, and hero WebP **70.11 kB**, each within the stated budgets.
- PWA: the live service worker installed as `forge-sync-v1` and an offline reload retained the home `h1`. A local production-build update simulation (without modifying source) advanced the service-worker cache from `forge-sync-v1` to `forge-sync-v2` after the old client closed.

## Defects by severity

None identified in the tested scope.

## Verification limitation

No Docker or Podman executable is available in this verifier container, so the container image itself was not built. Its exact build-stage command (`cargo build --release --locked`, using Rust 1.98) passed locally, and the final image recipe only adds CA certificates and Git around that binary. This is an environment limitation, not a product failure.
