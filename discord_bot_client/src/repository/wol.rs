use crate::config::CONFIG;
use schema::wol::{WolRequestJson, WolTarget};

pub(crate) async fn request_wol(target: WolTarget) -> anyhow::Result<()> {
    let request = WolRequestJson { target };

    let response = reqwest::Client::new()
        .post(format!("{}/api/wol", CONFIG.wakaba_game_api_base_url))
        .bearer_auth(&CONFIG.wakaba_game_api_bearer_token)
        .json(&request)
        .send()
        .await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to send Wake-on-LAN magic packet {}",
            response.text().await?
        ))
    }
}
