use super::configuration::Configuration;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
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
    ) -> Result<TokenRecord, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = format!("{}/oauth/token", self.configuration.base_path);
        let token_request = &StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("authorization_code"),
            refresh_token: None,
            code: Some(authorization_code),
        };

        let token = surf::post(uri)
            .body_json(token_request)?
            .recv_json::<TokenRecord>()
            .await?;
        Ok(token)
    }

    pub async fn refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<TokenRecord, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let uri = format!("{}/oauth/token", self.configuration.base_path);
        let post_body = &StravaTokenPostBody {
            client_id: self.configuration.client_id.to_owned(),
            client_secret: self.configuration.client_secret.to_owned(),
            grant_type: String::from("refresh_token"),
            refresh_token: Some(refresh_token),
            code: None,
        };

        let token = surf::post(uri)
            .body_json(post_body)?
            .recv_json::<TokenRecord>()
            .await?;
        Ok(token)
    }
}
