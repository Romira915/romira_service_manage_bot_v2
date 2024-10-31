use crate::config::CONFIG;
use schema::sdtd::SdtdRequestJson;
use schema::SystemdCommand;

pub(crate) async fn request_sdtd_start() -> anyhow::Result<()> {
    let request = SdtdRequestJson {
        command: SystemdCommand::Start,
    };
    let response = reqwest::Client::new()
        .post(format!("{}/api/sdtd", CONFIG.wakaba_game_api_base_url))
        .header(
            "Authorization",
            format!("Bearer {}", CONFIG.wakaba_game_api_bearer_token),
        )
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to start 7 Days to Die server {}", response.text().await?))
    }
}
