use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub wakaba_game_api_bearer_token: String,
    pub newrelic_license_key: String,
    pub newrelic_service_name: String,
    pub wakaba_game_api_base_url: String,
    pub discord_romira_user_id: u64
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let _ = dotenv::from_filename(".env.discord_bot_client");
    envy::from_env().expect("Failed to load configuration")
});
