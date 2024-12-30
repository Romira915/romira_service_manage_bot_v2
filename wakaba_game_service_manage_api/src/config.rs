use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen_address: String,
    pub newrelic_license_key: String,
    pub newrelic_service_name: String,
    pub bearer_token: String,
    pub amd3900x_mac_address: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let _ = dotenv::from_filename(".env.wakaba_game_service_manage_api");
    envy::from_env().expect("Failed to load configuration")
});
