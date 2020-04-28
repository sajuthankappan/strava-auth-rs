use super::configuration::Configuration;
use crate::error::StravaAuthError;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::format;
use std::rc::Rc;
use strava_data::models::SummaryAthlete;

pub struct TokenApi {
    pub configuration: Rc<Configuration>,
}

#[derive(Deserialize, Serialize, Debug)]
struct StravaTokenPostBody {
    client_id: String,
    client_secret: String,
    grant_type: String,
    refresh_token: Option<String>,
    code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TokenRecord {
    token_type: String,
    access_token: String,
    expires_at: i64,
    expires_in: i64,
    refresh_token: String,
    athlete: Option<SummaryAthlete>,
}

impl TokenApi {
    pub fn new(configuration: Rc<Configuration>) -> TokenApi {
        TokenApi {
            configuration: configuration,
        }
    }

    pub async fn create_token(
        &self,
        authorization_code: String,
    ) -> Result<TokenRecord, StravaAuthError> {
        let uri = format!("{}/oauth/token", self.configuration.base_path);
        let token_request = &StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("authorization_code"),
            refresh_token: None,
            code: Some(authorization_code),
        };

        let mut response = surf::post(uri)
            .body_json(token_request)
            .map_err(|err| StravaAuthError {
                code: 100,
                message: format!("{}", err),
            })?
            .await
            .map_err(|err| StravaAuthError {
                code: 101,
                message: format!("{}", err),
            })?;

        if response.status() != 200 {
            let body = response
                .body_string()
                .await
                .map_err(|err| StravaAuthError {
                    code: 102,
                    message: format!("{}", err),
                })?;

            return Err(StravaAuthError {
                code: 103,
                message: format!("Error http code: {}. Body: {}", response.status(), body),
            });
        }

        let token = response
            .body_json::<TokenRecord>()
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
        let uri = format!("{}/oauth/token", self.configuration.base_path);
        let post_body = &StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("refresh_token"),
            refresh_token: Some(refresh_token),
            code: None,
        };

        let mut response = surf::post(uri)
            .body_json(post_body)
            .map_err(|err| StravaAuthError {
                code: 100,
                message: format!("{}", err),
            })?
            .await
            .map_err(|err| StravaAuthError {
                code: 101,
                message: format!("{}", err),
            })?;

        if response.status() != 200 {
            let body = response
                .body_string()
                .await
                .map_err(|err| StravaAuthError {
                    code: 102,
                    message: format!("{}", err),
                })?;

            return Err(StravaAuthError {
                code: 103,
                message: format!("Error http code: {}. Body: {}", response.status(), body),
            });
        }

        let token = response
            .body_json::<TokenRecord>()
            .await
            .map_err(|err| StravaAuthError {
                code: 104,
                message: format!("{}", err),
            })?;
        Ok(token)
    }
}
