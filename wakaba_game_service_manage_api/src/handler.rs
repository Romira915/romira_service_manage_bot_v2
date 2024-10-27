use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use schema::sdtd::SdtdRequestJson;

pub(super) async fn sdtd_handler(
    State(state): State<AppState>,
    Json(data): Json<SdtdRequestJson>,
) -> impl IntoResponse {
    match data.command {
        schema::SystemdCommand::Start => {
            state.sdtd_control.start().await.unwrap();
        }
        schema::SystemdCommand::Stop => {
            state.sdtd_control.stop().await.unwrap();
        }
        schema::SystemdCommand::Restart => {
            state.sdtd_control.restart().await.unwrap();
        }
        schema::SystemdCommand::IsActive => {
            let is_active = state.sdtd_control.is_active().await.unwrap();
            if !is_active {
                return StatusCode::SERVICE_UNAVAILABLE;
            }
        }
    }

    StatusCode::OK
}
