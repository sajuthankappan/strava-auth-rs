use super::configuration::Configuration;
use crate::error::StravaAuthError;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use strava_data::models::SummaryAthlete;

pub struct TokenApi {
    pub configuration: Arc<Configuration>,
    client: Client,
}

#[derive(Deserialize, Serialize, Debug)]
struct StravaTokenPostBody {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub refresh_token: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenRecord {
    pub token_type: String,
    pub access_token: String,
    pub expires_at: i64,
    pub expires_in: i64,
    pub refresh_token: String,
    pub athlete: Option<SummaryAthlete>,
}

impl TokenApi {
    pub fn new(configuration: Arc<Configuration>) -> TokenApi {
        TokenApi::with_client(configuration, Client::new())
    }

    /// Creates a `TokenApi` that sends requests through `client`, e.g. one
    /// configured with timeouts or a proxy.
    pub fn with_client(configuration: Arc<Configuration>, client: Client) -> TokenApi {
        TokenApi {
            configuration,
            client,
        }
    }

    pub async fn create_token(
        &self,
        authorization_code: String,
    ) -> Result<TokenRecord, StravaAuthError> {
        self.post_token(StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("authorization_code"),
            refresh_token: None,
            code: Some(authorization_code),
        })
        .await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<TokenRecord, StravaAuthError> {
        self.post_token(StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("refresh_token"),
            refresh_token: Some(refresh_token),
            code: None,
        })
        .await
    }

    async fn post_token(
        &self,
        post_body: StravaTokenPostBody,
    ) -> Result<TokenRecord, StravaAuthError> {
        let url = format!("{}/oauth/token", self.configuration.base_path);

        let res = self
            .client
            .post(url.as_str())
            .json(&post_body)
            .send()
            .await
            .map_err(|err| StravaAuthError::Request(err.into()))?;

        let status = res.status();
        if status != StatusCode::OK {
            let body = res
                .text()
                .await
                .map_err(|err| StravaAuthError::Request(err.into()))?;

            return Err(StravaAuthError::Status {
                status: status.as_u16(),
                body,
            });
        }

        res.json::<TokenRecord>()
            .await
            .map_err(|err| StravaAuthError::Decode(err.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn with_client_sends_requests_through_given_client() {
        let mut configuration = Configuration::new(String::from("id"), String::from("secret"));
        // Nothing listens on port 1, so the request fails before reaching Strava.
        configuration.base_path = String::from("http://127.0.0.1:1");
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();
        let token_api = TokenApi::with_client(Arc::new(configuration), client);

        let err = token_api
            .refresh_token(String::from("refresh"))
            .await
            .unwrap_err();

        assert!(matches!(err, StravaAuthError::Request(_)), "{err:?}");
    }
}
