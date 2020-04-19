extern crate strava_auth;
use strava_auth::{AuthClient, Configuration};
use async_std::task;
use std::env;

#[test]
fn test_create_access_token() {
    env_logger::init();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let authorization_code = env::var("AUTHORIZATION_CODE").unwrap();
    
    let configuration = Configuration::new(client_id, client_secret);
    let api_client = AuthClient::new(configuration);
    let acceess_token = task::block_on(api_client.token_api.create_token(authorization_code)).unwrap();
    dbg!(acceess_token);
}

#[test]
fn test_refresh_access_token() {
    env_logger::init();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let refresh_token = env::var("REFRESH_TOKEN").unwrap();
    
    let configuration = Configuration::new(client_id, client_secret);
    let api_client = AuthClient::new(configuration);
    let acceess_token = task::block_on(api_client.token_api.refresh_token(refresh_token)).unwrap();
    dbg!(acceess_token);
}