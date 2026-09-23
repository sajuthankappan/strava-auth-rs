# strava-auth &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs] [![CI Status]][actions]
[Latest Version]: https://img.shields.io/crates/v/strava-auth.svg
[crates.io]: https://crates.io/crates/strava-auth
[Docs]: https://docs.rs/strava-auth/badge.svg
[docs.rs]: https://docs.rs/strava-auth
[CI Status]: https://github.com/sajuthankappan/strava-auth-rs/actions/workflows/ci.yml/badge.svg?branch=master
[actions]: https://github.com/sajuthankappan/strava-auth-rs/actions/workflows/ci.yml

**Strava OAuth token API client for Rust**

## Features

- Exchange an authorization code for access and refresh tokens
- Refresh an expired access token
- Bring your own `reqwest::Client` (timeouts, proxies, connection pooling)

Tokens are returned as a `TokenRecord`, which includes the authenticated athlete as a [`strava-data`](https://crates.io/crates/strava-data) `SummaryAthlete` when Strava returns it.

Minimum supported Rust version: 1.87

## Usage example

Exchange the authorization code from Strava's OAuth redirect for tokens

```rust
use strava_auth::{AuthClient, Configuration};

let configuration = Configuration::new(client_id, client_secret);
let client = AuthClient::new(configuration);
let token = client.token_api.create_token(authorization_code).await?;
println!("access token expires at {}", token.expires_at);
```

Refresh an expired access token

```rust
let token = client.token_api.refresh_token(refresh_token).await?;
// Strava may rotate the refresh token, so store the new one
save_refresh_token(&token.refresh_token);
```

Use a custom HTTP client

```rust
use std::time::Duration;
use strava_auth::reqwest::Client;

let http = Client::builder().timeout(Duration::from_secs(10)).build()?;
let client = AuthClient::with_client(configuration, http);
```

## Error handling

Errors are returned as `StravaAuthError`:

```rust
use strava_auth::error::StravaAuthError;

match client.token_api.refresh_token(refresh_token).await {
    Ok(token) => println!("new access token expires at {}", token.expires_at),
    // Strava rejected the request, e.g. an invalid or revoked refresh token
    Err(StravaAuthError::Status { status, body }) => {
        eprintln!("Strava returned HTTP {status}: {body}");
    }
    // Network / TLS errors, or a response that is not a valid token
    Err(e) => eprintln!("token request failed: {e}"),
}
```

## Running the integration tests

The integration tests call the live Strava API and are ignored by default. Create a `.env` with `CLIENT_ID`, `CLIENT_SECRET`, `AUTHORIZATION_CODE` and `REFRESH_TOKEN`, then run:

```sh
cargo test -- --ignored
```

## Maintainer

Maintained by [Saju Thankappan](https://github.com/sajuthankappan), creator of [Smito One](https://smito.in), which uses this crate for its Strava integration.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
