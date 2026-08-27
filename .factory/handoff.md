# forge-sync repair handoff

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
