use super::configuration::Configuration;
use crate::error::StravaAuthError;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::format;
use std::sync::Arc;
use strava_data::models::SummaryAthlete;

pub struct TokenApi {
    pub configuration: Arc<Configuration>,
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
            configuration: configuration,
        }
    }

    pub async fn create_token(
        &self,
        authorization_code: String,
    ) -> Result<TokenRecord, StravaAuthError> {
        let url = format!("{}/oauth/token", self.configuration.base_path);
        let token_request = &StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("authorization_code"),
            refresh_token: None,
            code: Some(authorization_code),
        };

        let res = Client::new()
            .post(url.as_str())
            .json(token_request)
            .send()
            .await
            .map_err(|err| StravaAuthError {
                code: 100,
                message: format!("{}", err),
            })?;

        if res.status().clone() != StatusCode::OK {
            let status = res.status().clone();
            let body = res.text().await.map_err(|err| StravaAuthError {
                code: 102,
                message: format!("{}", err),
            })?;

            return Err(StravaAuthError {
                code: 103,
                message: format!("Error http code: {}. Body: {}", status, body),
            });
        }

        let token = res
            .json::<TokenRecord>()
            .await
            .map_err(|err| StravaAuthError {
                code: 104,
                message: format!("{}", err),
            })?;
        Ok(token)
    }

    pub async fn refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<TokenRecord, StravaAuthError> {
        let url = format!("{}/oauth/token", self.configuration.base_path);
        let post_body = &StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("refresh_token"),
            refresh_token: Some(refresh_token),
            code: None,
        };

        let res = Client::new()
            .post(url.as_str())
            .json(post_body)
            .send()
            .await
            .map_err(|err| StravaAuthError {
                code: 100,
                message: format!("{}", err),
            })?;

        if res.status().clone() != StatusCode::OK {
            let status = res.status().clone();
            let body = res.text().await.map_err(|err| StravaAuthError {
                code: 102,
                message: format!("{}", err),
            })?;

            return Err(StravaAuthError {
                code: 103,
                message: format!("Error http code: {}. Body: {}", status, body),
            });
        }

        let token = res
            .json::<TokenRecord>()
            .await
            .map_err(|err| StravaAuthError {
                code: 104,
                message: format!("{}", err),
            })?;
        Ok(token)
    }
}
