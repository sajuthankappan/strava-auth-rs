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
        TokenApi {
            configuration,
            client: Client::new(),
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
