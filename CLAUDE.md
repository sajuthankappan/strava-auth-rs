# strava-auth-rs

Rust bindings for the Strava OAuth token API, published to crates.io as `strava-auth`. Depends on `strava-data` (sibling repo `../strava-data-rs`) for `SummaryAthlete`.

## Git workflow

- This repo has a single maintainer, so commit and push directly to `master`. Don't create feature branches or PRs.
- Only commit or push when explicitly asked.
- Don't publish to crates.io. The maintainer publishes.

## Build and test

- `cargo build --all-targets` to check everything compiles.
- `tests/integration_tests.rs` calls the live Strava API and needs a `.env` with `CLIENT_ID`, `CLIENT_SECRET`, `AUTHORIZATION_CODE` and `REFRESH_TOKEN`. Without it these tests fail, which is expected.

## Conventions

- Add an entry to `CHANGELOG.md` for user-visible changes.
