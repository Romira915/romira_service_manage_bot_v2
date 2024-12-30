use crate::config::CONFIG;
use crate::SystemdControl;
use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use schema::sdtd::{SdtdRequestJson, SdtdResponseJson};
use schema::wol::{WolRequestJson, WolResponseJson};
use std::str::FromStr;
use std::sync::Arc;
use wol::MacAddr;

pub(super) struct SdtdError(anyhow::Error);

impl IntoResponse for SdtdError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SdtdResponseJson::from_result(&format!("error: {}", self.0))),
        )
            .into_response()
    }
}

impl<E> From<E> for SdtdError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

pub(super) async fn sdtd_handler(
    State(sdtd_systemd): State<Arc<dyn SystemdControl>>,
    Json(data): Json<SdtdRequestJson>,
) -> Result<impl IntoResponse, SdtdError> {
    let response = match data.command {
        schema::SystemdCommand::Start => {
            sdtd_systemd.start().await?;
            SdtdResponseJson::from_result("success")
        }
        schema::SystemdCommand::Stop => {
            sdtd_systemd.stop().await?;
            SdtdResponseJson::from_result("success")
        }
        schema::SystemdCommand::Restart => {
            sdtd_systemd.restart().await?;
            SdtdResponseJson::from_result("success")
        }
        schema::SystemdCommand::IsActive => {
            let is_active = sdtd_systemd.is_active().await?;

            if is_active {
                SdtdResponseJson::new("success", schema::SystemdStatus::Active)
            } else {
                SdtdResponseJson::new("success", schema::SystemdStatus::Inactive)
            }
        }
    };

    Ok(Json(response))
}

pub(super) struct WolError(anyhow::Error);

impl IntoResponse for WolError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WolResponseJson {
                result: format!("error: {}", self.0),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for WolError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

pub(super) async fn wol_handler(
    Json(data): Json<WolRequestJson>,
) -> Result<impl IntoResponse, WolError> {
    let target_mac = match data.target {
        schema::wol::WolTarget::Amd3900X => &CONFIG.amd3900x_mac_address,
    };

    wol::send_wol(
        MacAddr::from_str(target_mac).expect("Failed to parse mac address"),
        Some(CONFIG.wol_broadcast_address),
        None,
    )
    .context("Failed to send WOL packet")?;

    Ok(Json(WolResponseJson {
        result: "success".to_string(),
    }))
}
