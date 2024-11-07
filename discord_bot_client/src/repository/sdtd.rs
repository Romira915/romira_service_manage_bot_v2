use crate::config::CONFIG;
use schema::sdtd::{SdtdRequestJson, SdtdResponseJson};
use schema::{SystemdCommand, SystemdStatus};
use serde_json::Value;

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
        Err(anyhow::anyhow!(
            "Failed to start 7 Days to Die server {}",
            response.text().await?
        ))
    }
}

pub(crate) async fn request_sdtd_status() -> anyhow::Result<SystemdStatus> {
    let request = SdtdRequestJson {
        command: SystemdCommand::IsActive,
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

    let result: SdtdResponseJson = if response.status().is_success() {
        response.json().await?
    } else {
        return Err(anyhow::anyhow!(
            "Failed to check 7 Days to Die server status {}",
            response.text().await?
        ));
    };

    let status = result
        .status
        .ok_or_else(|| anyhow::anyhow!("Status not found"))?;

    Ok(status)
}

pub(crate) async fn request_sdtd_stop() -> anyhow::Result<()> {
    let request = SdtdRequestJson {
        command: SystemdCommand::Stop,
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
        Err(anyhow::anyhow!(
            "Failed to stop 7 Days to Die server {}",
            response.text().await?
        ))
    }
}

pub(crate) async fn request_sdtd_restart() -> anyhow::Result<()> {
    let request = SdtdRequestJson {
        command: SystemdCommand::Restart,
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
        Err(anyhow::anyhow!(
            "Failed to restart 7 Days to Die server {}",
            response.text().await?
        ))
    }
}
