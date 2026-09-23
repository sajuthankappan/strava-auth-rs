# strava-auth-rs

Rust bindings for the Strava OAuth token API, published to crates.io as `strava-auth`. Depends on `strava-data` (sibling repo `../strava-data-rs`) for `SummaryAthlete`.

## Git workflow

- This repo has a single maintainer, so work happens directly on `master`. Don't create feature branches or PRs.
- Never commit or push unless explicitly asked for that specific change. Approval to "continue" or "do the next step" is not approval to commit. Leave changes uncommitted in the working tree for review.
- Don't publish to crates.io. The maintainer publishes.

## Build and test

- `cargo build --all-targets` to check everything compiles.
- `tests/integration_tests.rs` calls the live Strava API and needs a `.env` with `CLIENT_ID`, `CLIENT_SECRET`, `AUTHORIZATION_CODE` and `REFRESH_TOKEN`. They are `#[ignore]`d so `cargo test` and CI skip them; run them with `cargo test -- --ignored`.
- CI (`.github/workflows/ci.yml`) runs `cargo fmt --check`, `cargo clippy --all-targets` and `cargo test` on stable, beta and the MSRV (1.87), with warnings denied. Keep `rust-version` in `Cargo.toml` and the MSRV in the CI matrix in sync.

## Conventions

- Add an entry to `CHANGELOG.md` for user-visible changes.
