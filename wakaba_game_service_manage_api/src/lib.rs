use axum::extract::FromRef;
use axum::middleware;
use axum::response::Html;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
pub use systemd::SystemdControl;

pub mod config;
mod custom_middleware;
pub(crate) mod domain;
mod handler;
mod systemd;

pub fn app(app_state: AppState) -> Router {
    let api = Router::new()
        .route("/sdtd", post(handler::sdtd_handler))
        .layer(middleware::from_fn(custom_middleware::require_auth))
        .with_state(app_state);

    Router::new().route("/", get(handler)).nest("/api", api)
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub sdtd_systemd: Arc<dyn SystemdControl>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sdtd_systemd: Arc::new(systemd::Systemd::new("sdtdserver.service")),
        }
    }
}

impl AppState {
    pub fn new(sdtd_control: Arc<dyn SystemdControl>) -> Self {
        Self {
            sdtd_systemd: sdtd_control,
        }
    }
}

#[tracing::instrument]
async fn handler() -> Html<&'static str> {
    log::info!("handling request");
    Html("<h1>Hello, World!</h1>")
}

//noinspection NonAsciiCharacters
#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_handler() {
        let app = app(AppState::default());

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"<h1>Hello, World!</h1>");
    }
}
