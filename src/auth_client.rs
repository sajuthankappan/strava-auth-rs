use super::configuration::Configuration;
use super::TokenApi;
use std::sync::Arc;

pub struct AuthClient {
    pub configuration: Arc<Configuration>,
    pub token_api: Box<TokenApi>,
}

impl AuthClient {
    pub fn new(configuration: Configuration) -> AuthClient {
        let arc = Arc::new(configuration);

        AuthClient {
            configuration: Arc::clone(&arc),
            token_api: Box::new(TokenApi::new(arc)),
        }
    }
}
