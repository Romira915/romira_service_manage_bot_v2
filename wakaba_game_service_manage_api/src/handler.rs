use crate::SystemdControl;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use schema::sdtd::{SdtdRequestJson, SdtdResponseJson};
use std::sync::Arc;

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
