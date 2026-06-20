use serde::Deserialize;
use dotenvy::dotenv;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(e) => {
                panic!("Missing or invalid environment variables: {:#?}", e);
            }
        }
    }

    // pub fn table() -> String {
    //     return "test".to_string();
    // }
}