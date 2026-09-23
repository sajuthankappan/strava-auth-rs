use std::env;
use strava_auth::{AuthClient, Configuration};

#[tokio::test]
#[ignore = "calls the live Strava API; needs .env"]
async fn test_create_access_token() {
    dotenvy::dotenv().ok();
    let _ = env_logger::try_init();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let authorization_code = env::var("AUTHORIZATION_CODE").unwrap();

    let configuration = Configuration::new(client_id, client_secret);
    let api_client = AuthClient::new(configuration);
    let access_token = api_client
        .token_api
        .create_token(authorization_code)
        .await
        .unwrap();
    dbg!(access_token);
}

#[tokio::test]
#[ignore = "calls the live Strava API; needs .env"]
async fn test_refresh_access_token() {
    dotenvy::dotenv().ok();
    let _ = env_logger::try_init();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let refresh_token = env::var("REFRESH_TOKEN").unwrap();

    let configuration = Configuration::new(client_id, client_secret);
    let api_client = AuthClient::new(configuration);
    let access_token = api_client
        .token_api
        .refresh_token(refresh_token)
        .await
        .unwrap();
    dbg!(access_token);
}
