# forge-sync verification 3 handoff — PASS

**Verified candidate:** `fe31571a023330b9cfe53da10c5c9ad7f1a6af10`
**Verified deployment:** `https://forge-sync.sociobot.in/`
**Date:** 2026-08-27

## Final verification verdict

**PASS.** Independent clean-checkout verification passed for the release binary, package, mocked end-to-end mirror workflow, static site/PWA, accessibility, privacy surface, and the deployed URL. The full evidence is in [verification-3.md](verification-3.md).

## How verified

```sh
npm ci
cargo fmt --check
cargo test --locked
cargo clippy --all-targets -- -D warnings
cargo build --release --locked
cargo package --locked --allow-dirty
npm test
npm run build
```

The ready-to-publish package is produced by `cargo package --locked`; registry publishing remains factory-owned. The release binary was independently installed from the packed crate into an empty consumer root and exercised with `--version`, `config example`, `status --json`, and a missing-token failure.

The critical dry-run → real-sync regression was run with local GitHub/Forgejo mocks and actual local Git repositories: the plan did not write durable state/archive data and the following real sync created the target metadata it reported. A separate boundary mock confirmed archived repositories are excluded when configured. The static production build matches the live page byte-for-byte for checked HTML/assets/PWA files; axe reported 0 violations and 0 serious/critical findings.

## Known limitations

- The verifier container has no Docker/Podman, so it could not build the image itself. The Dockerfile's exact Rust build command passed; image runtime assembly still merits a CI/container-host smoke test.
- v1 intentionally represents pull requests as clearly labelled target issues and leaves bidirectional comment relay non-sending behind its experimental flag.

---

# Prior repair record

## What changed

- Fixed the release-blocking dry-run corruption path. `sync --dry-run` now
  opens existing SQLite state read-only (or uses in-memory state on a first
  run), and all state mutation methods are no-ops in that mode. This covers
  mappings, repository status/errors, audit rows, and the GitHub discovery
  cache. It never creates a state directory or database.
- Dry-run archive access is read-only: it creates no archive directory,
  snapshots, manifest, Git repository, or archive commit. Git mirroring and
  forge mutation calls remain skipped.
- Added `tests/dry_run_regression.rs`, an exact mocked sequence of a clean dry
  run followed by a real Forgejo sync. It asserts the dry run leaves no state
  or archive, makes no target writes, and that the real pass creates the
  reported label and issue. Normal state mappings are then recorded as usual.
- Made configuration-originated errors consistently exit with documented code
  `2`, covering unreadable/invalid TOML, validation errors, and missing
  configured token variables. Added process-level CLI regression coverage.
- Restored immediate visible 3px cobalt focus for keyboard users under
  `prefers-reduced-motion`, and disabled animations/transitions in that mode.
  The accessibility browser check traverses focusable controls with Tab at
  mobile width and asserts a visible ring.
- Added `site/public/staticwebapp.config.json`. Vite copies it to
  `dist/site/staticwebapp.config.json`; Azure Static Web Apps will apply
  immutable caching for `/assets/*` and the Permissions-Policy/security
  headers. A site test validates those exact settings.

## Run and verify

```sh
npm ci
npm test
npm run build             # produces dist/site/staticwebapp.config.json
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release --locked
cargo package --locked --allow-dirty
```

The ready-to-publish crate command is `cargo package --locked`; publishing is
owned by the factory.

## Verification completed

- `npm ci`, `npm test`, and `npm run build` passed.
- Rust tests include the dry-run → real migration regression, CLI exit-code
  regression, mocked API tests, and real local Git mirroring.
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed.
- Browser checks against `dist/site`: factory `verify-url.sh` passed with no
  console errors; reduced-motion mobile Tab focus passed; Playwright axe found
  0 violations (0 serious/critical); an offline service-worker reload retained
  the home-page h1.
- The built Static Web Apps artifact was checked for
  `Permissions-Policy: camera=(), microphone=(), geolocation=()` and
  `Cache-Control: public, max-age=31536000, immutable` on `/assets/*`.

## Known boundaries

- The local Vite preview server does not emulate Azure Static Web Apps header
  delivery. The deployable `dist/site/staticwebapp.config.json` artifact is
  present and validated; verify its response headers again after deployment.
- Pull requests remain represented as labeled issues, and bidirectional relay
  remains intentionally non-sending in v1.
