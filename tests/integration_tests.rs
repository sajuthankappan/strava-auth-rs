extern crate strava_auth;
use strava_auth::{AuthClient, Configuration};
use async_std::task;

#[test]
fn test_create_access_token() {
    env_logger::init();

    let configuration = Configuration::new(String::from("client_id"), String::from("client_secret"));
    let api_client = AuthClient::new(configuration);
    let acceess_token = task::block_on(api_client.token_api.create_token(String::from("authorization_code")));
    dbg!(acceess_token.unwrap());
}

#[test]
fn test_refresh_access_token() {
    env_logger::init();

    let configuration = Configuration::new(String::from("client_id"), String::from("client_secret"));
    let api_client = AuthClient::new(configuration);
    let acceess_token = task::block_on(api_client.token_api.refresh_token(String::from("refresh_token")));
    dbg!(acceess_token.unwrap());
}