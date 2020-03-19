use super::configuration::Configuration;
use super::TokenApi;
use std::rc::Rc;

pub struct AuthClient {
    pub configuration: Rc<Configuration>,
    pub token_api: Box<TokenApi>,
}

impl AuthClient {
    pub fn new(configuration: Configuration) -> AuthClient {
        let rc = Rc::new(configuration);

        AuthClient {
            configuration: rc.clone(),
            token_api: Box::new(TokenApi::new(rc.clone())),
        }
    }
}
