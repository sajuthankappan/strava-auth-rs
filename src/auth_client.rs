use super::TokenApi;
use super::configuration::Configuration;
use reqwest::Client;
use std::sync::Arc;

pub struct AuthClient {
    pub configuration: Arc<Configuration>,
    pub token_api: TokenApi,
}

impl AuthClient {
    pub fn new(configuration: Configuration) -> AuthClient {
        AuthClient::with_client(configuration, Client::new())
    }

    /// Creates an `AuthClient` that sends requests through `client`, e.g. one
    /// configured with timeouts or a proxy.
    pub fn with_client(configuration: Configuration, client: Client) -> AuthClient {
        let arc = Arc::new(configuration);

        AuthClient {
            configuration: Arc::clone(&arc),
            token_api: TokenApi::with_client(arc, client),
        }
    }
}
