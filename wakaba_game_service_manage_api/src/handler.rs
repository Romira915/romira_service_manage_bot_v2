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
    StatusCode::OK
}
