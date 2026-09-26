pub struct Configuration {
    pub base_path: String,
    /// Base URL of the OAuth endpoints that live outside `/api/v3`, such as `/oauth/revoke`.
    pub oauth_base_path: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Configuration {
    pub fn new(client_id: String, client_secret: String) -> Configuration {
        Configuration {
            base_path: String::from("https://www.strava.com/api/v3"),
            oauth_base_path: String::from("https://www.strava.com/oauth"),
            client_id,
            client_secret,
        }
    }
}
