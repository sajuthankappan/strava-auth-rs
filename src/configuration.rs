pub struct Configuration {
    pub base_path: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Configuration {
    pub fn new(client_id: String, client_secret: String) -> Configuration {
        Configuration {
            base_path: String::from("https://www.strava.com/api/v3"),
            client_id: client_id,
            client_secret: client_secret
        }
    }
}
