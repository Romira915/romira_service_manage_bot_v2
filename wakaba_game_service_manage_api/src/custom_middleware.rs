use crate::config::CONFIG;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;

pub(super) async fn require_auth(request: Request, next: Next) -> Result<impl IntoResponse, StatusCode> {
    let auth_header = request.headers().get("Authorization");

    match auth_header {
        Some(value) if value == &format!("Bearer {}", CONFIG.bearer_token) => {
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
