use super::configuration::Configuration;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub struct TokenApi {
    pub configuration: Rc<Configuration>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
    refresh_token: Option<String>,
    code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub expires_at: u64,
    pub expires_in: u64,
    pub refresh_token: String,
}

impl TokenApi {
    pub fn new(configuration: Rc<Configuration>) -> TokenApi {
        TokenApi {
            configuration: configuration,
        }
    }

    pub async fn create_access_token(
        &self,
        authorization_code: String,
    ) -> Result<Token, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = format!("{}/oauth/token", self.configuration.base_path);
        let token_request = &TokenRequest {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("authorization_code"),
            refresh_token: None,
            code: Some(authorization_code),
        };

        let token = surf::post(uri)
            .body_json(token_request)?
            .recv_json::<Token>()
            .await?;
        Ok(token)
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: String,
    ) -> Result<Token, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = format!("{}/oauth/token", self.configuration.base_path);
        let token_request = &TokenRequest {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("refresh_token"),
            refresh_token: Some(refresh_token),
            code: None,
        };

        let token = surf::post(uri)
            .body_json(token_request)?
            .recv_json::<Token>()
            .await?;
        Ok(token)
    }
}