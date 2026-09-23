## Unreleased

* Breaking: Replace `StravaAuthError { code, message }` with a `thiserror` enum (`Request`, `Status { status, body }`, `Decode`). Underlying errors are available via `Error::source()`
* Breaking: `AuthClient::token_api` is now `TokenApi` instead of `Box<TokenApi>`
* Breaking: `TokenApi` has a private field, so it can no longer be built with a struct literal; use `TokenApi::new`
* Reuse a single HTTP client across requests instead of creating one per request
* Add `AuthClient::with_client` and `TokenApi::with_client` to supply a custom `reqwest::Client`, and re-export `reqwest`
* Upgrade to Rust 2024 edition (minimum Rust 1.87)
* Remove unused `log` dependency
* Upgrade reqwest to 0.13, which uses rustls instead of native-tls (OpenSSL) for TLS
* Remove unused `serde_json` dependency

## 0.6.0

* Remove dependency on getset

## 0.5.0

* Upgrade reqwest to 0.11.4 & strava-data to 0.5.0

## 0.4.0

* Replace surf with reqwest & update strava-data to 0.4.0

## 0.3.2

* Change configuration from Rc to Arc

## 0.3.1

* Update strava-data version

## 0.3.0

* Add basic custom error type

## 0.2.3

* Fix: Make TokenRecord as pub

## 0.2.2

* Fix: Convert u64 to i64, as mongodb and wasm doesn't support unsigned data types (yet)

## 0.2.1

* Add SummaryAthlete model for TokenRecord

## 0.1.0

* First preview release